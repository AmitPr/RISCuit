//! Stage-ablation profiler: captures a real execution trace, then times each
//! interpreter pipeline stage separately over that trace so the deltas
//! attribute the per-instruction cost.
//!
//! Stages:
//!   fetch        - mem.load::<u32>(pc) over the traced PCs
//!   decode_seq   - Rv32IMASC::parse(raw) in execution order (predictable)
//!   decode_shuf  - same raws, shuffled (defeats branch prediction, same mix)
//!   fetch+decode - combined, execution order
//!   step_direct  - real Hart32::step in a tight loop (no Machine::run)
//!   run_machine  - Machine::run (adds state check + Result plumbing)
use std::hint::black_box;
use std::time::Instant;

use riscv_kernel_linux::MockLinux;
use riscv_vm::machine::Machine;
use riscv_vm::memory::Memory;
use riscv_vm::riscv_inst::codegen::rv32imasc::Rv32IMASC;

fn tsc_hz() -> f64 {
    let t0 = Instant::now();
    let c0 = unsafe { core::arch::x86_64::_rdtsc() };
    std::thread::sleep(std::time::Duration::from_millis(200));
    let c1 = unsafe { core::arch::x86_64::_rdtsc() };
    (c1 - c0) as f64 / t0.elapsed().as_secs_f64()
}

fn bench<F: FnMut()>(name: &str, n: u64, hz: f64, reps: usize, mut f: F) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..reps {
        let t0 = Instant::now();
        f();
        let dt = t0.elapsed().as_secs_f64();
        best = best.min(dt);
    }
    let ns = best * 1e9 / n as f64;
    let cyc = best * hz / n as f64;
    println!("{name:14} {ns:6.2} ns/inst  {cyc:6.2} cyc/inst");
    ns
}

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("usage: trace_stages <elf> [n_insts]");
    let n: usize = args
        .next()
        .map(|s| s.parse().unwrap())
        .unwrap_or(20_000_000);
    let elf = std::fs::read(&path).expect("Failed to read ELF file");
    let hz = tsc_hz();
    println!("tsc ~{:.2} GHz", hz / 1e9);

    // Capture (pc, raw) trace from a real run.
    let mut machine = Machine::new(MockLinux::new(false));
    machine
        .kernel
        .load_static_elf(&mut machine.hart, &mut machine.mem, &elf, &[], &[]);
    let mut pcs: Vec<u32> = Vec::with_capacity(n);
    let mut raws: Vec<u32> = Vec::with_capacity(n);
    while pcs.len() < n {
        let pc = machine.hart.pc;
        pcs.push(pc);
        raws.push(machine.mem.load::<u32>(pc));
        machine.step().expect("step failed");
        if machine.state == riscv_vm::machine::MachineState::Halted {
            break;
        }
    }
    let n = pcs.len() as u64;
    let compressed = raws.iter().filter(|&&r| r & 0b11 != 0b11).count();
    println!(
        "trace: {n} insts, {:.1}% compressed",
        compressed as f64 / n as f64 * 100.0
    );

    // Stage 0: fetch only (guest code pages still mapped in `machine.mem`).
    let mem = &machine.mem;
    bench("fetch", n, hz, 5, || {
        for &pc in &pcs {
            black_box(mem.load::<u32>(black_box(pc)));
        }
    });

    // Stage 1: decode, execution order.
    bench("decode_seq", n, hz, 5, || {
        for &raw in &raws {
            black_box(Rv32IMASC::parse(black_box(raw)));
        }
    });

    // Stage 1b: decode, shuffled (same instruction mix, no sequence pattern).
    let mut shuffled = raws.clone();
    // Deterministic Fisher-Yates via xorshift; rand crate not needed.
    let mut s = 0x9e3779b9u64;
    for i in (1..shuffled.len()).rev() {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        shuffled.swap(i, (s % (i as u64 + 1)) as usize);
    }
    bench("decode_shuf", n, hz, 5, || {
        for &raw in &shuffled {
            black_box(Rv32IMASC::parse(black_box(raw)));
        }
    });

    // Stage 2: fetch + decode, execution order.
    bench("fetch+decode", n, hz, 5, || {
        for &pc in &pcs {
            black_box(Rv32IMASC::parse(mem.load::<u32>(black_box(pc))));
        }
    });

    // Stage 3: the real thing, Hart32::step in a tight loop.
    let elf2 = elf.clone();
    bench("step_direct", n, hz, 3, || {
        let mut m = Machine::new(MockLinux::new(false));
        m.kernel
            .load_static_elf(&mut m.hart, &mut m.mem, &elf2, &[], &[]);
        for _ in 0..n {
            if m.hart.step(&mut m.mem, &mut m.kernel).is_err() {
                break;
            }
        }
        black_box(m.hart.inst_count);
    });

    // Stage 4: through Machine::run-style stepping (state check per inst).
    bench("machine_step", n, hz, 3, || {
        let mut m = Machine::new(MockLinux::new(false));
        m.kernel
            .load_static_elf(&mut m.hart, &mut m.mem, &elf2, &[], &[]);
        for _ in 0..n {
            if m.step().is_err() || m.state == riscv_vm::machine::MachineState::Halted {
                break;
            }
        }
        black_box(m.hart.inst_count);
    });
}
