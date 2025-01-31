use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

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

criterion_group!(microbenches, decode_bench);
criterion_main!(microbenches);
