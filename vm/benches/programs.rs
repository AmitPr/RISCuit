use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};
use riscv_kernel_linux::MockLinux32;
use riscv_vm::machine::Machine;

fn program_setup(elf: &[u8]) -> Machine<MockLinux32> {
    let mut machine = Machine::new(MockLinux32::new(false));
    machine
        .kernel
        .load_static_elf(&mut machine.hart, &mut machine.mem, elf, &[], &[]);

    machine
}

fn program_bench(c: &mut Criterion, group_name: &str, bench_name: &str, rel_path: &str) {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path);
    let Ok(elf) = std::fs::read(&file) else {
        eprintln!("skipping {bench_name}: {} not built", file.display());
        return;
    };

    let mut group = c.benchmark_group(group_name);
    group.sample_size(200);
    group.bench_function(bench_name, |b| {
        b.iter_batched(
            || program_setup(&elf),
            |mut machine| {
                machine.run().expect("Failed to run");
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

fn primes_bench(c: &mut Criterion) {
    program_bench(
        c,
        "primes",
        "primes-guest",
        "../riscv/guest_std/target/riscv32imac-unknown-linux-musl/release/guest_std",
    );
}

fn dhrystone_bench(c: &mut Criterion) {
    program_bench(
        c,
        "dhrystone",
        "dhrystone-guest",
        "../riscv/dhrystone/dhrystone",
    );
}

criterion_group!(programs, primes_bench, dhrystone_bench);
criterion_main!(programs);
