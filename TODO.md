# TODOs and exploration avenues

Serialized backlog from the profiling/optimization/rv64 work. Items marked
**(pre-existing)** were inherited and are known-but-unfixed; everything else
came out of measurements or review in that work.

## Performance — measured and ranked

1. **Predecode / basic-block cache** — the big one, deliberately deferred.
   The interpreter runs ~9.4 cyc/inst (rv32) and ~7 of those are the
   irreducible `fetch load -> decode-table load -> dispatch -> next_pc`
   chain that predecoding removes. Every other lever is exhausted:
   measured wins so far were all "move loop-carried state from memory to
   registers" (fused loop +50%, pc-in-reg, local inst_count +10%,
   payload-free join +12%). Expected 1.5-3x; template JIT beyond that
   (qemu TCG is currently ~5x ahead on compute).
2. ~~**Memory64 limit check -> MMU-enforced faults.**~~ Resolved without
   MMU tricks: the check's real cost was the per-access reload of the
   arena base and `mapped` (aliasing pessimism), not the predicted
   branch. `MemView` keeps both in registers across the loop and mem64
   now runs at or above the old no-check ablation. The PROT_NONE/SIGSEGV
   design (reserve the configured max, `mprotect` on grow, signal ->
   guest fault) stays documented as the fallback if a workload ever
   surfaces the remaining ~2 inst/access, but it trades reserved VA per
   hart and process-global signal state for a cost that no longer
   measures.
3. **RV64 W-instruction sext overhead** — measured +1.75 host inst/guest
   inst on identical instruction counts (alu: 588M vs 616M host insts per
   16M guest insts); explains the alu-class rv64 gap. Inherent to
   interpretation (hardware sign-extends in writeback for free); a JIT
   folds it. Not worth chasing in the interpreter.
4. **Compressed-density difference** on rv64 (several W/shift forms have no
   C encoding): branch 58.8% -> 47.1% compressed, muldiv 58.8% -> 42.1%.
   The 32-bit decode path costs more than C_LOOKUP (measured ~10% via
   no-C rv32 builds). Nothing to do directly; predecoding erases it.
5. **PGO**: measured +4-7% across all guests (train on bench guests,
   `llvm-profdata` from the rustup toolchain, not system LLVM). Deemed not
   worth the pipeline complexity for now.
6. `run_for(fuel)` **budgeted execution** — the density/interleave probes
   run thousands of harts via `step()` (~2x slower than `run()`); a fueled
   run loop is also the scheduling primitive a many-hart host needs.
7. Loader: `__DATA_BEGIN__` lookup iterates the full symtab (O(symbols)
   with strtab reads); make it lazy or drop it (TODO in code asks whether
   libc needs gp at all) (pre-existing).
8. Codegen `EXHAUSTIVE_MATCH_THRESHOLD` puzzle: manual `unreachable!()`
   arms measured slower than expected; pinned to 0 (pre-existing).
9. **Code-layout roulette dominates the remaining micro-margins** on
   Skylake-family hosts: per-guest wall MIPS swings +-5-20% between
   builds from block placement alone (one hot arm's alignment can cost a
   leg 20% with an identical instruction stream). The JCC-erratum padding
   flag in `.cargo/config.toml` removes the worst 2x cliffs;
   block-alignment hammers (`-align-all-nofallthru-blocks`) and
   codegen-units/LTO changes just reroll the dice. Treat cachegrind
   retired-instruction counts as the stable metric; PGO or predecoding
   (fewer, bigger blocks) are the durable fixes if per-leg wall variance
   ever matters.
10. Negative results, documented so they aren't re-tried blindly:
    `become`-threaded dispatch (nightly TCO works; ~40% slower than the
    fused loop on loop-heavy guests — single dispatch site is already
    well-predicted; may still win on large irregular code, e.g. dhrystone),
    branchless unified decode table (-25%: predicted branch beats a
    serializing cmov), a sentinel-dispatch back-edge (`Slow -> parse_slow
    -> re-dispatch` loop makes LLVM jump-thread speculated tree decode
    into the hot loop head, 4x regression; the cold out-of-line
    `exec_slow` shape is the one that works), speculative next-fetch
    (neutral: OoO already does it), `-C target-cpu=native` (neutral
    before the explicit-`pext` decode index existed).

