//! Dynamic instruction-mix probe: fraction of compressed (16-bit) vs
//! full-width encodings actually executed, per guest. Decode cost differs
//! between the two paths, so mix differences show up as MIPS differences.
use riscv_kernel_linux::{KernelXlen, MockLinux};
use riscv_vm::hart::{X32, X64, Xlen};
use riscv_vm::machine::{Kernel, Machine};
use riscv_vm::memory::Memory;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("usage: imix <elf> [n_insts]");
    let n: u64 = args.next().map(|s| s.parse().unwrap()).unwrap_or(20_000_000);
    let elf = std::fs::read(&path).expect("Failed to read ELF file");

    match elf.get(4) {
        Some(1) => run::<X32>(&path, &elf, n),
        Some(2) => run::<X64>(&path, &elf, n),
        other => panic!("unrecognized ELF class: {other:?}"),
    }
}

fn run<X: KernelXlen>(path: &str, elf: &[u8], n: u64)
where
    MockLinux<X>: Kernel<Xlen = X, Memory = X::Memory>,
{
    let mut m = Machine::new(MockLinux::<X>::new(false));
    m.kernel
        .load_static_elf(&mut m.hart, &mut m.mem, elf, &[], &[]);

    let mut compressed = 0u64;
    let mut executed = 0u64;
    while executed < n && m.state.is_running() {
        let inst: u32 = m.mem.load(m.hart.pc).unwrap();
        if inst & 0b11 != 0b11 {
            compressed += 1;
        }
        executed += 1;
        if m.step().is_err() {
            break;
        }
    }
    println!(
        "{path}: rv{} executed={executed} compressed={:.1}%",
        X::BITS,
        compressed as f64 / executed as f64 * 100.0
    );
}
