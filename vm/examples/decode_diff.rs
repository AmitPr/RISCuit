//! Differential check: flat-table parse must agree with the decision-tree
//! fallback (the previous decoder) across the entire 32-bit encoding space.
use riscv_vm::riscv_inst::codegen::rv32imasc::Rv32IMASC;

fn main() {
    // (table Some, tree Some, different) | (table Some, tree None) |
    // (table None, tree Some) -- only the last is expected: the tree is a
    // classifier that stops checking fixed bits once one candidate remains,
    // so it loosely accepts invalid encodings the table rejects.
    let (mut both_differ, mut table_loose, mut tree_loose) = (0u64, 0u64, 0u64);
    let mut inst: u32 = 0;
    loop {
        if inst & 0b11 == 0b11 {
            let fast = Rv32IMASC::parse(inst);
            let slow = Rv32IMASC::parse_slow(inst);
            match (fast, slow) {
                (Some(f), Some(s)) if f != s => {
                    if both_differ < 5 {
                        println!("DIFFER {inst:#010x}: table={f:?} tree={s:?}");
                    }
                    both_differ += 1;
                }
                (Some(f), None) => {
                    if table_loose < 5 {
                        println!("TABLE-LOOSE {inst:#010x}: table={f:?}");
                    }
                    table_loose += 1;
                }
                (None, Some(_)) => tree_loose += 1,
                _ => {}
            }
        }
        inst = match inst.checked_add(1) {
            Some(i) => i,
            None => break,
        };
    }
    println!("done: both_differ={both_differ} table_loose={table_loose} tree_loose={tree_loose}");
}
