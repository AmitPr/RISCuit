//! Standalone throughput/latency harness: runs a guest ELF N times and
//! reports machine-creation, load, and exec timings plus exact MIPS from
//! the hart's retired-instruction counter.
//!
//! Width-generic: the ELF class byte picks the rv32 or rv64 machine.
use std::time::Instant;

use riscv_kernel_linux::{KernelXlen, MockLinux};
use riscv_vm::hart::{X32, X64};
use riscv_vm::machine::{Kernel, Machine};

fn median(mut xs: Vec<f64>) -> f64 {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    xs[xs.len() / 2]
}

fn bench<X: KernelXlen>(path: &str, elf: &[u8], iters: usize)
where
    MockLinux<X>: Kernel<Xlen = X, Memory = X::Memory>,
{
    let mut new_times = Vec::new();
    let mut load_times = Vec::new();
    let mut exec_times = Vec::new();
    let mut insts = 0u64;
    let mut exit_code = None;

    let profiler = std::env::var("FLAMEGRAPH").ok().map(|out| {
        (
            pprof::ProfilerGuardBuilder::default()
                .frequency(10_000)
                .build()
                .unwrap(),
            out,
        )
    });

    for _ in 0..iters {
        let t0 = Instant::now();
        let mut machine = Machine::new(MockLinux::<X>::new(false));
        let t1 = Instant::now();
        machine
            .kernel
            .load_static_elf(&mut machine.hart, &mut machine.mem, elf, &[], &[]);
        let t2 = Instant::now();
        machine.run().expect("Failed to run");
        let t3 = Instant::now();

        new_times.push(t1.duration_since(t0).as_secs_f64());
        load_times.push(t2.duration_since(t1).as_secs_f64());
        exec_times.push(t3.duration_since(t2).as_secs_f64());
        insts = machine.hart.inst_count;
        exit_code = machine.kernel.exit_code();
    }

    if let Some((guard, out)) = profiler {
        let report = guard.report().build().unwrap();
        let file = std::fs::File::create(&out).unwrap();
        report.flamegraph(file).unwrap();
        eprintln!("flamegraph written to {out}");
    }

    let exec = median(exec_times);
    println!(
        "{path}: exit={exit_code:?} insts={insts} new={:.1}us load={:.1}us exec={:.3}ms mips={:.1}",
        median(new_times) * 1e6,
        median(load_times) * 1e6,
        exec * 1e3,
        insts as f64 / exec / 1e6
    );
}

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("usage: bench_mips <elf> [iters]");
    let iters: usize = args.next().map(|s| s.parse().unwrap()).unwrap_or(5);
    let elf = std::fs::read(&path).expect("Failed to read ELF file");

    match elf.get(4) {
        Some(1) => bench::<X32>(&path, &elf, iters),
        Some(2) => bench::<X64>(&path, &elf, iters),
        other => panic!("unrecognized ELF class: {other:?}"),
    }
}
