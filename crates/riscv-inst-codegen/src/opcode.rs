use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::isa::Isa;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// Parses M..L=Val
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
        let mask = crate::mask(self.end - self.start + 1, 0);
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

    pub fn codegen_struct(&self, accessors: &HashMap<String, (Ident, TokenStream)>) -> TokenStream {
        let opcode_ident = self.name_ident();
        let operand_accessors = self
            .operands
            .iter()
            .map(|operand| {
                &accessors
                    .get(operand)
                    .unwrap_or_else(|| {
                        panic!("Failed to find accessor for operand '{}'", operand);
                    })
                    .1
            })
            .collect::<Vec<_>>();
        let operand_inner_ty = if self.is_c() {
            quote! { pub u16 }
        } else {
            quote! { pub u32 }
        };

        let impls = {
            let name = &self.name;
            let operand_accessors = self
                .operands
                .iter()
                .map(|operand| {
                    &accessors
                        .get(operand)
                        .unwrap_or_else(|| {
                            panic!("Failed to find accessor for operand '{}'", operand);
                        })
                        .0
                })
                .collect::<Vec<_>>();

            quote! {
                impl std::fmt::Debug for #opcode_ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.debug_struct(stringify!(#name))
                            .field("inst", &self.0)
                            #( .field(stringify!(#operand_accessors), &self.#operand_accessors()) )*
                            .finish()
                    }
                }

                impl std::fmt::Display for #opcode_ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", #name)?;
                        #(write!(f, " {:?}", self.#operand_accessors())?;)*
                        Ok(())
                    }
                }
            }
        };

        quote! {
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub struct #opcode_ident(#operand_inner_ty);

            impl #opcode_ident {
                #(#operand_accessors)*
            }

            #impls
        }
    }

    pub fn full_instance(&self, isa: &Isa, src: Ident) -> TokenStream {
        let ident = self.name_ident();
        let isa_ident = isa.ident();

        quote! { #isa_ident::#ident(#ident(#src as _)) }
    }

    pub fn is_c(&self) -> bool {
        self.isas.iter().any(|isa| isa.contains("c"))
    }
}
