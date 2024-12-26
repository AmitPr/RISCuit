use crate::decode;
use crate::{BOpcode::*, IOpcode::*, JOpcode::*, Opcode::*, ROpcode::*, SOpcode::*, UOpcode::*};

// Tests derived from riscv-tests (https://github.com/riscv/riscv-tests)

#[test]
fn test_lui_auipc() {
    // lui x20,0x1
    let inst = decode(0x00001a37).unwrap();
    match inst {
        U { inst, op: LUI } => {
            assert_eq!(inst.rd(), 20); // x20
            assert_eq!(inst.imm(), 0x1000); // 0x1
        }
        _ => panic!("Wrong instruction type"),
    }

    // lui x5,0x80000
    let inst = decode(0x800002b7).unwrap();
    match inst {
        U { inst, op: LUI } => {
            assert_eq!(inst.rd(), 5); // x5
            assert_eq!(inst.imm(), 0x80000000);
        }
        _ => panic!("Wrong instruction type"),
    }

    // auipc x10,0xffffe
    let inst = decode(0xffffe517).unwrap();
    match inst {
        U { inst, op: AUIPC } => {
            assert_eq!(inst.rd(), 10); // x10
            assert_eq!(inst.imm(), 0xffffe000);
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_jumps() {
    // jal x0,0xfe1ff (offset -32)
    let inst = decode(0xfe1ff06f).unwrap();
    match inst {
        J { inst, op: JAL } => {
            assert_eq!(inst.rd(), 0); // x0
            assert_eq!(inst.imm(), -32); // Actual offset is imm*2
        }
        _ => panic!("Wrong instruction type"),
    }

    // jal x0, 20
    let inst = decode(0x0140006f).unwrap();
    match inst {
        J { inst, op: JAL } => {
            assert_eq!(inst.rd(), 0); // x0
            assert_eq!(inst.imm(), 20); // Actual offset is imm*2
        }
        _ => panic!("Wrong instruction type"),
    }

    // jalr x0,0(x1)
    let inst = decode(0x00008067).unwrap();
    match inst {
        I { inst, op: JALR } => {
            assert_eq!(inst.rd(), 0); // x0
            assert_eq!(inst.rs1(), 1); // x1
            assert_eq!(inst.imm(), 0);
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_branches() {
    // beq x15,x14,264 (offset 264)
    let inst = decode(0x10e78463).unwrap();
    match inst {
        B { inst, op: BEQ } => {
            assert_eq!(inst.rs1(), 15); // x15
            assert_eq!(inst.rs2(), 14); // x14
            assert_eq!(inst.imm(), 264);
        }
        _ => panic!("Wrong instruction type"),
    }

    // bne x4,x5,-32 (offset -32)
    let inst = decode(0xfe5210e3).unwrap();
    match inst {
        B { inst, op: BNE } => {
            assert_eq!(inst.rs1(), 4); // x4
            assert_eq!(inst.rs2(), 5); // x5
            assert_eq!(inst.imm(), -32);
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_loads() {
    // lb x5,32(x1)
    let inst = decode(0x02008283).unwrap();
    match inst {
        I { inst, op: LB } => {
            assert_eq!(inst.rd(), 5); // x5
            assert_eq!(inst.rs1(), 1); // x1
            assert_eq!(inst.imm(), 32);
        }
        _ => panic!("Wrong instruction type"),
    }

    // lhu x30,6(x1)
    let inst = decode(0x0060df03).unwrap();
    match inst {
        I { inst, op: LHU } => {
            assert_eq!(inst.rd(), 30); // x30
            assert_eq!(inst.rs1(), 1); // x1
            assert_eq!(inst.imm(), 6);
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_stores() {
    // sb x14,0(x15)
    let inst = decode(0x00e78023).unwrap();
    match inst {
        S { inst, op: SB } => {
            assert_eq!(inst.rs1(), 15); // x15
            assert_eq!(inst.rs2(), 14); // x14 (source register)
            assert_eq!(inst.imm(), 0);
        }
        _ => panic!("Wrong instruction type"),
    }

    // sh x2,-6(x1)
    let inst = decode(0xfe209d23).unwrap();
    match inst {
        S { inst, op: SH } => {
            assert_eq!(inst.rs1(), 1); // x1
            assert_eq!(inst.rs2(), 2); // x2 (source register)
            assert_eq!(inst.imm(), -6);
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_immediate_arithmetic() {
    // addi x15,x0,2
    let inst = decode(0x00200793).unwrap();
    match inst {
        I { inst, op: ADDI } => {
            assert_eq!(inst.rd(), 15); // x15
            assert_eq!(inst.rs1(), 0); // x0
            assert_eq!(inst.imm(), 2);
        }
        _ => panic!("Wrong instruction type"),
    }

    // slli x16,x16,0x3
    let inst = decode(0x00381813).unwrap();
    match inst {
        I { inst, op: SLLI } => {
            assert_eq!(inst.rd(), 16); // x16
            assert_eq!(inst.rs1(), 16); // x16
            assert_eq!(inst.shamt(), 3); // shift amount
        }
        _ => panic!("Wrong instruction type"),
    }

    // xori x13,x13,-1
    let inst = decode(0xfff6c693).unwrap();
    match inst {
        I { inst, op: XORI } => {
            assert_eq!(inst.rd(), 13); // x13
            assert_eq!(inst.rs1(), 13); // x13
            assert_eq!(inst.imm(), -1);
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_register_arithmetic() {
    // add x12,x11,x12
    let inst = decode(0x00c58633).unwrap();
    match inst {
        R { inst, op: ADD } => {
            assert_eq!(inst.rd(), 12); // x12
            assert_eq!(inst.rs1(), 11); // x11
            assert_eq!(inst.rs2(), 12); // x12
            assert_eq!(inst.funct7(), 0x00); // add vs sub
        }
        _ => panic!("Wrong instruction type"),
    }

    // sub x10,x10,x11
    let inst = decode(0x40b50533).unwrap();
    match inst {
        R { inst, op: SUB } => {
            assert_eq!(inst.rd(), 10); // x10
            assert_eq!(inst.rs1(), 10); // x10
            assert_eq!(inst.rs2(), 11); // x11
            assert_eq!(inst.funct7(), 0x20); // add vs sub
        }
        _ => panic!("Wrong instruction type"),
    }

    // xor x15,x12,x15
    let inst = decode(0x00f647b3).unwrap();
    match inst {
        R { inst, op: XOR } => {
            assert_eq!(inst.rd(), 15); // x15
            assert_eq!(inst.rs1(), 12); // x12
            assert_eq!(inst.rs2(), 15); // x15
            assert_eq!(inst.funct7(), 0x00);
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_system() {
    // ecall
    let inst = decode(0x00000073).unwrap();
    match inst {
        I { inst, op: SYSTEM } => {
            assert_eq!(inst.rd(), 0);
            assert_eq!(inst.rs1(), 0);
        }
        _ => panic!("Wrong instruction type"),
    }
}
