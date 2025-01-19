use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitInt};

use crate::isa_ident;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BitEnc {
    pub start: usize,
    pub end: usize,
    pub value: u32,
}

impl BitEnc {
    pub fn parse(spec: String) -> Option<Self> {
        let mut parts = spec.split('=');
        let mut range = parts.next().unwrap().split("..");
        let end = range.next().unwrap().parse::<usize>().unwrap();
        let start = range.next().map_or(end, |end| end.parse().unwrap());

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

    pub fn codegen_extract(&self, src: Ident) -> TokenStream {
        let width = self.end - self.start + 1;
        let mask: u64 = (1 << (self.end - self.start + 1)) - 1;
        let mask = LitInt::new(
            &format!("{:#0width$b}", mask),
            proc_macro2::Span::call_site(),
        );
        let shift = self.start;

        if shift == 0 {
            quote! { (#src & #mask) }
        } else {
            quote! { ((#src >> #shift) & #mask) }
        }
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
}

impl Opcode {
    pub fn parse(mut args: Vec<String>) -> Self {
        let name = args[0].clone();
        let mut isas = vec![];
        let mut codec = None;
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
            .filter(|&arg| arg.chars().next().map_or(false, |c| c.is_alphabetic()))
            .cloned()
            .collect();

        let encodings = args[1..]
            .iter()
            .filter(|&arg| arg.chars().next().map_or(false, |c| c.is_numeric()))
            .cloned()
            .filter_map(BitEnc::parse)
            .collect();

        Self {
            name,
            operands,
            encodings,
            codec,
            isas,
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

        syn::Ident::new(&opcode, proc_macro2::Span::call_site())
    }

    pub fn codegen_struct(
        &self,
        accessors: &HashMap<String, TokenStream>,
        rv_c: bool,
    ) -> TokenStream {
        let opcode_ident = self.name_ident();
        let operand_accessors = self
            .operands
            .iter()
            .map(|operand| {
                accessors.get(operand).unwrap_or_else(|| {
                    panic!("Failed to find accessor for operand '{}'", operand);
                })
            })
            .collect::<Vec<_>>();
        let operand_inner_ty = if rv_c {
            quote! { pub u16 }
        } else {
            quote! { pub u32 }
        };

        quote! {
            pub struct #opcode_ident(#operand_inner_ty);

            impl #opcode_ident {
                #(#operand_accessors)*
            }
        }
    }

    pub fn full_instance(&self, base: &str, src: Ident) -> TokenStream {
        let ident = self.name_ident();
        let relevant_isa = isa_ident(self.isas.iter().find(|isa| isa.starts_with(base)).unwrap());
        let base_ident = isa_ident(base);

        quote! { #base_ident::#relevant_isa(#relevant_isa::#ident(#ident(#src as _))) }
    }
}
