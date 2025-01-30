use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitInt};

use crate::isa::Isa;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// Parses M..L=Val
pub struct BitEnc {
    pub start: usize,
    pub end: usize,
    pub value: u32,
}

impl BitEnc {
    pub fn parse(spec: impl AsRef<str>) -> Option<Self> {
        let mut parts = spec.as_ref().split('=');
        let mut range = parts.next().unwrap().split("..");
        let end = range.next().unwrap().parse::<usize>().unwrap(); // First
        let start = range.next().map_or(end, |end| end.parse().unwrap()); // Maybe second

        // "ignore" in some specs e.g. 14..2=ignore
        let value = parts.next().unwrap();
        if value == "ignore" {
            None
        } else {
            let value = if let Some(value) = value.strip_prefix("0x") {
                u32::from_str_radix(value, 16).unwrap()
            } else {
                value.parse::<u32>().unwrap()
            };

            Some(Self { start, end, value })
        }
    }

    pub fn eq_range(&self, start: usize, end: usize) -> bool {
        self.start == start && self.end == end
    }

    /// Code for extracting this range to bottom bits
    pub fn codegen_extract(&self, src: Ident) -> TokenStream {
        let mask = crate::mask(self.end - self.start + 1, 0);
        let shift = self.start;

        if shift == 0 {
            quote! { (#src & #mask) }
        } else {
            quote! { ((#src >> #shift) & #mask) }
        }
    }

    pub fn width(&self) -> usize {
        self.end - self.start + 1
    }
}

#[derive(Debug, Clone)]
pub struct Opcode {
    pub name: String,
    pub operands: Vec<String>,
    pub encodings: Vec<BitEnc>,
    #[allow(unused)]
    pub codec: String,
    pub isas: Vec<String>,
    pub discriminant: Option<u8>,
}

impl Opcode {
    pub fn parse(mut args: Vec<String>) -> Self {
        let name = args[0].clone();
        let mut isas = vec![];
        let mut codec = None;

        // Last arguments are ISAs, then codec
        while let Some(arg) = args.pop() {
            if arg.starts_with("rv") {
                isas.push(arg);
            } else {
                codec = Some(arg);
                break;
            }
        }
        let codec = codec.unwrap();

        let operands = args[1..]
            .iter()
            .filter(|arg| arg.chars().next().map_or(false, char::is_alphabetic))
            .cloned()
            .collect();

        let encodings = args[1..]
            .iter()
            .filter(|arg| arg.chars().next().map_or(false, char::is_numeric))
            .filter_map(BitEnc::parse)
            .collect();

        Self {
            name,
            operands,
            encodings,
            codec,
            isas,
            discriminant: None,
        }
    }

    pub fn name_ident(&self) -> syn::Ident {
        let opcode = self
            .name
            .split('.')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().chain(chars).collect(),
                }
            })
            .collect::<String>();

        Ident::new(&opcode, Span::call_site())
    }

    pub fn codegen_struct(&self, accessors: &HashMap<String, (Ident, TokenStream)>) -> TokenStream {
        let opcode_ident = self.name_ident();
        let acessor_impls = self
            .operands
            .iter()
            .map(|operand| &accessors.get(operand).expect("Failed to find accessor").1)
            .collect::<Vec<_>>();

        let impls = {
            let name = &self.name;
            quote! {
                impl std::fmt::Debug for #opcode_ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.debug_struct(stringify!(#name)).finish()
                    }
                }

                impl std::fmt::Display for #opcode_ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", #name)
                    }
                }
            }
        };

        quote! {
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub struct #opcode_ident;

            impl #opcode_ident {
                #(#acessor_impls)*
            }

            #impls
        }
    }

    pub fn full_instance(&self, isa: &Isa) -> TokenStream {
        let ident = self.name_ident();
        let isa_ident = isa.ident();

        quote! { #isa_ident::#ident(#ident) }
    }

    pub fn as_variant(&self) -> TokenStream {
        let name = self.name_ident();
        if let Some(d) = self.discriminant {
            let d = LitInt::new(&format!("{}", d), Span::call_site());
            quote! { #name(#name) = #d }
        } else {
            quote! { #name(#name) }
        }
    }

    pub fn is_c(&self) -> bool {
        self.isas.iter().any(|isa| isa.contains("c"))
    }

    pub fn mask_match(&self) -> (u32, u32) {
        let mut mask = 0;
        let mut value = 0;

        for enc in &self.encodings {
            let width = enc.width();
            let start = enc.start;

            mask |= ((1 << width) - 1) << start;
            value |= enc.value << start;
        }

        (mask, value)
    }
}
