use proc_macro2::TokenStream;
use quote::quote;

use crate::Opcode;

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
