use std::str::FromStr;

use syn::parse::Parse;

#[derive(Debug, Clone, Copy)]
pub enum Base {
    RV32I,
    RV64I,
    RV128I,
}

impl FromStr for Base {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RV32I" => Ok(Base::RV32I),
            "RV64I" => Ok(Base::RV64I),
            "RV128I" => Ok(Base::RV128I),
            _ => Err(format!("Unknown base ISA: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Extension {
    M,
    F,
    Q,
    D,
    A,
    S,
    C,
}

impl FromStr for Extension {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "M" => Ok(Extension::M),
            "F" => Ok(Extension::F),
            "Q" => Ok(Extension::Q),
            "D" => Ok(Extension::D),
            "A" => Ok(Extension::A),
            "S" => Ok(Extension::S),
            "C" => Ok(Extension::C),
            _ => Err(format!("Unknown extension: {}", s)),
        }
    }
}

impl Parse for Base {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lit_str = input.parse::<syn::LitStr>()?;
        Self::from_str(lit_str.value().as_str())
            .map_err(|_| syn::Error::new(lit_str.span(), "Unknown base ISA"))
    }
}

impl Parse for Extension {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lit_str = input.parse::<syn::LitStr>()?;
        Self::from_str(lit_str.value().as_str())
            .map_err(|_| syn::Error::new(lit_str.span(), "Unknown extension"))
    }
}

pub fn isa_inherits_from(base: Base) -> Vec<Base> {
    match base {
        Base::RV32I => vec![Base::RV32I],
        Base::RV64I => vec![Base::RV64I, Base::RV32I],
        Base::RV128I => vec![Base::RV128I, Base::RV64I, Base::RV32I],
        _ => panic!("Unknown ISA: {:?}", base),
    }
}

pub fn resolve_active_isas(base: Base, extensions: Vec<Extension>) -> Vec<(Base, Extension)> {
    let required_bases = isa_inherits_from(base);
    // required_bases X extensions
    required_bases
        .iter()
        .flat_map(|base| std::iter::repeat(*base).zip(extensions.iter().copied()))
        .collect::<Vec<_>>()
}
