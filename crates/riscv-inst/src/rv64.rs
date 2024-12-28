use crate::instruction;

instruction! {
    64,
    r::R {
        opcode: [6:0],
        rd: [11:7],
        funct3: [14:12],
        rs1: [19:15],
        rs2: [24:20],
        funct7: [31:25],
    },
    mask: [31:25] | [14:12] | [6:0],
    opcodes {
        [6:0] == 0b0110011 && [14:12] == 0x0 && [31:25] == 0x00 => ADD,
        [6:0] == 0b0110011 && [14:12] == 0x0 && [31:25] == 0x20 => SUB,
        [6:0] == 0b0110011 && [14:12] == 0x1 && [31:25] == 0x00 => SLL,
        [6:0] == 0b0110011 && [14:12] == 0x2 && [31:25] == 0x00 => SLT,
        [6:0] == 0b0110011 && [14:12] == 0x3 && [31:25] == 0x00 => SLTU,
        [6:0] == 0b0110011 && [14:12] == 0x4 && [31:25] == 0x00 => XOR,
        [6:0] == 0b0110011 && [14:12] == 0x5 && [31:25] == 0x00 => SRL,
        [6:0] == 0b0110011 && [14:12] == 0x5 && [31:25] == 0x20 => SRA,
        [6:0] == 0b0110011 && [14:12] == 0x6 && [31:25] == 0x00 => OR,
        [6:0] == 0b0110011 && [14:12] == 0x7 && [31:25] == 0x00 => AND,

        // 64-bit instructions
        [6:0] == 0b0111011 && [14:12] == 0x0 && [31:25] == 0x00 => ADDW,
        [6:0] == 0b0111011 && [14:12] == 0x0 && [31:25] == 0x20 => SUBW,
        [6:0] == 0b0111011 && [14:12] == 0x1 && [31:25] == 0x00 => SLLW,
        [6:0] == 0b0111011 && [14:12] == 0x5 && [31:25] == 0x00 => SRLW,
        [6:0] == 0b0111011 && [14:12] == 0x5 && [31:25] == 0x20 => SRAW,

        // M extension
        #[cfg(feature="m")] [6:0] == 0b0110011 && [14:12] == 0x0 && [31:25] == 0x01 => MUL,
        #[cfg(feature="m")] [6:0] == 0b0110011 && [14:12] == 0x1 && [31:25] == 0x01 => MULH,
        #[cfg(feature="m")] [6:0] == 0b0110011 && [14:12] == 0x2 && [31:25] == 0x01 => MULHSU,
        #[cfg(feature="m")] [6:0] == 0b0110011 && [14:12] == 0x3 && [31:25] == 0x01 => MULHU,
        #[cfg(feature="m")] [6:0] == 0b0110011 && [14:12] == 0x4 && [31:25] == 0x01 => DIV,
        #[cfg(feature="m")] [6:0] == 0b0110011 && [14:12] == 0x5 && [31:25] == 0x01 => DIVU,
        #[cfg(feature="m")] [6:0] == 0b0110011 && [14:12] == 0x6 && [31:25] == 0x01 => REM,
        #[cfg(feature="m")] [6:0] == 0b0110011 && [14:12] == 0x7 && [31:25] == 0x01 => REMU,

        // RV64M extension
        #[cfg(feature="m")] [6:0] == 0b0111011 && [14:12] == 0x0 && [31:25] == 0x01 => MULW,
        #[cfg(feature="m")] [6:0] == 0b0111011 && [14:12] == 0x4 && [31:25] == 0x01 => DIVW,
        #[cfg(feature="m")] [6:0] == 0b0111011 && [14:12] == 0x6 && [31:25] == 0x01 => REMW,
        #[cfg(feature="m")] [6:0] == 0b0111011 && [14:12] == 0x5 && [31:25] == 0x01 => DIVUW,
        #[cfg(feature="m")] [6:0] == 0b0111011 && [14:12] == 0x7 && [31:25] == 0x01 => REMUW,
    }
}
pub use r::{Opcode as ROpcode, R};

instruction! {
    64,
    i::I {
        opcode: [6:0],
        rd: [11:7],
        funct3: [14:12],
        rs1: [19:15],
        shamt: [24:20],
        imm: [sigext 31:20],
    },
    mask: [14:12] | [6:0],
    opcodes {
        [6:0] == 0b0010011 && [14:12] == 0x0 => ADDI,
        [6:0] == 0b0010011 && [14:12] == 0x2 => SLTI,
        [6:0] == 0b0010011 && [14:12] == 0x3 => SLTIU,
        [6:0] == 0b0010011 && [14:12] == 0x4 => XORI,
        [6:0] == 0b0010011 && [14:12] == 0x6 => ORI,
        [6:0] == 0b0010011 && [14:12] == 0x7 => ANDI,

        [6:0] == 0b0010011 && [14:12] == 0x1 if [31:25] == 0x00 => SLLI,
        [6:0] == 0b0010011 && [14:12] == 0x5 if [31:25] == 0x00 => SRLI,
        [6:0] == 0b0010011 && [14:12] == 0x5 if [31:25] == 0x20 => SRAI,

        // 64-bit instructions
        [6:0] == 0b0011011 && [14:12] == 0x0 => ADDIW,
        [6:0] == 0b0011011 && [14:12] == 0x1 if [31:25] == 0x00 => SLLIW,
        [6:0] == 0b0011011 && [14:12] == 0x5 if [31:25] == 0x00 => SRLIW,
        [6:0] == 0b0011011 && [14:12] == 0x5 if [31:25] == 0x20 => SRAIW,

        [6:0] == 0b0000011 && [14:12] == 0x0 => LB,
        [6:0] == 0b0000011 && [14:12] == 0x1 => LH,
        [6:0] == 0b0000011 && [14:12] == 0x2 => LW,
        [6:0] == 0b0000011 && [14:12] == 0x3 => LD,
        [6:0] == 0b0000011 && [14:12] == 0x4 => LBU,
        [6:0] == 0b0000011 && [14:12] == 0x5 => LHU,

        [6:0] == 0b1100111 && [14:12] == 0x0 => JALR,

        // ECALL and EBREAK need special handling downstream to differentiate
        [6:0] == 0b1110011 && [14:12] == 0x0 => SYSTEM,
    }
}
pub use i::{Opcode as IOpcode, I};

