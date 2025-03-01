use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use rand::prelude::*;
use riscv_kernel_linux::MockLinux;
use riscv_vm::machine::Machine;

fn decode_setup() -> Vec<u32> {
    let seed = [0; 32];
    let mut rng = SmallRng::from_seed(seed);
    let mut corpus = Vec::new();
    for _ in 0..1_000_000 {
        corpus.push(rng.gen());
    }

    corpus
}

fn decode_bench(c: &mut Criterion) {
    let corpus = decode_setup();
    c.bench_function("decode_rv32imasc", |b| {
        b.iter(|| {
            for &inst in &corpus {
                black_box(riscv_vm::riscv_inst::codegen::rv32imasc::Rv32IMASC::parse(
                    black_box(inst),
                ));
            }
        })
    });
}

fn roundtrip_bench(c: &mut Criterion) {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../riscv/roundtrip/target/riscv32imac-unknown-linux-musl/release/roundtrip");
    let elf = std::fs::read(file).expect("Failed to read ELF file");

    c.bench_function("roundtrip_e2e", |b| {
        b.iter(|| {
            let mut machine = Machine::new(MockLinux::new(false));
            machine.kernel.load_static_elf(
                &mut machine.hart,
                &mut machine.mem,
                black_box(&elf),
                &[],
                &[],
            );
            machine.run().expect("Failed to run");
        })
    });
}

fn roundtrip_setup_bench(c: &mut Criterion) {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../riscv/roundtrip/target/riscv32imac-unknown-linux-musl/release/roundtrip");
    let elf = std::fs::read(file).expect("Failed to read ELF file");

    c.bench_function("roundtrip_load", |b| {
        b.iter(|| {
            let mut machine = Machine::new(MockLinux::new(false));
            machine.kernel.load_static_elf(
                &mut machine.hart,
                &mut machine.mem,
                black_box(&elf),
                &[],
                &[],
            );
        })
    });
}

fn roundtrip_exec_bench(c: &mut Criterion) {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../riscv/roundtrip/target/riscv32imac-unknown-linux-musl/release/roundtrip");
    let elf = std::fs::read(file).expect("Failed to read ELF file");

    c.bench_function("roundtrip_exec", |b| {
        b.iter_batched(
            || {
                let mut machine = Machine::new(MockLinux::new(false));
                machine.kernel.load_static_elf(
                    &mut machine.hart,
                    &mut machine.mem,
                    black_box(&elf),
                    &[],
                    &[],
                );
                machine
            },
            |mut machine| {
                machine.run().expect("Failed to run");
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

criterion_group!(
    name = microbenches;
    config = Criterion::default().with_profiler(PProfProfiler::new(10_000, Output::Flamegraph(None)));
    targets = decode_bench, roundtrip_bench, roundtrip_setup_bench, roundtrip_exec_bench,
);
criterion_main!(microbenches);
