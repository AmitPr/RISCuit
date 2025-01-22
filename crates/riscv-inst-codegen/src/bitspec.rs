use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    bracketed,
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    token, LitInt,
};

/// Parses M:L or M
#[derive(Debug, Clone)]
struct BitRange {
    msb: usize,
    lsb: usize,
}

impl Parse for BitRange {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let msb: LitInt = input.parse()?;
        let lsb = if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;
            let lsb: LitInt = input.parse()?;
            lsb.base10_parse::<usize>()?
        } else {
            msb.base10_parse::<usize>()?
        };

        Ok(BitRange {
            msb: msb.base10_parse()?,
            lsb,
        })
    }
}

impl BitRange {
    fn width(&self) -> usize {
        self.msb - self.lsb + 1
    }
}

struct Bitspec {
    gather: BitRange,
    scatter: Vec<BitRange>,
}

impl Parse for Bitspec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let gather = input.parse::<BitRange>()?;
        let scatter = if input.peek(token::Bracket) {
            let content;
            bracketed!(content in input);
            let scatter = Punctuated::<BitRange, token::Or>::parse_separated_nonempty(&content)?;
            scatter.into_iter().collect::<Vec<_>>()
        } else {
            vec![BitRange {
                msb: gather.msb - gather.lsb,
                lsb: 0,
            }]
        };

        Ok(Bitspec { gather, scatter })
    }
}

fn mask(width: usize, shift: usize) -> LitInt {
    let mask: u64 = (1 << width) - 1;
    let mask: u64 = mask << shift;
    let mask = format!("{:#b}", mask);

    LitInt::new(&mask, proc_macro2::Span::call_site())
}

impl Bitspec {
    fn accessor(&self, src: TokenStream) -> TokenStream {
        let BitRange { mut msb, .. } = &self.gather;

        let mut segments = Vec::with_capacity(self.scatter.len());
        for scatter in &self.scatter {
            // take scatter.width() bits from gather, and place them at scatter.lsb
            let src_msb = msb;
            let dst_msb = scatter.msb;
            let shift = match src_msb.cmp(&dst_msb) {
                std::cmp::Ordering::Less => {
                    // shift left
                    let shamt = LitInt::new(
                        &format!("{}", dst_msb - src_msb),
                        proc_macro2::Span::call_site(),
                    );
                    quote! {
                        (#src << #shamt)
                    }
                }
                std::cmp::Ordering::Equal => quote! { #src },
                std::cmp::Ordering::Greater => {
                    // shift right
                    let shamt = LitInt::new(
                        &format!("{}", src_msb - dst_msb),
                        proc_macro2::Span::call_site(),
                    );
                    quote! {
                        (#src >> #shamt)
                    }
                }
            };
            let dst_lsb = scatter.lsb;
            let src_mask = mask(scatter.width(), dst_lsb);

            msb -= scatter.width();

            segments.push(quote! {
                (#shift & #src_mask)
            });
        }

        quote! {
            (#(#segments)|*)
        }
    }
}

pub fn serialize_bitspecs(src: TokenStream, out_ty: TokenStream, bitspecs: &str) -> TokenStream {
    let parser = Punctuated::<Bitspec, token::Comma>::parse_terminated;
    let bitspecs = parser
        .parse_str(bitspecs)
        .unwrap_or_else(|e| panic!("Failed to parse bitspec {}: {}", bitspecs, e))
        .into_iter()
        .collect::<Vec<_>>();

    let accessors = bitspecs
        .iter()
        .map(|bitspec| bitspec.accessor(src.clone()))
        .collect::<Vec<_>>();

    let hsb = bitspecs
        .iter()
        .map(|b| b.scatter.iter().map(|r| r.msb).max().unwrap_or_default())
        .max();

    let accessor = quote! {
        #(#accessors)|*
    };

    match out_ty.to_string().as_str() {
        "i32" => {
            let shamt = 31 - hsb.unwrap_or(0);
            let shamt = LitInt::new(&format!("{}", shamt), proc_macro2::Span::call_site());
            quote! {
                (((#accessor) << #shamt) as i32) >> #shamt
            }
        }
        "i64" => {
            let shamt = 63 - hsb.unwrap_or(0);
            quote! {
                (((#accessor) << #shamt) as i64) >> #shamt
            }
        }
        "Reg" => {
            if hsb.unwrap_or_default() > 4 {
                panic!("Reg can only be accessed with 5 bits");
            }
            quote! {
                {
                    let acc = #accessor;
                    unsafe { Reg::from_u5(acc as u8) }
                }
            }
        }
        "FReg" => {
            if hsb.unwrap_or_default() > 4 {
                panic!("FReg can only be accessed with 5 bits");
            }
            quote! {
                {
                    let acc = #accessor;
                    unsafe { FReg::from_u5(acc as u8) }
                }
            }
        }
        "CReg" => {
            if hsb.unwrap_or_default() > 2 {
                panic!("CReg can only be accessed with 3 bits");
            }
            quote! {
                {
                    let acc = #accessor;
                    unsafe { Reg::from_u5(acc as u8 + 8) }
                }
            }
        }
        _ => accessor,
    }
}
