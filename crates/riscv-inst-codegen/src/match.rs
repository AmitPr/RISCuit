use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitInt};

use crate::{
    isa::Isa,
    lookup_tables::generate_lookup_table,
    opcode::{BitEnc, Opcode},
};

pub fn generate_opcode_parser(
    opcodes: &mut [Opcode],
    isa: Isa,
) -> (Option<TokenStream>, TokenStream) {
    let (mut compressed, mut uncompressed) =
        opcodes.iter_mut().partition::<Vec<_>, _>(|o| o.is_c());

    // Assign discriminants to compressed, then uncompressed opcodes
    let mut assigned_cunimp = false;
    compressed
        .iter_mut()
        .chain(uncompressed.iter_mut())
        .enumerate()
        .for_each(|(i, c)| {
            c.discriminant = if c.name == "c.unimp" {
                assigned_cunimp = true;
                Some(0)
            } else {
                let i = if assigned_cunimp { i } else { i + 1 };
                Some(i as u8)
            }
        });

    let table = if compressed.is_empty() {
        None
    } else {
        Some(generate_lookup_table(&compressed))
    };
    let c_decode = if table.is_some() {
        let include_path = format!("./{isa}_lookup_table.rs");
        quote! {
            const C_LOOKUP: [u8; 0xFFFF] = include!(#include_path);

            #[allow(clippy::missing_transmute_annotations)]
            Some(unsafe { core::mem::transmute(*C_LOOKUP.get_unchecked(inst as u16 as usize)) })
        }
    } else {
        quote! { None }
    };

    let tree = build_decision_tree(&uncompressed);
    let return_ty = isa.ident();
    let parser = build_match(
        Ident::new("inst", proc_macro2::Span::call_site()),
        isa,
        &tree,
        &uncompressed,
        None,
    );

    let decode_fn = quote! {
        #[inline(always)]
        pub fn parse(inst: u32) -> Option<#return_ty> {
            if inst & 0b11 != 0b11 {
                #c_decode
            } else {
                #parser
            }
        }
    };

    (table, decode_fn)
}

#[derive(Debug)]
enum DecisionNode {
    Leaf(Vec<usize>),
    Branch {
        bit_range: (usize, usize),
        children: Vec<(u32, Box<DecisionNode>)>,
    },
}

fn build_decision_tree(encodings: &[&mut Opcode]) -> DecisionNode {
    let all_indices: Vec<usize> = (0..encodings.len()).collect();
    build_tree_node(encodings, &all_indices)
}

fn build_tree_node(ops: &[&mut Opcode], indices: &[usize]) -> DecisionNode {
    // If we have 1 or fewer instructions, make a leaf
    if indices.len() <= 1 {
        return DecisionNode::Leaf(indices.to_vec());
    }

    // Find the most discriminating bit range
    let bit_counts = bit_ranges(ops, indices);
    let (start, end) = find_best_bit_range(&bit_counts, indices);

    // Group instructions by their pattern in this range
    let mut groups: BTreeMap<u32, Vec<usize>> = BTreeMap::new();

    for &idx in indices {
        let pattern = ops[idx]
            .encodings
            .iter()
            .filter(|enc| enc.eq_range(start, end))
            .map(|enc| enc.value)
            .next()
            .unwrap();

        groups.entry(pattern).or_default().push(idx);
    }

    // If we couldn't find any discriminating bits, make a leaf
    if groups.len() == 1 {
        return DecisionNode::Leaf(indices.to_vec());
    }

    // Recursively build child nodes
    let children: Vec<(u32, Box<DecisionNode>)> = groups
        .into_iter()
        .map(|(pattern, group_indices)| {
            let child = build_tree_node(ops, &group_indices);
            (pattern, Box::new(child))
        })
        .collect();

    DecisionNode::Branch {
        bit_range: (start, end),
        children,
    }
}

fn bit_ranges(
    opcodes: &[&mut Opcode],
    indices: &[usize],
) -> BTreeMap<(usize, usize), (BTreeSet<u32>, usize)> {
    let mut bit_counts: BTreeMap<(usize, usize), (BTreeSet<u32>, usize)> = BTreeMap::new();

    for i in indices {
        for BitEnc { start, end, value } in &opcodes[*i].encodings {
            let (patterns, count) = bit_counts.entry((*start, *end)).or_default();
            *count += 1;
            patterns.insert(*value);
        }
    }

    bit_counts
}

