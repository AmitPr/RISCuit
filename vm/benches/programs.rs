use criterion::{criterion_group, criterion_main, Criterion};
use riscv_kernel_linux::MockLinux;
use riscv_vm::machine::Machine;

fn primes_setup() -> Machine<MockLinux> {
    // TODO: Hardcoded path for now.
    const PRIMES_ELF_FILE: &str = "/Users/amit/Documents/projects/derisc/riscv/guest_std/target/riscv32imac-unknown-linux-musl/release/guest_std";
    let elf = std::fs::read(PRIMES_ELF_FILE).expect("Failed to read ELF file");

    let mut machine = Machine::new(MockLinux::new(false));
    machine
        .kernel
        .load_static_elf(&mut machine.hart, &mut machine.mem, &elf, &[], &[]);

    machine
}

fn primes_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("primes");
    group.sample_size(200);
    group.bench_function("primes-guest", |b| {
        b.iter_batched(
            primes_setup,
            |mut machine| {
                machine.run().expect("Failed to run");
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

fn dhrystone_setup() -> Machine<MockLinux> {
    // TODO: Hardcoded path for now.
    const PRIMES_ELF_FILE: &str = "/Users/amit/Documents/projects/derisc/riscv/dhrystone/dhrystone";
    let elf = std::fs::read(PRIMES_ELF_FILE).expect("Failed to read ELF file");

    let mut machine = Machine::new(MockLinux::new(false));
    machine
        .kernel
        .load_static_elf(&mut machine.hart, &mut machine.mem, &elf, &[], &[]);

    machine
}

fn dhrystone_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("dhrystone");
    group.sample_size(200);
    group.bench_function("dhrystone-guest", |b| {
        b.iter_batched(
            dhrystone_setup,
            |mut machine| {
                machine.run().expect("Failed to run");
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

criterion_group!(programs, primes_bench, dhrystone_bench);
criterion_main!(programs);
