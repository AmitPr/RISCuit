use syn::{Ident, LitInt};

pub mod bitspec;
pub mod r#match;
pub mod opcode;

pub use {bitspec::*, opcode::*, r#match::*};

pub fn mask(width: usize, shift: usize) -> LitInt {
    let mask: u64 = (1 << width) - 1;
    let mask: u64 = mask << shift;
    let mask = format!("{:#b}", mask);

    LitInt::new(&mask, proc_macro2::Span::call_site())
}

pub fn isa_ident(isa: &str) -> Ident {
    // capitalize first letter
    let isa = isa
        .chars()
        .next()
        .unwrap()
        .to_uppercase()
        .chain(isa.chars().skip(1))
        .collect::<String>();
    Ident::new(&isa, proc_macro2::Span::call_site())
}
