use super::execute;
use riscv_inst::rv64::{
    BOpcode::*, IOpcode::*, JOpcode::*, ROpcode::*, SOpcode::*, UOpcode::*, B, I, J, R, S, U,
};

execute! {
    R,
    ADD: (reg rs1, reg rs2): store => {
        rs1.wrapping_add(rs2)
    },
    SUB: (reg rs1, reg rs2): store => {
        rs1.wrapping_sub(rs2)
    },
    SLL: (reg rs1, reg rs2): store => {
        rs1 << (rs2 & 0x1f)
    },
    SLT: (reg rs1, reg rs2): store => {
        ((rs1 as i64) < (rs2 as i64)) as u64
    },
    SLTU: (reg rs1, reg rs2): store => {
        (rs1 < rs2) as u64
    },
    XOR: (reg rs1, reg rs2): store => {
        rs1 ^ rs2
    },
    SRL: (reg rs1, reg rs2): store => {
        rs1 >> (rs2 & 0x1f)
    },
    SRA: (reg rs1, reg rs2): store => {
        (rs1 as i64 >> (rs2 & 0x1f)) as u64
    },
    OR: (reg rs1, reg rs2): store => {
        rs1 | rs2
    },
    AND: (reg rs1, reg rs2): store => {
        rs1 & rs2
    },
    ADDW: (reg rs1, reg rs2): store => {
        (rs1 as i32).wrapping_add(rs2 as i32) as i64 as u64
    },
    SUBW: (reg rs1, reg rs2): store => {
        (rs1 as i32).wrapping_sub(rs2 as i32) as i64 as u64
    },
    SLLW: (reg rs1, reg rs2): store => {
        ((rs1 as i32) << (rs2 & 0x1f)) as i64 as u64
    },
    SRLW: (reg rs1, reg rs2): store => {
        (rs1 as i32 as u64) >> (rs2 & 0x1f)
    },
    SRAW: (reg rs1, reg rs2): store => {
        ((rs1 as i32) >> (rs2 & 0x1f)) as u64
    },
    MUL: (reg rs1, reg rs2): store => {
        rs1.wrapping_mul(rs2)
    },
    MULH: (reg rs1, reg rs2): store => {
        ((rs1 as i128 * rs2 as i128) >> 64) as u64
    },
    MULHSU: (reg rs1, reg rs2): store => {
        (((rs1 as i64 as i128) * (rs2 as i128)) >> 64) as u64
    },
    MULHU: (reg rs1, reg rs2): store => {
        ((rs1 as u128 * rs2 as u128) >> 64) as u64
    },
    DIV: (reg rs1, reg rs2): store => {
        if rs2 == 0 {
            // Division by zero returns -1
            u64::MAX
        } else if (rs1 as i64) == i64::MIN && (rs2 as i64) == -1 {
            // Handle signed division overflow
            rs1
        } else {
            (rs1 as i64).wrapping_div((rs2 as i64)) as u64
        }
    },
    DIVU: (reg rs1, reg rs2): store => {
        if rs2 == 0 {
            // Division by zero returns MAX
            u64::MAX
        } else {
            rs1.wrapping_div(rs2)
        }
    },
    REM: (reg rs1, reg rs2): store => {
        if rs2 == 0 {
            // Remainder of division by zero returns the dividend
            rs1
        } else if (rs1 as i64) == i64::MIN && (rs2 as i64) == -1 {
            // Handle signed division overflow - remainder is 0
            0
        } else {
            (rs1 as i64).wrapping_rem((rs2 as i64)) as u64
        }
    },
    REMU: (reg rs1, reg rs2): store => {
        if rs2 == 0 {
            // Remainder of division by zero returns the dividend
            rs1
        } else {
            rs1.wrapping_rem(rs2)
        }
    }
}
