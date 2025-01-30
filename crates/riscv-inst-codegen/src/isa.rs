use std::fmt::Display;

use syn::Ident;

use crate::Opcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Base {
    RV32,
    RV64,
}

impl Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Base::RV32 => write!(f, "rv32"),
            Base::RV64 => write!(f, "rv64"),
        }
    }
}

impl Base {
    pub const fn capitalized(&self) -> &'static str {
        match self {
            Base::RV32 => "Rv32",
            Base::RV64 => "Rv64",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FExt {
    F,
    D,
    Q,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RvExt {
    M,
    A,
    C,
    S,
    F(FExt),
}

impl RvExt {
    pub const fn to_char(&self) -> &'static str {
        match self {
            RvExt::M => "m",
            RvExt::A => "a",
            RvExt::C => "c",
            RvExt::S => "s",
            RvExt::F(FExt::F) => "f",
            RvExt::F(FExt::D) => "fd",
            RvExt::F(FExt::Q) => "fqq",
        }
    }

    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            'm' => Some(RvExt::M),
            'a' => Some(RvExt::A),
            'c' => Some(RvExt::C),
            's' => Some(RvExt::S),
            'f' => Some(RvExt::F(FExt::F)),
            'd' => Some(RvExt::F(FExt::D)),
            'q' => Some(RvExt::F(FExt::Q)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Isa {
    pub base: Base,
    pub exts: &'static [RvExt],
}

impl Isa {
    pub const fn new(base: Base, exts: &'static [RvExt]) -> Self {
        Isa { base, exts }
    }

    pub fn contains(&self, ext_str: &str) -> bool {
        let (ext_base, rest) = if let Some(rest) = ext_str.strip_prefix("rv32") {
            (Base::RV32, rest)
        } else if let Some(rest) = ext_str.strip_prefix("rv64") {
            (Base::RV64, rest)
        } else {
            return false;
        };
        if ext_base != self.base {
            return false;
        }

        rest.chars()
            .filter_map(RvExt::from_char)
            .all(|ext| self.exts.contains(&ext))
    }

    pub fn contains_op(&self, op: Opcode) -> bool {
        op.isas.iter().any(|isa| self.contains(isa))
    }

    pub fn is_c(&self) -> bool {
        self.exts.contains(&RvExt::C)
    }

    pub fn ident(&self) -> Ident {
        let base = self.base.capitalized();
        let exts = self
            .exts
            .iter()
            .map(|e| RvExt::to_char(e).to_ascii_uppercase())
            .collect::<String>();

        Ident::new(
            &format!("{}I{}", base, exts),
            proc_macro2::Span::call_site(),
        )
    }
}

impl Display for Isa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ext_str = self.exts.iter().map(RvExt::to_char).collect::<String>();
        write!(f, "{}i{}", self.base, ext_str)
    }
}
