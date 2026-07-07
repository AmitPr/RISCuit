//! Latency-vs-throughput probe: steps N independent machines round-robin.
//! If the interpreter is bound by the per-instruction dependency chain
//! (pc -> fetch -> decode -> dispatch -> next_pc), interleaving independent
//! chains should raise aggregate MIPS nearly linearly until the core's
//! execution width saturates.
use std::time::Instant;

use riscv_kernel_linux::MockLinux32;
use riscv_vm::machine::Machine;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("usage: interleave <elf> [n_steps]");
    let n: u64 = args.next().map(|s| s.parse().unwrap()).unwrap_or(20_000_000);
    let elf = std::fs::read(&path).expect("Failed to read ELF file");

    let mk = || {
        let mut m = Machine::new(MockLinux32::new(false));
        m.kernel
            .load_static_elf(&mut m.hart, &mut m.mem, &elf, &[], &[]);
        m
    };

    for lanes in [1usize, 2, 4, 8] {
        let mut ms: Vec<_> = (0..lanes).map(|_| mk()).collect();
        let t0 = Instant::now();
        for _ in 0..n {
            for m in &mut ms {
                let _ = m.hart.step(&mut m.mem, &mut m.kernel);
            }
        }
        let dt = t0.elapsed().as_secs_f64();
        let total = n * lanes as u64;
        println!(
            "lanes={lanes}: {:.2} ns/inst aggregate, {:.1} MIPS total, {:.1} MIPS/lane",
            dt * 1e9 / total as f64,
            total as f64 / dt / 1e6,
            n as f64 / dt / 1e6,
        );
    }
}
