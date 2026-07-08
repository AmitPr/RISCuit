use proc_macro2::TokenStream;
use quote::quote;

use crate::Opcode;

/// Bits of a 32-bit instruction covered by the flat table index:
/// opcode (0-6, low two implied 0b11), funct3 (12-14), funct7 (25-31).
const KNOWN_MASK: u32 = 0xFE00_707F;

/// Build a 2^15-entry table indexed by opcode[6:2] | funct3 << 5 | funct7 << 8
/// mapping each cell to an exact discriminant or a sentinel. Exactness is
/// preserved: a cell only gets a discriminant when exactly one instruction
/// matches it and all of that instruction's fixed bits lie inside the index.
///
/// The sentinels are the ISA's `Invalid`/`Slow` variant discriminants: every
/// table value is a valid enum variant, so dispatch needs no range check.
pub fn generate_lookup_table32(
    uncompressed: &[&mut Opcode],
    disc_invalid: u8,
    disc_slow: u8,
) -> TokenStream {
    let mapping = uncompressed
        .iter()
        .map(|o| {
            let (mask, value) = o.mask_match();
            let d = o.discriminant.unwrap();
            assert!(d < disc_invalid, "discriminant collides with sentinel");
            (d, mask, value)
        })
        .collect::<Vec<_>>();

    let mut table = vec![disc_invalid; 1 << 15];
    for (idx, cell) in table.iter_mut().enumerate() {
        let idx = idx as u32;
        let known = 0b11 | ((idx & 0x1F) << 2) | (((idx >> 5) & 0x7) << 12) | ((idx >> 8) << 25);

        let mut exact = None;
        let mut ambiguous = false;
        for &(d, mask, value) in &mapping {
            let m_in = mask & KNOWN_MASK;
            if known & m_in == value & m_in {
                if mask & !KNOWN_MASK == 0 {
                    ambiguous |= exact.replace(d).is_some();
                } else {
                    ambiguous = true;
                }
            }
        }

        *cell = match (exact, ambiguous) {
            (Some(d), false) => d,
            (None, false) => disc_invalid,
            _ => disc_slow,
        };
    }

    quote! { [ #(#table),* ] }
}

pub fn generate_lookup_table(compressed: &[&mut Opcode]) -> TokenStream {
    let mapping = compressed
        .iter()
        .map(|o| {
            let (mask, value) = o.mask_match();
            (
                o.discriminant.unwrap(),
                mask as u16,
                value as u16,
                o.encodings.len(),
            )
        })
        .collect::<Vec<_>>();

    let mut table: [u8; 0xFFFF] = [0; 0xFFFF];
    for i in (0..u16::MAX).filter(|i| i & 0b11 != 0b11) {
        let mut op = mapping
            .iter()
            .filter_map(|(d, m, v, l)| if (i & m) == *v { Some((*d, l)) } else { None })
            .collect::<Vec<_>>();
        // Sort by more restrictive encodings first
        op.sort_by(|(_, a), (_, b)| b.cmp(a));

        if let Some((op, _)) = op.first() {
            table[i as usize] = *op;
        }
    }

    quote! { [ #(#table),* ] }
}
