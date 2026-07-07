use std::error::Error;

use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum MemoryAccess {
    Load,
    Store,
}

#[derive(Error, Debug)]
pub enum HartError {
    #[error("Invalid instruction at address {addr:#x}: {inst:x}")]
    InvalidInst { addr: u64, inst: u32 },
    #[error("Illegal instruction \"0x{inst:08x}\" at address {addr:#x}")]
    IllegalInst { addr: u64, inst: u32 },
    #[error("Unimplemented instruction \"0x{inst:08x}\" at address {addr:#x}")]
    UnimplementedInst { addr: u64, inst: u32 },
}

impl HartError {
    pub fn invalid(addr: impl Into<u64>, inst: u32) -> Self {
        Self::InvalidInst {
            addr: addr.into(),
            inst,
        }
    }
    pub fn illegal(addr: impl Into<u64>, inst: u32) -> Self {
        Self::IllegalInst {
            addr: addr.into(),
            inst,
        }
    }
    pub fn unimplemented(addr: impl Into<u64>, inst: u32) -> Self {
        Self::UnimplementedInst {
            addr: addr.into(),
            inst,
        }
    }
}

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Memory access ({access:?}) at {addr:#x} is not aligned to {required} bytes")]
    UnalignedMemoryAccess {
        access: MemoryAccess,
        addr: u64,
        required: u32,
    },
    #[error("Memory access ({access:?}) at {addr:#x} is outside the guest address space")]
    Fault { access: MemoryAccess, addr: u64 },
    #[error("Memory access ({access:?}) of length {len} at {addr:#x} would overflow")]
    OverflowMemoryAccess {
        access: MemoryAccess,
        addr: u64,
        /// Saturated for diagnostics; keeps the error (and every Result
        /// carrying it) within a two-register return.
        len: u32,
    },
}

/// Hart/memory payloads are boxed to keep this (and every hot-path
/// `Result` carrying it) within a two-register return; error construction
/// is cold.
#[derive(Error, Debug)]
pub enum MachineError<E: Error> {
    #[error("Hart error: {0}")]
    Hart(Box<HartError>),
    #[error("Memory error: {0}")]
    Memory(Box<MemoryError>),
    #[error("Kernel error: {0}")]
    Kernel(E),
}

impl<E: Error> From<HartError> for MachineError<E> {
    #[cold]
    fn from(e: HartError) -> Self {
        Self::Hart(Box::new(e))
    }
}

impl<E: Error> From<MemoryError> for MachineError<E> {
    #[cold]
    fn from(e: MemoryError) -> Self {
        Self::Memory(Box::new(e))
    }
}