instruction! {
    64,
    s::S {
        opcode: [6:0],
        funct3: [14:12],
        rs1: [19:15],
        rs2: [24:20],
        imm: [sigext 31:25 | 11:7],
    },
    mask: [14:12] | [6:0],
    opcodes {
        [6:0] == 0b0100011 && [14:12] == 0x0 => SB,
        [6:0] == 0b0100011 && [14:12] == 0x1 => SH,
        [6:0] == 0b0100011 && [14:12] == 0x2 => SW,

        // 64-bit instructions
        [6:0] == 0b0100011 && [14:12] == 0x3 => SD,
    }
}
pub use s::{Opcode as SOpcode, S};

instruction! {
    64,
    b::B {
        opcode: [6:0],
        funct3: [14:12],
        rs1: [19:15],
        rs2: [24:20],
        imm: [sigext 31:31 | 7:7 | 30:25 | 11:8 | <0 repeat 1>],
    },
    mask: [14:12] | [6:0],
    opcodes {
        [6:0] == 0b1100011 && [14:12] == 0x0 => BEQ,
        [6:0] == 0b1100011 && [14:12] == 0x1 => BNE,
        [6:0] == 0b1100011 && [14:12] == 0x4 => BLT,
        [6:0] == 0b1100011 && [14:12] == 0x5 => BGE,
        [6:0] == 0b1100011 && [14:12] == 0x6 => BLTU,
        [6:0] == 0b1100011 && [14:12] == 0x7 => BGEU,
    }
}
pub use b::{Opcode as BOpcode, B};

instruction! {
    64,
    u::U {
        opcode: [6:0],
        rd: [11:7],
        imm: [31:12 | <0 repeat 12>],
    },
    mask: [6:0],
    opcodes {
        [6:0] == 0b0110111 => LUI,
        [6:0] == 0b0010111 => AUIPC,
    }
}
pub use u::{Opcode as UOpcode, U};

instruction! {
    64,
    j::J {
        opcode: [6:0],
        rd: [11:7],
        imm: [sigext 31:31 | 19:12 | 20:20 | 30:21 | <0 repeat 1>],
    },
    mask: [6:0],
    opcodes {
        [6:0] == 0b1101111 => JAL,
    }
}
pub use j::{Opcode as JOpcode, J};

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    R { inst: R, op: ROpcode },
    I { inst: I, op: IOpcode },
    S { inst: S, op: SOpcode },
    B { inst: B, op: BOpcode },
    U { inst: U, op: UOpcode },
    J { inst: J, op: JOpcode },
}

#[cfg(feature = "c")]
pub use crate::rv_c::{RISCV_C2G_C0, RISCV_C2G_C1, RISCV_C2G_C2};

impl Opcode {
    pub const fn decode_32bits(inst: u32) -> Option<Self> {
        match inst & 0x7f {
            0b0110011 | 0b0111011 => match ROpcode::decode(inst) {
                Some(op) => Some(Opcode::R { inst: R(inst), op }),
                None => None,
            },
            0b0000011 | 0b1100111 | 0b1110011 | 0b0010011 => match IOpcode::decode(inst) {
                Some(op) => Some(Opcode::I { inst: I(inst), op }),
                None => None,
            },
            0b0100011 => match SOpcode::decode(inst) {
                Some(op) => Some(Opcode::S { inst: S(inst), op }),
                None => None,
            },
            0b1100011 => match BOpcode::decode(inst) {
                Some(op) => Some(Opcode::B { inst: B(inst), op }),
                None => None,
            },
            0b0110111 | 0b0010111 => match UOpcode::decode(inst) {
                Some(op) => Some(Opcode::U { inst: U(inst), op }),
                None => None,
            },
            0b1101111 => match JOpcode::decode(inst) {
                Some(op) => Some(Opcode::J { inst: J(inst), op }),
                None => None,
            },
            _ => None,
        }
    }

    pub const fn decode(inst: u32) -> Option<(Self, u32)> {
        macro_rules! option {
            ($e: expr) => {
                match $e {
                    Some(v) => v,
                    None => return None,
                }
            };
        }
        match inst & 0b11 {
            0b11 => Some((option!(Self::decode_32bits(inst)), 4)),

            #[cfg(feature = "c")]
            0b00 => Some((
                option!(Self::decode_32bits(
                    RISCV_C2G_C0[(inst as u16 >> 2) as usize]
                )),
                2,
            )),
            #[cfg(feature = "c")]
            0b01 => Some((
                option!(Self::decode_32bits(
                    RISCV_C2G_C1[(inst as u16 >> 2) as usize]
                )),
                2,
            )),
            #[cfg(feature = "c")]
            0b10 => Some((
                option!(Self::decode_32bits(
                    RISCV_C2G_C2[(inst as u16 >> 2) as usize]
                )),
                2,
            )),

            #[cfg(not(feature = "c"))]
            0b00 | 0b01 | 0b10 => None,

            _ => unreachable!(),
        }
    }
}