fn find_best_bit_range(
    bit_counts: &BTreeMap<(usize, usize), (BTreeSet<u32>, usize)>,
    indices: &[usize],
) -> (usize, usize) {
    // Get encoding with most patterns, where count == len(indices)
    let (start, end) = bit_counts
        .iter()
        .filter(|(_, (_, count))| *count == indices.len())
        .max_by_key(|(_, (patterns, _))| patterns.len())
        .map(|(range, _)| range)
        .unwrap();

    (*start, *end)
}

fn build_match(
    src: Ident,
    isa: Isa,
    cur: &DecisionNode,
    opcodes: &[&mut Opcode],
    pattern: Option<LitInt>,
) -> TokenStream {
    match cur {
        DecisionNode::Leaf(vec) => {
            // There should have been at least one branch before this leaf
            let pattern = pattern.unwrap();
            if vec.len() == 1 {
                // Only one instruction, just return that match arm
                let opcode = &opcodes[vec[0]];
                let ident = opcode.full_instance(&isa);
                quote! {
                   #pattern => Some(#ident)
                }
            } else {
                // The instructions are equal, but some have more restrictive encoding
                // e.g. c.addi16sp = c.lui with  11..7=2
                let mut opcodes = vec
                    .iter()
                    .map(|&idx| (&opcodes[idx], opcodes[idx].encodings.clone()))
                    .collect::<Vec<_>>();
                opcodes.sort_by_key(|(_, encodings)| encodings.len());

                // The first opcode has all the common encodings
                let common = &opcodes[0].1;

                let mut arms = vec![];
                // Iterate backwards, from most to least restrictive
                for (op, encs) in opcodes.iter().rev() {
                    let unique = encs
                        .iter()
                        .filter(|enc| !common.contains(enc))
                        .collect::<Vec<_>>();

                    let ident = op.full_instance(&isa);

                    if unique.is_empty() {
                        arms.push(quote! { #pattern => Some(#ident) });
                    } else {
                        let if_expr = unique.into_iter().map(|enc| {
                            let extract = enc.codegen_extract(src.clone());
                            let value = LitInt::new(
                                &format!("{}", enc.value),
                                proc_macro2::Span::call_site(),
                            );
                            quote! { #extract == #value }
                        });
                        arms.push(quote! { #pattern if #(#if_expr)&&* => Some(#ident) });
                    }
                }

                quote! {
                    #(#arms),*
                }
            }
        }
        DecisionNode::Branch {
            bit_range: (start, end),
            children,
        } => {
            let enc = BitEnc {
                start: *start,
                end: *end,
                value: 0,
            };
            let match_expr = enc.codegen_extract(src.clone());

            // TODO: The manual "unreachable!()" + None branches seem like they should
            // outperform, but it seems worse!
            const EXHAUSTIVE_MATCH_THRESHOLD: usize = 0;
            let arms = (0..(2u32.pow(enc.width() as u32)))
                .filter_map(|i| {
                    let width = enc.width();
                    let pattern = LitInt::new(&format!("{:#0width$b}", i), Span::call_site());

                    if let Some((_, child)) = children.iter().find(|(pat, _)| *pat == i) {
                        Some(build_match(src.clone(), isa, child, opcodes, Some(pattern)))
                    } else if width > EXHAUSTIVE_MATCH_THRESHOLD {
                        // Don't expand for wide bit ranges, just return
                        None
                    } else {
                        Some(quote! { #pattern => None })
                    }
                })
                .collect::<Vec<_>>();

            // Wide bit ranges need the None fallback, otherwise,
            // we've exhaustively checked so unreachable!()
            let fallback = if enc.width() > EXHAUSTIVE_MATCH_THRESHOLD {
                quote! { _ => None }
            } else {
                quote! { _ => unreachable!() }
            };

            if let Some(pattern) = pattern {
                quote! {
                    #pattern => {
                        match #match_expr {
                            #(#arms),*,
                            #fallback
                        }
                    }
                }
            } else {
                // Top-level match
                quote! {
                    match #match_expr {
                        #(#arms),*,
                        #fallback
                    }
                }
            }
        }
    }
}
