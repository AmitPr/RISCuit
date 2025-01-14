use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    LitInt, Result, Token,
};

pub enum RangeType {
    Range {
        start: u8,
        end: u8,
    },
    Padding {
        /// true for 1, false for 0
        value: bool,
        /// number of padding bits
        count: u8,
    },
}

impl Parse for RangeType {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            let value: LitInt = input.parse()?;
            let bit_value = value.base10_parse::<u8>()? == 1;

            let count = if input.peek(Token![*]) {
                input.parse::<Token![*]>()?;
                let count: LitInt = input.parse()?;
                let count_int = count.base10_parse::<u8>()?;
                if count_int == 0 {
                    return Err(syn::Error::new(
                        count.span(),
                        "Padding count must be greater than 0",
                    ));
                }
                if count_int >= 32 {
                    return Err(syn::Error::new(
                        count.span(),
                        "Padding count must be less than 32",
                    ));
                }
                count_int
            } else {
                1
            };

            Ok(Self::Padding {
                value: bit_value,
                count,
            })
        } else {
            let end = input.parse::<LitInt>()?;
            let start = if input.peek(Token![:]) {
                input.parse::<Token![:]>()?;
                input.parse()?
            } else {
                end.clone()
            };
            let start_int = start.base10_parse::<u8>()?;
            let end_int = end.base10_parse::<u8>()?;
            match (start_int, end_int) {
                (0..=31, 0..=31) if start_int <= end_int => Ok(()),
                (_, _) if start_int > end_int => {
                    Err(syn::Error::new(start.span(), "LSB is greater than MSB"))
                }
                (0..=31, _) => Err(syn::Error::new(end.span(), "LSB must be in [0,31]")),
                (_, 0..=31) => Err(syn::Error::new(start.span(), "MSB must be in [0,31]")),
                _ => Err(syn::Error::new(start.span(), "Range must be in [0,31]")),
            }?;
            Ok(Self::Range {
                start: start.base10_parse()?,
                end: end.base10_parse()?,
            })
        }
    }
}

impl RangeType {
    pub fn width(&self) -> u8 {
        match self {
            Self::Range { start, end } => end - start + 1,
            Self::Padding { count, .. } => *count,
        }
    }
}

// Parses var[(range)|*]
pub struct BitField {
    pub name: syn::Ident,
    pub ranges: Vec<RangeType>,
}

impl Parse for BitField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let ranges;
        bracketed!(ranges in input);
        let ranges = ranges
            .parse_terminated(RangeType::parse, Token![|])?
            .into_iter()
            .collect();
        Ok(BitField { name, ranges })
    }
}
pub struct BitInput {
    pub sign_extend: bool,
    pub field: BitField,
}

impl Parse for BitInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let sign_extend = if input.peek(syn::Ident) && input.peek2(Token![,]) {
            let ident = input.parse::<syn::Ident>()?;
            if ident != "sign" {
                return Err(syn::Error::new(ident.span(), "expected `sign`"));
            }
            input.parse::<Token![,]>()?;
            true
        } else {
            false
        };
        let field = input.parse()?;
        Ok(BitInput { sign_extend, field })
    }
}
