use syn::{
    bracketed,
    parse::{Parse, ParseStream, Peek},
    LitInt, Result, Token,
};

pub enum RangeType {
    Range(BitRange),
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
                count.base10_parse::<u8>()?
            } else {
                1
            };

            Ok(Self::Padding {
                value: bit_value,
                count,
            })
        } else {
            Ok(Self::Range(input.parse()?))
        }
    }
}

impl RangeType {
    pub fn width(&self) -> u8 {
        match self {
            Self::Range(range) => range.width(),
            Self::Padding { count, .. } => *count,
        }
    }
}

pub struct BitRange {
    end: LitInt,
    start: LitInt,
}

impl Parse for BitRange {
    fn parse(input: ParseStream) -> Result<Self> {
        let end: LitInt = input.parse()?;
        let start = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            input.parse()?
        } else {
            end.clone()
        };
        Ok(BitRange { end, start })
    }
}

impl BitRange {
    pub fn start(&self) -> u8 {
        self.start.base10_parse::<u8>().unwrap()
    }
    pub fn end(&self) -> u8 {
        self.end.base10_parse::<u8>().unwrap()
    }
    pub fn width(&self) -> u8 {
        self.end() - self.start() + 1
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
