use std::error::Error;

use thiserror::Error;

#[derive(Debug)]
pub enum MemoryAccess {
    Load,
    Store,
}

#[derive(Error, Debug)]
pub enum HartError {
    #[error("Invalid instruction at address {addr:#08x}: {inst:x}")]
    InvalidInst { addr: u32, inst: u32 },
    #[error("Illegal instruction \"0x{inst:08x}\" at address {addr:#08x}")]
    IllegalInst { addr: u32, inst: u32 },
    #[error("Unimplemented instruction \"0x{inst:08x}\" at address {addr:#08x}")]
    UnimplementedInst { addr: u32, inst: u32 },
}

impl HartError {
    pub const fn invalid(addr: u32, inst: u32) -> Self {
        Self::InvalidInst { addr, inst }
    }
    pub const fn illegal(addr: u32, inst: u32) -> Self {
        Self::IllegalInst { addr, inst }
    }
    pub const fn unimplemented(addr: u32, inst: u32) -> Self {
        Self::UnimplementedInst { addr, inst }
    }
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
