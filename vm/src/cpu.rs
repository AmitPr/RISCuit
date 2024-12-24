use riscv_inst::*;

use crate::memory::Memory;
// use riscv_decode::{decode, Instruction};
use std::io::{self, Write};

/// A simple CPU for RV32I instructions
pub struct Cpu {
    regs: [u32; 32],
    pub pc: u32,
    pub mem: Memory,
    pub running: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            regs: [0; 32],
            pc: 0,
            mem: Memory::new(),
            running: true,
        }
    }

    /// Read a register. x0 is always 0 in RISC-V.
    fn get_reg(&self, r: u32) -> u32 {
        if r == 0 {
            0
        } else {
            self.regs[r as usize]
        }
    }

    /// Write a register (except x0).
    fn set_reg(&mut self, r: u32, val: u32) {
        if r != 0 {
            self.regs[r as usize] = val;
        }
    }

    /// Execute one instruction
    fn step(&mut self) {
        if !self.running {
            return;
        }

        let inst = self.mem.load_word(self.pc);
        let inst = Instruction::new(inst);

        let opcode: Opcode = inst
            .opcode()
            .unwrap_or_else(|| panic!("Invalid instruction: {:08x}", inst.0));
        println!("0x{:08x}: {:?}", self.pc, opcode);
        match opcode {
            Opcode::Load => {
                let dest = inst.rd();
                let base = self.get_reg(inst.rs1());
                let offset = inst.signed_imm();
                let src = base.wrapping_add_signed(offset);

                let func3 = LoadStoreFunc3::from(inst.funct3()).expect("Invalid funct3");
                match func3 {
                    LoadStoreFunc3::IByte => {
                        let byte = self.mem.load_byte(src) as i8 as i32 as u32;
                        self.set_reg(dest, byte);
                    }
                    LoadStoreFunc3::IHalf => {
                        let half = self.mem.load_half(src) as i16 as i32 as u32;
                        self.set_reg(dest, half);
                    }
                    LoadStoreFunc3::UByte => {
                        let byte = self.mem.load_byte(src) as u32;
                        self.set_reg(dest, byte);
                    }
                    LoadStoreFunc3::UHalf => {
                        let half = self.mem.load_half(src) as u32;
                        self.set_reg(dest, half);
                    }
                    LoadStoreFunc3::IWord | LoadStoreFunc3::UWord => {
                        let word = self.mem.load_word(src);
                        self.set_reg(dest, word);
                    }
                }
                self.pc = self.pc.wrapping_add(4);
            }
            Opcode::Store => {
                // TODO: Modify riscv-inst for better S-type decoding
                let u_offset = ((inst.funct7() as u32) << 5) | inst.rd();
                let offset = (u_offset << 20) as i32 >> 20;
                let base = self.get_reg(inst.rs1());
                let src = self.get_reg(inst.rs2());
                let dest = base.wrapping_add_signed(offset);

                let func3 = LoadStoreFunc3::from(inst.funct3()).expect("Invalid funct3");
                match func3 {
                    LoadStoreFunc3::IByte => {
                        let data = (src & 0xff) as u8;
                        self.mem.store_byte(dest, data);
                    }
                    LoadStoreFunc3::IHalf => {
                        let data = (src & 0xffff) as u16;
                        self.mem.store_half(dest, data);
                    }
                    LoadStoreFunc3::IWord => {
                        self.mem.store_word(dest, src);
                    }
                    _ => {}
                }
                self.pc = self.pc.wrapping_add(4);
            }
            Opcode::IntReg => {
                let dest = inst.rd();
                let a = self.get_reg(inst.rs1());
                let b = self.get_reg(inst.rs2());

                let func3 = IntRegFunc3::from(inst.funct3()).expect("Invalid funct3");
                match func3 {
                    IntRegFunc3::AddSub => {
                        let func7 = inst.funct7();
                        if func7 == 0 {
                            // add
                            let res = a.wrapping_add(b);
                            self.set_reg(dest, res);
                        } else if func7 == 0b0100000 {
                            // sub
                            let res = a.wrapping_sub(b);
                            self.set_reg(dest, res);
                        } else {
                            todo!();
                        }
                    }
                    IntRegFunc3::Slt => {
                        let res = (a as i32) < (b as i32);
                        self.set_reg(dest, res as u32);
                    }
                    IntRegFunc3::Sltu => {
                        let res = (a < b) as u32;
                        self.set_reg(dest, res);
                    }
                    IntRegFunc3::Sll => {
                        let shamt = b & 0x1f;
                        let res = a << shamt;
                        self.set_reg(dest, res);
                    }
                    IntRegFunc3::SrlSra => {
                        let func7 = inst.funct7();
                        if func7 == 0 {
                            // srl
                            let shamt = b & 0x1f;
                            let res = a >> shamt;
                            self.set_reg(dest, res);
                        } else if func7 == 0b0100000 {
                            // sra (sign-extend)
                            let shamt = b & 0x1f;
                            let res = (a as i32 >> shamt) as u32;
                            self.set_reg(dest, res);
                        }
                    }
                    IntRegFunc3::Xor => {
                        let res = a ^ b;
                        self.set_reg(dest, res);
                    }
                    IntRegFunc3::Or => {
                        let res = a | b;
                        self.set_reg(dest, res);
                    }
                    IntRegFunc3::And => {
                        let res = a & b;
                        self.set_reg(dest, res);
                    }
                }
                self.pc = self.pc.wrapping_add(4);
            }
            Opcode::IntImm => {
                let dest = inst.rd();
                let a = self.get_reg(inst.rs1());
                let b = inst.signed_imm();

                let func3 = IntImmFunc3::from(inst.funct3()).expect("Invalid funct3");
                match func3 {
                    IntImmFunc3::Addi => {
                        let res = a.wrapping_add_signed(b);
                        self.set_reg(dest, res);
                    }
                    IntImmFunc3::Slti => {
                        let res = (a as i32) < b;
                        self.set_reg(dest, res as u32);
                    }
                    IntImmFunc3::Sltiu => {
                        let res = a < (b as u32);
                        self.set_reg(dest, res as u32);
                    }
                    IntImmFunc3::Xori => {
                        let res = a ^ (b as u32);
                        self.set_reg(dest, res);
                    }
                    IntImmFunc3::Ori => {
                        let res = a | (b as u32);
                        self.set_reg(dest, res);
                    }
                    IntImmFunc3::Andi => {
                        let res = a & (b as u32);
                        self.set_reg(dest, res);
                    }
                    IntImmFunc3::Slli => {
                        let shamt = b & 0x1f;
                        let res = a << shamt;
                        self.set_reg(dest, res);
                    }
                    IntImmFunc3::SrliSrai => {
                        let func7 = inst.funct7();
                        if func7 == 0 {
                            // srli
                            let shamt = b & 0x1f;
                            let res = a >> shamt;
                            self.set_reg(dest, res);
                        } else if func7 == 0b0100000 {
                            // srai (sign-extend)
                            let shamt = b & 0x1f;
                            let res = (a as i32 >> shamt) as u32;
                            self.set_reg(dest, res);
                        } else {
                            todo!();
                        }
                    }
                }
                self.pc = self.pc.wrapping_add(4);
            }
            Opcode::Lui => {
                let dest = inst.rd();
                let imm = inst.imm20() << 12;
                self.set_reg(dest, imm);
                self.pc = self.pc.wrapping_add(4);
            }
            Opcode::Auipc => {
                let dest = inst.rd();
                let imm = inst.imm20() << 12;
                let res = self.pc.wrapping_add_signed(imm as i32);
                self.set_reg(dest, res);
                self.pc = self.pc.wrapping_add(4);
            }
            Opcode::Jal => {
                let dest = inst.rd();
                let next_pc = self.pc.wrapping_add(4);
                self.set_reg(dest, next_pc);

                let raw = inst.0;
                let offset = ((((raw & 0x80000000) as i32) >> 11) as u32) // imm[20]
                    | (raw & 0xFF000) // imm[19:12]
                    | ((raw >> 9) & 0x800) // imm[11]
                    | ((raw >> 20) & 0x7FE); // imm[10:1]
                println!(
                    "jal: pc=0x{:08x}, offset=0x{:08x}, next={:08x}",
                    self.pc,
                    offset,
                    self.pc.wrapping_add_signed(offset as i32)
                );
                self.pc = self.pc.wrapping_add_signed(offset as i32);
            }
            Opcode::Jalr => {
                let dest = inst.rd();
                let base = self.get_reg(inst.rs1());
                let offset = inst.signed_imm();
                let next_pc = self.pc.wrapping_add(4);
                self.set_reg(dest, next_pc);
                self.pc = base.wrapping_add_signed(offset);
            }
            Opcode::Branch => {
                let u_imm = ((inst.funct7() as u32) << 5) | inst.rd();
                let offset = (u_imm << 20) as i32 >> 20;
                let a = self.get_reg(inst.rs1());
                let b = self.get_reg(inst.rs2());

                let func3 = BranchFunc3::from(inst.funct3()).expect("Invalid funct3");
                match func3 {
                    BranchFunc3::Beq => {
                        if a == b {
                            self.pc = self.pc.wrapping_add_signed(offset);
                        } else {
                            self.pc = self.pc.wrapping_add(4);
                        }
                    }
                    BranchFunc3::Bne => {
                        if a != b {
                            self.pc = self.pc.wrapping_add_signed(offset);
                        } else {
                            self.pc = self.pc.wrapping_add(4);
                        }
                    }
                    BranchFunc3::Blt => {
                        if (a as i32) < (b as i32) {
                            self.pc = self.pc.wrapping_add_signed(offset);
                        } else {
                            self.pc = self.pc.wrapping_add(4);
                        }
                    }
                    BranchFunc3::Bge => {
                        if (a as i32) >= (b as i32) {
                            self.pc = self.pc.wrapping_add_signed(offset);
                        } else {
                            self.pc = self.pc.wrapping_add(4);
                        }
                    }
                    BranchFunc3::Bltu => {
                        if a < b {
                            self.pc = self.pc.wrapping_add_signed(offset);
                        } else {
                            self.pc = self.pc.wrapping_add(4);
                        }
                    }
                    BranchFunc3::Bgeu => {
                        if a >= b {
                            self.pc = self.pc.wrapping_add_signed(offset);
                        } else {
                            self.pc = self.pc.wrapping_add(4);
                        }
                    }
                }
            }
            Opcode::Fence => {
                // TODO: Do we need to implement this?
                self.pc = self.pc.wrapping_add(4);
            }
            Opcode::Ecall => {
                if inst.funct7() == 1 {
                    // ebreak
                    println!("ebreak at 0x{:08x}", self.pc);
                    self.running = false;
                } else {
                    let a0 = self.get_reg(10);
                    let a1 = self.get_reg(11);
                    match a0 {
                        1 => {
                            // print int
                            print!("{}", a1 as i32);
                            io::stdout().flush().ok();
                        }
                        2 => {
                            // print char
                            let c = (a1 & 0xff) as u8 as char;
                            print!("{}", c);
                            io::stdout().flush().ok();
                        }
                        3 => {
                            // print string from [a1..]
                            let mut addr = a1;
                            loop {
                                let b = self.mem.load_byte(addr);
                                if b == 0 {
                                    break;
                                }
                                print!("{}", b as char);
                                addr = addr.wrapping_add(1);
                            }
                            io::stdout().flush().ok();
                        }
                        93 => {
                            // exit
                            println!("exiting with code {}", a1 as i32);
                            self.running = false;
                        }
                        _ => {}
                    }
                    self.pc = self.pc.wrapping_add(4);
                }
            }
        }
    }

    /// Run until halted
    pub fn run(&mut self) {
        while self.running {
            self.step();
        }
    }
}
