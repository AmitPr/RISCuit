//! Harts, generic over register width.
//!
//! [`Xlen`] is the width marker ([`X32`]/[`X64`]): it fixes the register
//! type and supplies the few width conversions kernels need. [`Hart<X>`] is
//! the width-generic CPU state. Instruction semantics live in per-base-ISA
//! exec modules ([`exec32`]/[`exec64`]) -- the decoded enum types differ per
//! ISA, so each base gets its own match, with arms grouped by extension.
//! [`Execute`] wires the right exec into the width so `Machine` stays fully
//! generic.

mod exec32;
mod exec64;

use std::fmt;

use crate::{
    error::{MachineError, MemoryAccess, MemoryError},
    machine::{Kernel, StepResult},
    memory::Primitive,
};

/// Attach address/access context to a zero-sized [`crate::memory::Fault`].
#[cold]
#[inline(never)]
pub(crate) fn mem_fault<E: std::error::Error>(access: MemoryAccess, addr: u64) -> MachineError<E> {
    MemoryError::Fault { access, addr }.into()
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::X32 {}
    impl Sealed for super::X64 {}
}

/// Register width marker. See [`X32`] and [`X64`].
pub trait Xlen: sealed::Sealed + Sized + 'static {
    /// The register type (`u32`/`u64`).
    type U: Primitive
        + Copy
        + Default
        + PartialEq
        + Eq
        + fmt::Debug
        + fmt::Display
        + fmt::LowerHex
        + Into<u64>
        + Send
        + Sync
        + 'static;

    const BITS: u32;

    /// Truncate.
    fn from_u64(v: u64) -> Self::U;
    /// Sign-truncate (e.g. errno returns: `-1` becomes all-ones at width).
    fn from_i64(v: i64) -> Self::U;
    /// Zero-extend.
    fn to_u64(v: Self::U) -> u64 {
        v.into()
    }
    /// Sign-extend.
    fn to_i64(v: Self::U) -> i64;
}

pub enum X32 {}
impl Xlen for X32 {
    type U = u32;
    const BITS: u32 = 32;
    fn from_u64(v: u64) -> u32 {
        v as u32
    }
    fn from_i64(v: i64) -> u32 {
        v as u32
    }
    fn to_i64(v: u32) -> i64 {
        v as i32 as i64
    }
}

pub enum X64 {}
impl Xlen for X64 {
    type U = u64;
    const BITS: u32 = 64;
    fn from_u64(v: u64) -> u64 {
        v
    }
    fn from_i64(v: i64) -> u64 {
        v as u64
    }
    fn to_i64(v: u64) -> i64 {
        v as i64
    }
}

/// A single hardware thread: register file, pc, CSRs.
#[repr(C)]
pub struct Hart<X: Xlen> {
    // Hot fields first; the CSR file is cold and lives at the end.
    regs: [X::U; 32],
    pub pc: X::U,
    pub inst_count: u64,
    /// Atomic memory reservation set on this hart
    pub amo_rsv: Option<X::U>,
    csrs: [X::U; 4096],
}

pub type Hart32 = Hart<X32>;
pub type Hart64 = Hart<X64>;

use riscv_inst::Reg;

impl<X: Xlen> Hart<X> {
    pub fn new() -> Self {
        Hart {
            regs: [X::U::default(); 32],
            pc: X::U::default(),
            inst_count: 0,
            amo_rsv: None,
            csrs: [X::U::default(); 4096],
        }
    }

    /// Read a register. x0 is always 0 in RISC-V.
    #[inline(always)]
    pub fn get_reg(&self, r: Reg) -> X::U {
        self.regs[r as usize]
    }

    /// Write a register. x0 is re-zeroed after the write to avoid a select.
    #[inline(always)]
    pub fn set_reg(&mut self, r: Reg, val: X::U) {
        self.regs[r as usize] = val;
        self.regs[0] = X::U::default();
    }

    /// Dump registers to stdout
    pub fn regs(&self) -> impl Iterator<Item = (Reg, X::U)> + use<'_, X> {
        (0..32).map(|i| (unsafe { Reg::from_u5(i as u8) }, self.regs[i]))
    }

    pub fn regs_range(
        &self,
        start: Reg,
        end: Reg,
    ) -> impl Iterator<Item = (Reg, X::U)> + use<'_, X> {
        (start as usize..=end as usize).map(|i| (unsafe { Reg::from_u5(i as u8) }, self.regs[i]))
    }
}

impl<X: Execute> Hart<X> {
    /// Execute a single instruction.
    pub fn step<K: Kernel<Xlen = X>>(
        &mut self,
        mem: &mut K::Memory,
        kernel: &mut K,
    ) -> Result<StepResult, MachineError<K::Error>> {
        X::step(self, mem, kernel)
    }

    /// Run until the kernel halts the machine or an error occurs.
    pub fn run<K: Kernel<Xlen = X>>(
        &mut self,
        mem: &mut K::Memory,
        kernel: &mut K,
    ) -> Result<(), MachineError<K::Error>> {
        X::run(self, mem, kernel)
    }
}

impl<X: Xlen> Default for Hart<X> {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction execution for a width: implemented per base ISA in the exec
/// modules, dispatched statically through `Kernel::Xlen`.
pub trait Execute: Xlen {
    fn run<K: Kernel<Xlen = Self>>(
        hart: &mut Hart<Self>,
        mem: &mut K::Memory,
        kernel: &mut K,
    ) -> Result<(), MachineError<K::Error>>;

    fn step<K: Kernel<Xlen = Self>>(
        hart: &mut Hart<Self>,
        mem: &mut K::Memory,
        kernel: &mut K,
    ) -> Result<StepResult, MachineError<K::Error>>;
}

/// Outcome of executing one instruction.
///
/// Kernel entry (ecall/ebreak) is signalled rather than handled so callers
/// can flush register-held state (e.g. the instruction count) first; pc is
/// not advanced for those variants.
///
/// Deliberately payload-free: the next pc travels through the `&mut` pc
/// out-parameter of `exec_op_at`, and errors through the `&mut` error slot,
/// instead of riding in enum/`Result` payloads. A payload-carrying return
/// makes LLVM pack tag+payload into a two-register aggregate at every arm's
/// join point and unpack it at the loop head, putting a shift/test/shift
/// chain on the loop-carried pc dependence (measured ~12% interpreter
/// throughput).
pub(crate) enum Exec {
    /// Continue at the pc written through the out-parameter.
    Next,
    /// An ecall; invoke [`Kernel::syscall`]. pc is not advanced.
    Syscall,
    /// An ebreak / c.ebreak; invoke [`Kernel::ebreak`]. pc is not advanced.
    Ebreak,
    /// Execution faulted; the error is in the caller's error slot.
    Error,
}

/// Take the error deposited by an [`Exec::Error`] return. Cold: only ever
/// reached on the (terminal) error path.
#[cold]
#[inline(never)]
pub(crate) fn take_err<E: std::error::Error>(
    slot: &mut Option<MachineError<E>>,
) -> MachineError<E> {
    match slot.take() {
        Some(e) => e,
        // exec_op_at fills the slot before returning Exec::Error.
        None => unreachable!("Exec::Error without a deposited error"),
    }
}
