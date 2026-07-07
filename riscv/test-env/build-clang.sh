#!/usr/bin/env bash
# Build the riscv-tests ISA suites (rv32/rv64 x ui/um/ua/uc, `p` environment)
# with clang + lld -- no riscv-gnu-toolchain needed. Artifacts land in
# riscv/test-env/artifacts for test/riscv-test-codegen to pick up.
#
# Usage: ./build-clang.sh [path-to-riscv-tests-checkout]
# (clones https://github.com/riscv/riscv-tests + its env submodule if absent)
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
src="${1:-$here/riscv-tests}"
artifacts="$here/artifacts"

if [ ! -d "$src" ]; then
    git clone --depth 1 https://github.com/riscv/riscv-tests "$src"
    git -C "$src" submodule update --init --depth 1 env
fi

mkdir -p "$artifacts"
cd "$src"

built=0
for w in 32 64; do
    if [ "$w" = 32 ]; then tgt=riscv32-unknown-elf abi=ilp32; else tgt=riscv64-unknown-elf abi=lp64; fi
    for ext in ui um ua uc; do
        march="rv${w}g"
        [ "$ext" = uc ] && march="rv${w}gc"
        for f in "isa/rv${w}${ext}"/*.S; do
            t="$(basename "$f" .S)"
            clang --target="$tgt" -march="$march" -mabi="$abi" -mcmodel=medany \
                -static -fvisibility=hidden -nostdlib -nostartfiles -fuse-ld=lld \
                -I env/p -I isa/macros/scalar -T env/p/link.ld \
                -o "$artifacts/rv${w}${ext}-p-$t" "$f"
            built=$((built + 1))
        done
    done
done
echo "built $built test binaries into $artifacts"