## Correctness / robustness

- **Fix `futex` properly** (pre-existing): WAIT currently zeroes the futex
  word as a "temporary fix" for a Rust-runtime exit deadlock; WAKE is a
  no-op. Blocks real threading support.
- **glibc guests hang** (pre-existing, root-caused): missing auxv entries
  (`AT_RANDOM` at minimum, also `AT_PHDR`/`AT_SECURE`) and `prlimit64`.
  The committed `guest_c/main` (glibc-static) spins after prlimit64
  returns ENOSYS. Fill the aux vector (TODO already in loader) and add
  prlimit64.
- **`getrandom` returns zeros** (pre-existing): wire a real RNG; related
  to `AT_RANDOM` above (stack canaries are currently all-zero).
- **Loader panics on malformed ELF**: bounds are now checked but still
  `expect()` — a hostile ELF aborts the host process. Make
  `load_static_elf` return `Result`.
- **Privilege levels** (pre-existing): `mret` is a no-op for the ISA
  tests; `sret`/`wfi`/`sfence` error. Needed for the riscv-tests `-v`
  (virtual memory) suites and any trap-handling guests.
- **LR/SC reservation set** (pre-existing): current model is
  single-hart-approximate; revisit if multi-hart ever lands (also:
  `Memory::slice` hands out `&[T]` over guest memory — fine
  single-threaded, needs an aliasing story for threads).
- riscv-tests gap: upstream has no AMO rd==rs2 coverage (which is how that
  bug survived); the `amo` bench guest covers it — consider a proper unit
  test crate for spec corner cases (jalr bit-0, c.jalr ra, div overflow).
- Extend `decode_diff` (table-vs-tree differential over all 2^32
  encodings) to rv64imasc; currently only rv32 is exhaustively checked.
- Boundary semantics note: an unaligned access ending exactly at the arena
  top reads guard-page zeros instead of faulting (same at rv32's 4 GiB
  edge). Accepted; document if it ever matters.
- `bytes_null_terminated` with `max_len: None` scans to the arena cap;
  kernel callers should bound path lengths.

## Architecture / features

- **F/D floating point**: riscv-meta specs and codegen plumbing already
  exist (commented-out ISA combos incl. FReg operands); needs an f-register
  file on `Hart<X>`, exec arms, and a softfloat-vs-host-float decision.
- **RV128**: `Xlen` accommodates it shape-wise (`U = u128`), but riscv-meta
  has no rv128 opcodes and `Memory` usize conversions assume <= 64-bit.
- **Per-extension exec composition**: if base x extension combos
  proliferate, move arm bodies into per-extension modules of
  `#[inline(always)]` fns shared across widths (or teach the codegen to
  emit the exec skeleton). Two hand-written exec files is right for two
  ISAs; wrong for ten.
- **Dynamic linking / PIE guests**: the pre-existing WIP direction
  (guest_dylib crate, target-spec `dynamic-linking: true`) — untouched by
  this work; the auxv work above (`AT_BASE`, `AT_PHDR`) is its
  prerequisite.
- **rv64 musl toolchain**: the buildroot config is rv32-only. An rv64
  variant unlocks std/dhrystone/primes guests on rv64 (and the criterion
  `programs` bench on both widths — it currently skips when guests are
  absent).
- **Snapshot / machine pooling**: cold start is ~10-16us e2e; a serverless
  embedding eventually wants clone/restore (nothing exists yet; the elastic
  arena makes CoW-clone via `MAP_PRIVATE` re-mapping plausible).
- **Width-generic debugger**: the `riscuit` binary's debugger is pinned to
  `MockLinux32`; generalize like `bench_mips` (ELF class dispatch).
- **Kernel breadth**: `clock_gettime`, real `mmap` prot handling,
  fd/filesystem sandbox layer — driven by whatever guests matter next.

## Tooling

- `riscv/test-env/build-clang.sh` builds all 148 riscv-tests binaries in
  seconds (vs the hours-long riscv-gnu-toolchain Docker build); consider
  retiring or slimming the Dockerfile.
- `riscv/bench-guests/` holds the benchmark guest sources + build script;
  `vm/examples/{bench_mips,density,imix}` pick machine width from the ELF
  class. `trace_stages`/`interleave` are still rv32-pinned diagnostics.
