use riscv_inst::codegen::rv32::Rv32;
use thiserror::Error;

#[derive(Debug)]
pub enum MemoryAccess {
    Load,
    Store,
    Swap,
}

#[derive(Error, Debug)]
pub enum VmError {
    #[error("Invalid instruction at address {addr:#08x}: {inst:x}")]
    InvalidInstruction { addr: u32, inst: u32 },
    #[error("Illegal instruction \"{op}\" at address {addr:#08x}")]
    IllegalInstruction { addr: u32, op: Rv32 },
    #[error("Unimplemented instruction \"{op}\" at address {addr:#08x}")]
    UnimplementedInstruction { addr: u32, op: Rv32 },
    #[error("Memory access ({access:?}) at {addr:#08x} is not aligned to {required} bytes")]
    UnalignedMemoryAccess {
        access: MemoryAccess,
        addr: u32,
        required: u32,
    },
}
