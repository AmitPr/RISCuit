use std::error::Error;

use riscv_inst::codegen::rv32::Rv32;
use thiserror::Error;

#[derive(Debug)]
pub enum MemoryAccess {
    Load,
    Store,
}

#[derive(Error, Debug)]
pub enum HartError {
    #[error("Invalid instruction at address {addr:#08x}: {inst:x}")]
    InvalidInstruction { addr: u32, inst: u32 },
    #[error("Illegal instruction \"{op}\" at address {addr:#08x}")]
    IllegalInstruction { addr: u32, op: Rv32 },
    #[error("Unimplemented instruction \"{op}\" at address {addr:#08x}")]
    UnimplementedInstruction { addr: u32, op: Rv32 },
}

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Memory access ({access:?}) at {addr:#08x} is not aligned to {required} bytes")]
    UnalignedMemoryAccess {
        access: MemoryAccess,
        addr: u32,
        required: u32,
    },
    #[error("Memory access ({access:?}) at {addr:#08x} is out of bounds")]
    OutOfBoundsMemoryAccess { access: MemoryAccess, addr: u32 },
    #[error("Memory access ({access:?}) of length {len} at {addr:#08x} would overflow")]
    OverflowMemoryAccess {
        access: MemoryAccess,
        addr: u32,
        len: u32,
    },
}

#[derive(Error, Debug)]
pub enum MachineError<E: Error> {
    #[error("Hart error: {0}")]
    Hart(#[from] HartError),
    #[error("Memory error: {0}")]
    Memory(#[from] MemoryError),
    #[error("Kernel error: {0}")]
    Kernel(E),
}
