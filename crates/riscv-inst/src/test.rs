use crate::*;
use Instruction::*;

// Tests derived from riscv-tests (https://github.com/riscv/riscv-tests)

#[test]
fn test_lui_auipc() {
    // lui x20,0x1
    let inst = Instruction::decode(0x00001a37).unwrap();
    match inst {
        Lui(utype) => {
            assert_eq!(utype.rd(), 20); // x20
            assert_eq!(utype.unsigned_imm(), 0x1000); // 0x1 << 12
            assert_eq!(utype.signed_imm(), 0x1000); // Same since positive
        }
        _ => panic!("Wrong instruction type"),
    }

    // lui x5,0x80000
    let inst = Instruction::decode(0x800002b7).unwrap();
    match inst {
        Lui(utype) => {
            assert_eq!(utype.rd(), 5); // x5
            assert_eq!(utype.unsigned_imm(), 0x80000000);
            assert_eq!(utype.signed_imm(), -0x80000000_i32); // Sign extended
        }
        _ => panic!("Wrong instruction type"),
    }

    // auipc x10,0xffffe
    let inst = Instruction::decode(0xffffe517).unwrap();
    match inst {
        Auipc(utype) => {
            assert_eq!(utype.rd(), 10); // x10
            assert_eq!(utype.unsigned_imm(), 0xffffe000);
            assert_eq!(utype.signed_imm(), -0x2000_i32); // Sign extended
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_jumps() {
    // jal x0,0xfe1ff (offset -32)
    let inst = Instruction::decode(0xfe1ff06f).unwrap();
    match inst {
        Jal(utype) => {
            assert_eq!(utype.rd(), 0); // x0
            assert_eq!(utype.signed_imm(), -32); // Actual offset is imm*2
        }
        _ => panic!("Wrong instruction type"),
    }

    // jal x0, 20
    let inst = Instruction::decode(0x0140006f).unwrap();
    match inst {
        Jal(utype) => {
            assert_eq!(utype.rd(), 0); // x0
            assert_eq!(utype.signed_imm(), 20); // Actual offset is imm*2
        }
        _ => panic!("Wrong instruction type"),
    }

    // jalr x0,0(x1)
    let inst = Instruction::decode(0x00008067).unwrap();
    match inst {
        Jalr(itype) => {
            assert_eq!(itype.rd(), 0); // x0
            assert_eq!(itype.rs1(), 1); // x1
            assert_eq!(itype.signed_imm(), 0);
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_branches() {
    // beq x15,x14,264 (offset 264)
    let inst = Instruction::decode(0x10e78463).unwrap();
    match inst {
        Branch(stype) => {
            let branch_inst = SType::<BranchFunc3, BImm>(stype.0, PhantomData);
            assert_eq!(branch_inst.rs1(), 15); // x15
            assert_eq!(branch_inst.rs2(), 14); // x14
            assert_eq!(branch_inst.funct3(), Some(BranchFunc3::Beq));
            assert_eq!(branch_inst.signed_imm(), 264);
        }
        _ => panic!("Wrong instruction type"),
    }

    // bne x4,x5,-32 (offset -32)
    let inst = Instruction::decode(0xfe5210e3).unwrap();
    match inst {
        Branch(stype) => {
            let branch_inst = SType::<BranchFunc3, BImm>(stype.0, PhantomData);
            assert_eq!(branch_inst.rs1(), 4); // x4
            assert_eq!(branch_inst.rs2(), 5); // x5
            assert_eq!(branch_inst.funct3(), Some(BranchFunc3::Bne));
            assert_eq!(branch_inst.signed_imm(), -32);
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_loads() {
    // lb x5,32(x1)
    let inst = Instruction::decode(0x02008283).unwrap();
    match inst {
        Load(stype) => {
            let load_inst = SType::<LoadStoreFunc3, LImm>(stype.0, PhantomData);
            assert_eq!(load_inst.rd(), 5); // x5
            assert_eq!(load_inst.rs1(), 1); // x1
            assert_eq!(load_inst.funct3(), Some(LoadStoreFunc3::IByte));
            assert_eq!(load_inst.signed_imm(), 32);
        }
        _ => panic!("Wrong instruction type"),
    }

    // lhu x30,6(x1)
    let inst = Instruction::decode(0x0060df03).unwrap();
    match inst {
        Load(stype) => {
            let load_inst = SType::<LoadStoreFunc3, LImm>(stype.0, PhantomData);
            assert_eq!(load_inst.rd(), 30); // x30
            assert_eq!(load_inst.rs1(), 1); // x1
            assert_eq!(load_inst.funct3(), Some(LoadStoreFunc3::UHalf));
            assert_eq!(load_inst.signed_imm(), 6);
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_stores() {
    // sb x14,0(x15)
    let inst = Instruction::decode(0x00e78023).unwrap();
    match inst {
        Store(stype) => {
            let store_inst = SType::<LoadStoreFunc3, SImm>(stype.0, PhantomData);
            assert_eq!(store_inst.rs1(), 15); // x15
            assert_eq!(store_inst.rs2(), 14); // x14 (source register)
            assert_eq!(store_inst.funct3(), Some(LoadStoreFunc3::IByte));
            assert_eq!(store_inst.signed_imm(), 0);
        }
        _ => panic!("Wrong instruction type"),
    }

    // sh x2,-6(x1)
    let inst = Instruction::decode(0xfe209d23).unwrap();
    match inst {
        Store(stype) => {
            let store_inst = SType::<LoadStoreFunc3, SImm>(stype.0, PhantomData);
            assert_eq!(store_inst.rs1(), 1); // x1
            assert_eq!(store_inst.rs2(), 2); // x2 (source register)
            assert_eq!(store_inst.funct3(), Some(LoadStoreFunc3::IHalf));
            assert_eq!(store_inst.signed_imm(), -6);
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_immediate_arithmetic() {
    // addi x15,x0,2
    let inst = Instruction::decode(0x00200793).unwrap();
    match inst {
        IntImm(itype) => {
            assert_eq!(itype.rd(), 15); // x15
            assert_eq!(itype.rs1(), 0); // x0
            assert_eq!(itype.funct3(), Some(IntImmFunc3::Addi));
            assert_eq!(itype.signed_imm(), 2);
        }
        _ => panic!("Wrong instruction type"),
    }

    // slli x16,x16,0x3
    let inst = Instruction::decode(0x00381813).unwrap();
    match inst {
        IntImm(itype) => {
            assert_eq!(itype.rd(), 16); // x16
            assert_eq!(itype.rs1(), 16); // x16
            assert_eq!(itype.funct3(), Some(IntImmFunc3::Slli));
            assert_eq!(itype.shamt(), 3); // shift amount
        }
        _ => panic!("Wrong instruction type"),
    }

    // xori x13,x13,-1
    let inst = Instruction::decode(0xfff6c693).unwrap();
    match inst {
        IntImm(itype) => {
            assert_eq!(itype.rd(), 13); // x13
            assert_eq!(itype.rs1(), 13); // x13
            assert_eq!(itype.funct3(), Some(IntImmFunc3::Xori));
            assert_eq!(itype.signed_imm(), -1);
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_register_arithmetic() {
    // add x12,x11,x12
    let inst = Instruction::decode(0x00c58633).unwrap();
    match inst {
        IntReg(rtype) => {
            assert_eq!(rtype.rd(), 12); // x12
            assert_eq!(rtype.rs1(), 11); // x11
            assert_eq!(rtype.rs2(), 12); // x12
            assert_eq!(rtype.funct3(), Some(IntRegFunc3::AddSub));
            assert_eq!(rtype.funct7(), 0x00); // add vs sub
        }
        _ => panic!("Wrong instruction type"),
    }

    // sub x10,x10,x11
    let inst = Instruction::decode(0x40b50533).unwrap();
    match inst {
        IntReg(rtype) => {
            assert_eq!(rtype.rd(), 10); // x10
            assert_eq!(rtype.rs1(), 10); // x10
            assert_eq!(rtype.rs2(), 11); // x11
            assert_eq!(rtype.funct3(), Some(IntRegFunc3::AddSub));
            assert_eq!(rtype.funct7(), 0x20); // add vs sub
        }
        _ => panic!("Wrong instruction type"),
    }

    // xor x15,x12,x15
    let inst = Instruction::decode(0x00f647b3).unwrap();
    match inst {
        IntReg(rtype) => {
            assert_eq!(rtype.rd(), 15); // x15
            assert_eq!(rtype.rs1(), 12); // x12
            assert_eq!(rtype.rs2(), 15); // x15
            assert_eq!(rtype.funct3(), Some(IntRegFunc3::Xor));
            assert_eq!(rtype.funct7(), 0x00);
        }
        _ => panic!("Wrong instruction type"),
    }
}

#[test]
fn test_system() {
    // ecall
    let inst = Instruction::decode(0x00000073).unwrap();
    match inst {
        Ecall(itype) => {
            assert_eq!(itype.rd(), 0);
            assert_eq!(itype.rs1(), 0);
            assert!(!itype.is_ebreak()); // Not ebreak
            assert_eq!(itype.ecall_code(), 0);
        }
        _ => panic!("Wrong instruction type"),
    }
}
