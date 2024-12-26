#[macro_use]
mod macros;

#[cfg(test)]
mod test;

use std::ops::Range;

#[inline(always)]
#[must_use = "this returns the result of the operation, without modifying the original"]
/// Get the bits in range [start..end) and place them at dst_pos
pub(crate) const fn bits32(val: u32, range: Range<u8>, dst_pos: u8) -> u32 {
    let width = range.end - range.start;
    let mask = ((1u32 << width) - 1) & (val >> range.start);
    mask << dst_pos
}

instruction! {
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

        // M extension
        [6:0] == 0b0110011 && [14:12] == 0x0 && [31:25] == 0x01 => MUL,
        [6:0] == 0b0110011 && [14:12] == 0x1 && [31:25] == 0x01 => MULH,
        [6:0] == 0b0110011 && [14:12] == 0x2 && [31:25] == 0x01 => MULHSU,
        [6:0] == 0b0110011 && [14:12] == 0x3 && [31:25] == 0x01 => MULHU,
        [6:0] == 0b0110011 && [14:12] == 0x4 && [31:25] == 0x01 => DIV,
        [6:0] == 0b0110011 && [14:12] == 0x5 && [31:25] == 0x01 => DIVU,
        [6:0] == 0b0110011 && [14:12] == 0x6 && [31:25] == 0x01 => REM,
        [6:0] == 0b0110011 && [14:12] == 0x7 && [31:25] == 0x01 => REMU,
    }
}
pub use r::{Opcode as ROpcode, R};

instruction! {
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

        [6:0] == 0b0000011 && [14:12] == 0x0 => LB,
        [6:0] == 0b0000011 && [14:12] == 0x1 => LH,
        [6:0] == 0b0000011 && [14:12] == 0x2 => LW,
        [6:0] == 0b0000011 && [14:12] == 0x4 => LBU,
        [6:0] == 0b0000011 && [14:12] == 0x5 => LHU,

        [6:0] == 0b1100111 && [14:12] == 0x0 => JALR,

        // ECALL and EBREAK need special handling downstream to differentiate
        [6:0] == 0b1110011 && [14:12] == 0x0 => SYSTEM,
    }
}
pub use i::{Opcode as IOpcode, I};

instruction! {
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
    }
}
pub use s::{Opcode as SOpcode, S};

instruction! {
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

pub const fn decode(inst: u32) -> Option<Opcode> {
    match inst & 0x7f {
        0b0110011 => match ROpcode::decode(inst) {
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
