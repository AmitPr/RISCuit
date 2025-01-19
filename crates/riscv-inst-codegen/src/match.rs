use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_opcode_parser(opcodes: Vec<Vec<String>>) {
    let all_encodings = opcodes
        .iter()
        .cloned()
        .map(opcode_bit_enc)
        .collect::<Vec<_>>();

    let required_positions = all_encodings
        .iter()
        .fold([0usize; 32], |mut acc, encoding| {
            for (i, bit) in encoding.iter().enumerate() {
                if bit.is_some() {
                    acc[i] += 1;
                }
            }
            acc
        });

    let all_encodings = opcodes
        .iter()
        .cloned()
        .map(opcode_encodings)
        .collect::<Vec<_>>();
    let tree = build_decision_tree(&all_encodings);
    print_tree(&tree, 0, &opcodes);
}

/// Get (start, end, value) triples for defined range of bits
fn opcode_encodings(mut opcode: Vec<String>) -> Vec<(usize, usize, u32)> {
    while let Some(arg) = opcode.pop() {
        if arg.starts_with("rv") {
            continue;
        }
        break;
    }

    // encodings of form:
    // [<end>..]<start>=<bits>
    let encodings = opcode[1..]
        .iter()
        .filter(|&arg| arg.chars().next().map_or(false, |c| c.is_numeric()))
        .cloned();

    let mut triples = vec![];

    for enc in encodings {
        let mut parts = enc.split('=');
        let range = parts.next().unwrap();
        let value = parts.next().unwrap();
        if value == "ignore" {
            continue;
        }
        let value = if let Some(value) = value.strip_prefix("0x") {
            u32::from_str_radix(value, 16).unwrap()
        } else {
            value.parse::<u32>().unwrap()
        };

        let mut range = range.split("..");
        let end = range.next().unwrap().parse::<usize>().unwrap();
        let start = range.next().map_or(end, |end| end.parse().unwrap());

        triples.push((start, end, value));
    }

    triples
}

/// None means the bit can be anything
/// Some(true/false) means the bit has to be 0/1
fn opcode_bit_enc(opcode: Vec<String>) -> [Option<bool>; 32] {
    let mut encoding = [None; 32];
    let encodings = opcode_encodings(opcode);

    for (start, end, mut value) in encodings {
        (start..=end).for_each(|i| {
            let bit = (value & 1) == 1;
            encoding[i] = Some(bit);

            value >>= 1;
        });
    }

    encoding
}

#[derive(Debug)]
enum DecisionNode {
    Leaf(Vec<usize>), // Instruction indices that match this path
    Branch {
        bit_range: (usize, usize),               // Inclusive range of bits to check
        children: Vec<(u32, Box<DecisionNode>)>, // (pattern, child node) pairs
    },
}

fn build_decision_tree(encodings: &[Vec<(usize, usize, u32)>]) -> DecisionNode {
    let all_indices: Vec<usize> = (0..encodings.len()).collect();
    build_tree_node(encodings, &all_indices)
}

fn build_tree_node(encodings: &[Vec<(usize, usize, u32)>], indices: &[usize]) -> DecisionNode {
    // If we have 1 or fewer instructions, make a leaf
    if indices.len() <= 1 {
        return DecisionNode::Leaf(indices.to_vec());
    }

    // Find the most discriminating bit range
    let (start, end) = find_best_bit_range(encodings, indices);
    println!("best range: {}..={}", end, start);

    // Group instructions by their pattern in this range
    let mut groups: HashMap<u32, Vec<usize>> = HashMap::new();

    for &idx in indices {
        let pattern = encodings[idx]
            .iter()
            .filter(|(s, e, _)| *s == start && *e == end)
            .map(|(_, _, v)| *v)
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
            let child = build_tree_node(encodings, &group_indices);
            (pattern, Box::new(child))
        })
        .collect();

    DecisionNode::Branch {
        bit_range: (start, end),
        children,
    }
}

fn find_best_bit_range(op_encs: &[Vec<(usize, usize, u32)>], indices: &[usize]) -> (usize, usize) {
    // range -> (possible values, number of instructions)
    let mut bit_counts: HashMap<(usize, usize), (HashSet<u32>, usize)> = HashMap::new();

    for i in indices {
        for (start, end, value) in &op_encs[*i] {
            let (patterns, count) = bit_counts.entry((*start, *end)).or_default();
            *count += 1;
            patterns.insert(*value);
        }
    }

    // Get encoding with most patterns, where count == len(indices)
    let (start, end) = bit_counts
        .iter()
        .filter(|(_, (_, count))| *count == indices.len())
        .max_by_key(|(_, (patterns, _))| patterns.len())
        .map(|(range, _)| range)
        .unwrap();

    (*start, *end)
}

// Helper function to print the tree for debugging
fn print_tree(node: &DecisionNode, indent: usize, opcodes: &[Vec<String>]) {
    match node {
        DecisionNode::Leaf(indices) => {
            let names = indices.iter().map(|&idx| &opcodes[idx][0]);
            println!(
                "{}Leaf: {:?}",
                " ".repeat(indent),
                names.collect::<Vec<_>>()
            );
        }
        DecisionNode::Branch {
            bit_range,
            children,
        } => {
            println!(
                "{}Branch on bits {}..={}",
                " ".repeat(indent),
                bit_range.1,
                bit_range.0
            );
            for (pattern, child) in children {
                println!("{}Pattern {:b}:", " ".repeat(indent + 2), pattern);
                print_tree(child, indent + 4, opcodes);
            }
        }
    }
}

fn build_match(cur: &DecisionNode, opcodes: &[Vec<String>]) -> TokenStream {
    match cur {
        DecisionNode::Leaf(vec) => {
            let opcode = &opcodes[vec[0]];
            quote! {}
        }
        DecisionNode::Branch {
            bit_range,
            children,
        } => todo!(),
    }
}
