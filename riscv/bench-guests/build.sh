#!/usr/bin/env bash
# Build the freestanding benchmark guests for rv32 and rv64 with clang+lld.
# No libc needed: guests syscall directly (see common.h).
set -euo pipefail
cd "$(dirname "$0")"
mkdir -p bin
for f in exit0 alu branch call mem muldiv syscall amo alu64; do
    clang --target=riscv32-unknown-elf -march=rv32imac -mabi=ilp32 -O2 \
        -nostdlib -fuse-ld=lld -Wl,-e,_start -Wl,--image-base=0x10000 \
        -o "bin/$f-rv32" "$f.c"
    clang --target=riscv64-unknown-elf -march=rv64imac -mabi=lp64 -O2 \
        -nostdlib -fuse-ld=lld -Wl,-e,_start -Wl,--image-base=0x10000 \
        -o "bin/$f-rv64" "$f.c"
done
echo "built $(ls bin | wc -l) guests into $(pwd)/bin"
