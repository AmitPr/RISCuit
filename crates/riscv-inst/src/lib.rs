mod traits;

#[cfg(test)]
mod test;

use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug, PartialEq, Eq)]
pub enum LoadSize {
    IByte,
    IHalf,
    IWord,
    UByte,
    UHalf,
    UWord,
}

impl LoadSize {
    pub const fn from(funct3: u8) -> Option<Self> {
        match funct3 {
            0b000 => Some(LoadSize::IByte),
            0b001 => Some(LoadSize::IHalf),
            0b010 => Some(LoadSize::IWord),
            0b100 => Some(LoadSize::UByte),
            0b101 => Some(LoadSize::UHalf),
            0b110 => Some(LoadSize::UWord),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StoreSize {
    Byte,
    Half,
    Word,
}

impl StoreSize {
    pub const fn from(funct3: u8) -> Option<Self> {
        match funct3 {
            0b000 => Some(StoreSize::Byte),
            0b001 => Some(StoreSize::Half),
            0b010 => Some(StoreSize::Word),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IntRegOp {
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,
    #[cfg(feature = "rv32m")]
    Mul,
    #[cfg(feature = "rv32m")]
    Mulh,
    #[cfg(feature = "rv32m")]
    Mulhsu,
    #[cfg(feature = "rv32m")]
    Mulu,
    #[cfg(feature = "rv32m")]
    Div,
    #[cfg(feature = "rv32m")]
    Divu,
    #[cfg(feature = "rv32m")]
    Rem,
    #[cfg(feature = "rv32m")]
    Remu,
}

impl IntRegOp {
    pub const fn from(funct3: u8, funct7: u8) -> Option<Self> {
        match (funct3, funct7) {
            (0b000, 0b0000000) => Some(IntRegOp::Add),
            (0b000, 0b0100000) => Some(IntRegOp::Sub),
            (0b001, 0b0000000) => Some(IntRegOp::Sll),
            (0b010, 0b0000000) => Some(IntRegOp::Slt),
            (0b011, 0b0000000) => Some(IntRegOp::Sltu),
            (0b100, 0b0000000) => Some(IntRegOp::Xor),
            (0b101, 0b0000000) => Some(IntRegOp::Srl),
            (0b101, 0b0100000) => Some(IntRegOp::Sra),
            (0b110, 0b0000000) => Some(IntRegOp::Or),
            (0b111, 0b0000000) => Some(IntRegOp::And),
            #[cfg(feature = "rv32m")]
            (0b000, 0b0000001) => Some(IntRegOp::Mul),
            #[cfg(feature = "rv32m")]
            (0b001, 0b0000001) => Some(IntRegOp::Mulh),
            #[cfg(feature = "rv32m")]
            (0b010, 0b0000001) => Some(IntRegOp::Mulhsu),
            #[cfg(feature = "rv32m")]
            (0b011, 0b0000001) => Some(IntRegOp::Mulu),
            #[cfg(feature = "rv32m")]
            (0b100, 0b0000001) => Some(IntRegOp::Div),
            #[cfg(feature = "rv32m")]
            (0b101, 0b0000001) => Some(IntRegOp::Divu),
            #[cfg(feature = "rv32m")]
            (0b110, 0b0000001) => Some(IntRegOp::Rem),
            #[cfg(feature = "rv32m")]
            (0b111, 0b0000001) => Some(IntRegOp::Remu),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IntImmOp {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
}

impl IntImmOp {
    pub const fn from(funct3: u8, funct7: u8) -> Option<Self> {
        match (funct3, funct7) {
            (0b000, _) => Some(IntImmOp::Addi),
            (0b010, _) => Some(IntImmOp::Slti),
            (0b011, _) => Some(IntImmOp::Sltiu),
            (0b100, _) => Some(IntImmOp::Xori),
            (0b110, _) => Some(IntImmOp::Ori),
            (0b111, _) => Some(IntImmOp::Andi),
            (0b001, 0b0000000) => Some(IntImmOp::Slli),
            (0b101, 0b0000000) => Some(IntImmOp::Srli),
            (0b101, 0b0100000) => Some(IntImmOp::Srai),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BranchOp {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

impl BranchOp {
    pub const fn from(funct3: u8) -> Option<Self> {
        match funct3 {
            0b000 => Some(BranchOp::Beq),
            0b001 => Some(BranchOp::Bne),
            0b100 => Some(BranchOp::Blt),
            0b101 => Some(BranchOp::Bge),
            0b110 => Some(BranchOp::Bltu),
            0b111 => Some(BranchOp::Bgeu),
            _ => None,
        }
    }
}

pub struct RType<T>(pub u32, PhantomData<T>);
impl<T> RType<T> {
    pub const fn rd(&self) -> u32 {
        (self.0 >> 7) & 0b1_1111
    }

    pub const fn rs1(&self) -> u32 {
        (self.0 >> 15) & 0b1_1111
    }

    pub const fn rs2(&self) -> u32 {
        (self.0 >> 20) & 0b1_1111
    }

    pub const fn funct7(&self) -> u8 {
        ((self.0 >> 25) & 0b111_1111) as u8
    }
}

impl RType<IntRegOp> {
    pub const fn op(&self) -> Option<IntRegOp> {
        IntRegOp::from(((self.0 >> 12) & 0b111) as u8, self.funct7())
    }
}

pub struct IType<T>(pub u32, PhantomData<T>);
pub struct JalrInst;
pub struct EcallInst;

impl<T> IType<T> {
    pub const fn rd(&self) -> u32 {
        (self.0 >> 7) & 0b1_1111
    }

    pub const fn rs1(&self) -> u32 {
        (self.0 >> 15) & 0b1_1111
    }

    pub const fn signed_imm(&self) -> i32 {
        self.0 as i32 >> 20
    }

    pub const fn unsigned_imm(&self) -> u32 {
        self.0 >> 20
    }

    pub const fn shamt(&self) -> u32 {
        (self.0 >> 20) & 0b1_1111
    }

    pub const fn funct7(&self) -> u8 {
        ((self.0 >> 25) & 0b111_1111) as u8
    }
}

impl IType<EcallInst> {
    pub const fn is_ebreak(&self) -> bool {
        self.funct7() == 1
    }

    pub const fn ecall_code(&self) -> u32 {
        self.unsigned_imm()
    }
}

impl IType<IntImmOp> {
    pub const fn op(&self) -> Option<IntImmOp> {
        IntImmOp::from(((self.0 >> 12) & 0b111) as u8, self.funct7())
    }
}

pub struct SType<T, U>(pub u32, PhantomData<(T, U)>);
pub struct SImm;
pub struct BImm;
pub struct LImm;

impl<T, U> SType<T, U> {
    pub const fn rs1(&self) -> u32 {
        (self.0 >> 15) & 0b1_1111
    }

    pub const fn rs2(&self) -> u32 {
        (self.0 >> 20) & 0b1_1111
    }
}

impl<T> SType<T, SImm> {
    pub const fn signed_imm(&self) -> i32 {
        let hi = (self.0 as i32) >> 25;
        let lo = (self.0 as i32 >> 7) & 0b1_1111;
        (hi << 5) | lo
    }
}

impl<T> SType<T, BImm> {
    pub const fn signed_imm(&self) -> i32 {
        let hi = ((self.0 & 0x8000_0000) as i32 >> 19) as u32;
        (hi | ((self.0 & 0x7e00_0000) >> 20)
            | ((self.0 & 0x0000_0f00) >> 7)
            | ((self.0 & 0x0000_0080) << 4)) as i32
    }
}

impl<T> SType<T, LImm> {
    pub const fn signed_imm(&self) -> i32 {
        self.0 as i32 >> 20
    }

    pub const fn rd(&self) -> u32 {
        (self.0 >> 7) & 0b1_1111
    }
}

impl SType<LoadSize, LImm> {
    pub const fn op(&self) -> Option<LoadSize> {
        LoadSize::from(((self.0 >> 12) & 0b111) as u8)
    }
}

impl SType<StoreSize, SImm> {
    pub const fn op(&self) -> Option<StoreSize> {
        StoreSize::from(((self.0 >> 12) & 0b111) as u8)
    }
}

impl SType<BranchOp, BImm> {
    pub const fn op(&self) -> Option<BranchOp> {
        BranchOp::from(((self.0 >> 12) & 0b111) as u8)
    }
}

pub struct UType<T>(pub u32, PhantomData<T>);
pub struct UImm;
pub struct JImm;
pub struct UpperImm;

impl<T> UType<T> {
    pub const fn rd(&self) -> u32 {
        (self.0 >> 7) & 0b1_1111
    }
}

impl UType<UImm> {
    pub const fn unsigned_imm(&self) -> u32 {
        self.0 >> 12
    }

    pub const fn signed_imm(&self) -> i32 {
        self.0 as i32 >> 12
    }
}

impl UType<JImm> {
    pub const fn signed_imm(&self) -> i32 {
        let hi = ((self.0 & 0x8000_0000) as i32 >> 11) as u32;
        (hi | ((self.0 & 0x7fe0_0000) >> 20)
            | ((self.0 & 0x0010_0000) >> 9)
            | (self.0 & 0x000f_f000)) as i32
    }
}

impl UType<UpperImm> {
    pub const fn unsigned_imm(&self) -> u32 {
        self.0 & 0xFFFFF000
    }

    pub const fn signed_imm(&self) -> i32 {
        (self.0 & 0xFFFFF000) as i32
    }
}

#[derive(Debug)]
pub enum Instruction {
    IntImm(IType<IntImmOp>),
    Lui(UType<UpperImm>),
    Auipc(UType<UpperImm>),
    IntReg(RType<IntRegOp>),
    Jal(UType<JImm>),
    Jalr(IType<JalrInst>),
    Branch(SType<BranchOp, BImm>),
    Load(SType<LoadSize, LImm>),
    Store(SType<StoreSize, SImm>),
    // TODO: Fence
    Fence(UType<UImm>),
    Ecall(IType<EcallInst>),
}

impl Instruction {
    pub fn decode(inst: u32) -> Option<Self> {
        let opcode = inst & 0b1111111;
        match opcode {
            0b000_0011 => Some(Instruction::Load(SType(inst, PhantomData))),
            0b010_0011 => Some(Instruction::Store(SType(inst, PhantomData))),
            0b011_0011 => Some(Instruction::IntReg(RType(inst, PhantomData))),
            0b001_0011 => Some(Instruction::IntImm(IType(inst, PhantomData))),
            0b011_0111 => Some(Instruction::Lui(UType(inst, PhantomData))),
            0b001_0111 => Some(Instruction::Auipc(UType(inst, PhantomData))),
            0b110_1111 => Some(Instruction::Jal(UType(inst, PhantomData))),
            0b110_0111 => Some(Instruction::Jalr(IType(inst, PhantomData))),
            0b110_0011 => Some(Instruction::Branch(SType(inst, PhantomData))),
            0b000_1111 => Some(Instruction::Fence(UType(inst, PhantomData))),
            0b111_0011 => Some(Instruction::Ecall(IType(inst, PhantomData))),
            _ => None,
        }
    }
}
