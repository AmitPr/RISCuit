mod traits;

#[cfg(test)]
mod test;

use std::{fmt::Debug, marker::PhantomData};

macro_rules! impl_from_bits {
    (pub enum $enum_name:ident { $($variant:ident = $value:expr),* $(,)? }) => {
        #[derive(Debug, PartialEq, Eq)]
        pub enum $enum_name {
            $( $variant = $value ),*
        }

        impl $enum_name {
            pub const fn from(value: u8) -> Option<Self> {
                match value {
                    $( $value => Some($enum_name::$variant), )*
                    _ => None,
                }
            }
        }
    };
}

impl_from_bits!(
    pub enum LoadStoreFunc3 {
        IByte = 0b000,
        IHalf = 0b001,
        IWord = 0b010,
        UByte = 0b100,
        UHalf = 0b101,
        UWord = 0b110,
    }
);

impl_from_bits!(
    pub enum IntRegFunc3 {
        AddSub = 0b000,
        Sll = 0b001,
        Slt = 0b010,
        Sltu = 0b011,
        Xor = 0b100,
        SrlSra = 0b101,
        Or = 0b110,
        And = 0b111,
    }
);

impl_from_bits!(
    pub enum IntImmFunc3 {
        Addi = 0b000,
        Slli = 0b001,
        Slti = 0b010,
        Sltiu = 0b011,
        Xori = 0b100,
        SrliSrai = 0b101,
        Ori = 0b110,
        Andi = 0b111,
    }
);

impl_from_bits!(
    pub enum BranchFunc3 {
        Beq = 0b000,
        Bne = 0b001,
        Blt = 0b100,
        Bge = 0b101,
        Bltu = 0b110,
        Bgeu = 0b111,
    }
);

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

impl RType<IntRegFunc3> {
    pub const fn funct3(&self) -> Option<IntRegFunc3> {
        IntRegFunc3::from(((self.0 >> 12) & 0b111) as u8)
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

impl IType<IntImmFunc3> {
    pub const fn funct3(&self) -> Option<IntImmFunc3> {
        IntImmFunc3::from(((self.0 >> 12) & 0b111) as u8)
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

impl SType<LoadStoreFunc3, LImm> {
    pub const fn funct3(&self) -> Option<LoadStoreFunc3> {
        LoadStoreFunc3::from(((self.0 >> 12) & 0b111) as u8)
    }
}

impl SType<LoadStoreFunc3, SImm> {
    pub const fn funct3(&self) -> Option<LoadStoreFunc3> {
        LoadStoreFunc3::from(((self.0 >> 12) & 0b111) as u8)
    }
}

impl SType<BranchFunc3, BImm> {
    pub const fn funct3(&self) -> Option<BranchFunc3> {
        BranchFunc3::from(((self.0 >> 12) & 0b111) as u8)
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
    IntImm(IType<IntImmFunc3>),
    Lui(UType<UpperImm>),
    Auipc(UType<UpperImm>),
    IntReg(RType<IntRegFunc3>),
    Jal(UType<JImm>),
    Jalr(IType<JalrInst>),
    Branch(SType<BranchFunc3, BImm>),
    Load(SType<LoadStoreFunc3, LImm>),
    Store(SType<LoadStoreFunc3, SImm>),
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
