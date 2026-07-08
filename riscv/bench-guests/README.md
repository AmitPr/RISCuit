# bench-guests

Freestanding C benchmark guests (no libc; direct ecall syscalls) buildable
with stock clang+lld for both rv32imac and rv64imac. Used by
`vm/examples/bench_mips` (ELF class picks the machine width).

| guest | exercises | expected exit |
|---|---|---|
| exit0 | cold-start roundtrip | 0 |
| alu | u32 xorshift (W-ops on rv64) | 141 |
| branch | collatz, data-driven branches | 117 |
| call | recursive fib | 5 |
| mem | 1 MiB strided read-modify-write | 0 |
| muldiv | mul/div/rem chains | 60 |
| syscall | 1M getpid ecalls | 64 |
| amo | AMO rd==rs2 hazard regression | 42 |
| alu64 | native u64 xorshift (rv64-only meaningful) | 145 |
