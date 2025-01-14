mod bits;
mod fields;
mod isa;

use std::{cmp::Ordering, str::FromStr};

use fields::operand_accessor_fn;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, LitInt, Token, Variant};

use bits::{BitInput, RangeType};

#[proc_macro]
pub fn bits(input: TokenStream) -> TokenStream {
    let BitInput { sign_extend, field } = parse_macro_input!(input as BitInput);
    let name = field.name;

    let total_width = field.ranges.iter().map(|range| range.width()).sum::<u8>() as usize;

    let mut pos = 0;
    let masks = field
        .ranges
        .iter()
        .rev()
        .map(|selector| match selector {
            // Shift the input to the correct position and then apply a positioned mask
            RangeType::Range { start, .. } => {
                // Place mask in the correct position
                let mask = ((1u32 << selector.width()) - 1) << pos;
                let mask = format!("0b{mask:0total_width$b}",);
                let mask = syn::LitInt::new(&mask, proc_macro2::Span::call_site());

                // Calculate shift before applying mask
                let shift = match start.cmp(&pos) {
                    Ordering::Less => {
                        let shift = pos - start;
                        quote! { (#name << #shift) }
                    }
                    Ordering::Equal => quote! { #name },
                    Ordering::Greater => {
                        let shift = start - pos;
                        quote! { (#name >> #shift) }
                    }
                };
                pos += selector.width();

                quote! { (#shift & #mask) }
            }
            // Pad the input with the specified number of bits of the specified value
            RangeType::Padding { value, count } => {
                // Create a mask with the specified number of bits
                let value = if *value { (1u32 << *count) - 1 } else { 0 };
                let padding = format!("0b{value:0total_width$b}");
                let padding = LitInt::new(&padding, Span::call_site());
                pos += count;

                quote! { #padding }
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

#[derive(Debug, Clone)]
struct Field {
    field: syn::Ident,
    alias: Option<syn::Ident>,
}

impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let field = input.parse()?;
        let alias = if input.peek(Token![as]) {
            input.parse::<Token![as]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Field { field, alias })
    }
}

struct Opcode {
    fields: Vec<Field>,
    name: syn::Ident,
    #[allow(dead_code)]
    isa: Option<isa::Base>,
    extension: Option<isa::Extension>,
}

impl Opcode {
    pub fn new(variant: Variant) -> syn::Result<Self> {
        let mut fields = Vec::new();
        let mut isa = None;
        let mut extension = None;
        for attr in &variant.attrs {
            let list = attr.meta.require_list()?;
            if attr.path().is_ident("fields") {
                fields = list
                    .parse_args_with(Punctuated::<Field, syn::Token![,]>::parse_terminated)?
                    .into_iter()
                    .collect();
            } else if attr.path().is_ident("isa") {
                // Parse #[isa(base = "BASE", ext = "EXT")]
                let fields: Punctuated<syn::MetaNameValue, syn::Token![,]> =
                    list.parse_args_with(Punctuated::parse_terminated)?;
                for field in fields {
                    match field.path.get_ident() {
                        Some(ident) if ident == "base" => {
                            if let syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(lit),
                                ..
                            }) = field.value
                            {
                                isa = Some(
                                    isa::Base::from_str(lit.value().as_str())
                                        .map_err(|e| syn::Error::new_spanned(lit, e))?,
                                );
                            } else {
                                return Err(syn::Error::new_spanned(
                                    field,
                                    "Expected a string literal",
                                ));
                            };
                        }
                        Some(ident) if ident == "ext" => {
                            if let syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(lit),
                                ..
                            }) = field.value
                            {
                                extension = Some(
                                    isa::Extension::from_str(lit.value().as_str())
                                        .map_err(|e| syn::Error::new_spanned(lit, e))?,
                                );
                            } else {
                                return Err(syn::Error::new_spanned(
                                    field,
                                    "Expected a string literal",
                                ));
                            };
                        }
                        _ => {
                            return Err(syn::Error::new_spanned(
                                field.path,
                                "Unknown field, expected `base` or `ext`",
                            ));
                        }
                    }
                }
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
            isa,
            extension,
        })
    }

    pub fn codegen_struct(&self) -> proc_macro2::TokenStream {
        let name = self.name.clone();
        let inner = match self.extension {
            Some(isa::Extension::C) => quote! { u16 },
            _ => quote! { u32 },
        };

        let field_accessors = self
            .fields
            .iter()
            .cloned()
            .map(operand_accessor_fn)
            .collect::<proc_macro2::TokenStream>();

        let fields_string = self
            .fields
            .iter()
            .map(|f| {
                f.alias
                    .as_ref()
                    .unwrap_or(&f.field)
                    .to_string()
                    .to_ascii_lowercase()
            })
            .collect::<Vec<_>>()
            .join(", ");

        let debug_impl = self.codegen_debug_impl();
        let display_impl = self.codegen_display_impl();

        let as_struct = quote! {
            #[derive(PartialEq, Eq)]
            #[doc = stringify!(#fields_string)]
            pub struct #name(pub #inner);

            impl #name {
                #field_accessors
            }

            impl Into<Opcode> for #name {
                fn into(self) -> Opcode {
                    Opcode::#name(self)
                }
            }

            #debug_impl
            #display_impl
        };

        as_struct
    }

    /// Converts the Pascal Case `name` to the instruction as if in disassembler
    pub fn name_to_inst(&self) -> String {
        let mut sections = vec![];
        for c in self.name.to_string().chars() {
            if c.is_uppercase() {
                sections.push(vec![]);
            }
            sections.last_mut().unwrap().push(c);
        }
        sections
            .iter()
            .map(|s| s.iter().collect::<String>().to_lowercase())
            .collect::<Vec<_>>()
            .join(".")
    }

    pub fn codegen_debug_impl(&self) -> proc_macro2::TokenStream {
        let name = self.name.clone();
        let fields = self
            .fields
            .iter()
            .map(|f| f.alias.as_ref().unwrap_or(&f.field));
        quote! {
            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!(#name))
                        .field("inst", &self.0)
                        #( .field(stringify!(#fields), &self.#fields()) )*
                        .finish()
                }
            }
        }
    }

    pub fn codegen_display_impl(&self) -> proc_macro2::TokenStream {
        let name = self.name.clone();
        let fields = self
            .fields
            .iter()
            .map(|f| f.alias.as_ref().unwrap_or(&f.field));
        let inst = self.name_to_inst();
        quote! {
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, #inst)?;
                    #( write!(f, " {}", self.#fields())?; )*
                    Ok(())
                }
            }
        }
    }
}
