mod bits;
mod fields;
mod isa;

use std::cmp::Ordering;

use fields::operand_accessor_fn;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Path, Token, Variant};

use bits::{BitInput, RangeType};

#[proc_macro]
pub fn bits(input: TokenStream) -> TokenStream {
    let BitInput { sign_extend, field } = parse_macro_input!(input as BitInput);
    let name = field.name;

    let total_width = field.ranges.iter().map(|range| range.width()).sum::<u8>();

    let mut pos = 0;
    let masks = field
        .ranges
        .iter()
        .rev()
        .map(|selector| match selector {
            RangeType::Range(range) => {
                // Place mask in the correct position
                let mask = 1u32
                    .wrapping_shl(range.width() as _)
                    .wrapping_sub(1)
                    .wrapping_shl(pos as _);
                let mask = format!(
                    "0b{mask:0width$b}",
                    mask = mask,
                    width = total_width as usize
                );
                let mask = syn::LitInt::new(&mask, proc_macro2::Span::call_site());

                // Calculate shift before applying mask
                let shift = match range.start().cmp(&pos) {
                    Ordering::Less => {
                        let shift = pos - range.start();
                        quote! { (#name << #shift) }
                    }
                    Ordering::Equal => quote! { #name },
                    Ordering::Greater => {
                        let shift = range.start() - pos;
                        quote! { (#name >> #shift) }
                    }
                };
                pos += range.width();

                quote! { (#shift & #mask) }
            }
            RangeType::Padding { value, count } => {
                // Create a mask with the specified number of bits
                let mask = if *value {
                    if *count >= 32 {
                        0xFFFFFFFF
                    } else {
                        1u32.wrapping_shl(*count as _) - 1
                    }
                } else {
                    0
                };
                let mask = format!(
                    "0b{mask:0width$b}",
                    mask = mask,
                    width = total_width as usize
                );
                let mask = syn::LitInt::new(&mask, proc_macro2::Span::call_site());
                pos += count;

                quote! { #mask }
            }
        })
        .collect::<Vec<_>>();

    let value = quote! { (#(#masks)|*) };
    if sign_extend {
        // shift so high bit is in position 31, convert to i32, and shift back
        let shift = 32 - total_width;
        quote! { (((#value << #shift) as i32) >> #shift) }
    } else {
        value
    }
    .into()
}

struct OpcodeEnum {
    pub _vis: syn::Visibility,
    pub _enum: Token![enum],
    pub ident: syn::Ident,
    pub _brace: syn::token::Brace,
    pub variants: Punctuated<Variant, Token![,]>,
}

impl Parse for OpcodeEnum {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _vis: input.parse()?,
            _enum: input.parse()?,
            ident: input.parse()?,
            _brace: syn::braced!(content in input),
            variants: content.parse_terminated(Variant::parse, Token![,])?,
        })
    }
}

#[proc_macro_attribute]
pub fn instructions(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as OpcodeEnum);

    let opcodes = input
        .variants
        .into_iter()
        .map(Opcode::new)
        .collect::<syn::Result<Vec<_>>>()
        .unwrap();

    let opcode_structs = opcodes
        .iter()
        .map(Opcode::codegen_struct)
        .collect::<proc_macro2::TokenStream>();

    let opcode_names = opcodes.iter().map(|o| &o.name);

    let enum_ident = input.ident;
    let opcode_enum = quote! {
        #[derive(Debug, PartialEq, Eq)]
        pub enum #enum_ident {
            #(#opcode_names (#opcode_names),)*
        }
    };

    let code = quote! {
        #opcode_enum
        #opcode_structs
    };

    code.into()
}

struct Opcode {
    fields: Vec<syn::Ident>,
    name: syn::Ident,
}

impl Opcode {
    pub fn new(variant: Variant) -> syn::Result<Self> {
        let mut fields = Vec::new();
        for attr in &variant.attrs {
            let list = attr.meta.require_list()?;
            if attr.path().is_ident("fields") {
                let paths: Punctuated<Path, syn::Token![,]> =
                    list.parse_args_with(Punctuated::parse_terminated)?;
                let as_idents = paths
                    .iter()
                    .map(|p| p.require_ident().cloned())
                    .collect::<syn::Result<_>>()?;
                fields = as_idents;
            } else if attr.path().is_ident("isa") {
            } else {
                return Err(syn::Error::new_spanned(
                    attr,
                    "Unknown attribute, expected `fields` or `isa`",
                ));
            }
        }
        Ok(Opcode {
            fields,
            name: variant.ident,
        })
    }

    pub fn codegen_struct(&self) -> proc_macro2::TokenStream {
        let name = self.name.clone();

        let field_accessors = self
            .fields
            .iter()
            .cloned()
            .map(operand_accessor_fn)
            .collect::<proc_macro2::TokenStream>();

        let as_struct = quote! {
            #[derive(Debug, PartialEq, Eq)]
            pub struct #name;

            impl #name {
                #field_accessors
            }

            impl Into<Opcode> for #name {
                fn into(self) -> Opcode {
                    Opcode::#name(self)
                }
            }
        };

        as_struct
    }
}
