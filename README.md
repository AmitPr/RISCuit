# RISCuit

a miniature risc-v userspace emulator/sandbox

designed with a focus on low latency, high performance, and simplicity, in that order.

## codebase

- `vm/` - an example consumer of the `riscuit` libraries.
- `crates/`
  - `riscv-vm` - the r5 processor, memory, and "machine" implementations/abstractions.
  - `riscv-inst` - r5 ISA definitions. mostly codegen.
  - `riscv-inst-codegen` - codegen for the r5 ISA.
  - `riscv-kernel-linux` - a kernel (running outside the guest) that provides a subset of the linux syscall interface.
  - `libc-riscv32` - libc constants from riscv32 header files.
- `riscv/` - toolchains and various guest programs (with binaries).
  - `guest_c` - simple guest C programs.
  - `guest_std` - simple guest Rust programs, with `std` support.
  - `roundtrip` - a guest used for benchmarking the latency of a minimal `return 0` program.
  - `test-env` - dockerfile that builds the `riscv-tests` suite.
  - `toolchain` - target specification and dockerized toolchain for building guest programs.
- `test/` - running the `riscv-tests` suite.
  - `riscv-tests` - codegen to run each test in the `riscv-tests` suite as a separate test.
  - `riscv-tests-codegen` - codegen for each `riscv-tests` test.

## building and running

```bash
cargo run --bin riscv-inst-codegen # generate the ISA
cargo run --bin riscv-tests-codegen # generate the tests
cargo test # run tests
cargo run --bin riscuit # run the vm
```
