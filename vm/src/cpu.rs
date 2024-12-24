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
        let inst = Instruction::decode(inst)
            .unwrap_or_else(|| panic!("Invalid instruction: {:08x}", inst));

        println!("0x{:08x}: {:?}", self.pc, inst);
        match inst {
            Instruction::IntImm(inst) => {
                let a = self.get_reg(inst.rs1());
                let b = inst.signed_imm();

                let res = match inst.funct3().expect("Invalid funct3") {
                    IntImmFunc3::Addi => a.wrapping_add_signed(b),
                    IntImmFunc3::Slti => ((a as i32) < b) as u32,
                    IntImmFunc3::Sltiu => (a < (b as u32)) as u32,
                    IntImmFunc3::Xori => a ^ (b as u32),
                    IntImmFunc3::Ori => a | (b as u32),
                    IntImmFunc3::Andi => a & (b as u32),
                    IntImmFunc3::Slli => a << inst.shamt(),
                    IntImmFunc3::SrliSrai => match inst.funct7() {
                        0 => a >> inst.shamt(),
                        0b0100000 => (a as i32 >> inst.shamt()) as u32,
                        _ => panic!("Invalid funct7: {:08x}", inst.funct7()),
                    },
                };

                self.set_reg(inst.rd(), res);
                self.pc = self.pc.wrapping_add(4);
            }
            Instruction::Lui(inst) => {
                self.set_reg(inst.rd(), inst.unsigned_imm());
                self.pc = self.pc.wrapping_add(4);
            }
            Instruction::Auipc(inst) => {
                let res = self.pc.wrapping_add_signed(inst.signed_imm());
                self.set_reg(inst.rd(), res);
                self.pc = self.pc.wrapping_add(4);
            }
            Instruction::IntReg(inst) => {
                let a = self.get_reg(inst.rs1());
                let b = self.get_reg(inst.rs2());

                let res = match inst.funct3().expect("Invalid funct3") {
                    IntRegFunc3::AddSub => match inst.funct7() {
                        0 => a.wrapping_add(b),
                        0b0100000 => a.wrapping_sub(b),
                        _ => panic!("Invalid funct7: {:08x}", inst.funct7()),
                    },
                    IntRegFunc3::Slt => ((a as i32) < (b as i32)) as u32,
                    IntRegFunc3::Sltu => (a < b) as u32,
                    IntRegFunc3::Sll => a << (b & 0x1f),
                    IntRegFunc3::SrlSra => match inst.funct7() {
                        0 => a >> (b & 0x1f),
                        0b0100000 => (a as i32 >> (b & 0x1f)) as u32,
                        _ => panic!("Invalid funct7: {:08x}", inst.funct7()),
                    },
                    IntRegFunc3::Xor => a ^ b,
                    IntRegFunc3::Or => a | b,
                    IntRegFunc3::And => a & b,
                };

                self.set_reg(inst.rd(), res);
                self.pc = self.pc.wrapping_add(4);
            }
            Instruction::Jal(inst) => {
                let next_pc = self.pc.wrapping_add(4);
                self.set_reg(inst.rd(), next_pc);
                self.pc = self.pc.wrapping_add_signed(inst.signed_imm());
            }
            Instruction::Jalr(inst) => {
                let next_pc = self.pc.wrapping_add(4);
                self.set_reg(inst.rd(), next_pc);

                let base = self.get_reg(inst.rs1());
                self.pc = base.wrapping_add_signed(inst.signed_imm());
            }
            Instruction::Branch(inst) => {
                let a = self.get_reg(inst.rs1());
                let b = self.get_reg(inst.rs2());

                let taken = match inst.funct3().expect("Invalid funct3") {
                    BranchFunc3::Beq => a == b,
                    BranchFunc3::Bne => a != b,
                    BranchFunc3::Blt => (a as i32) < (b as i32),
                    BranchFunc3::Bge => (a as i32) >= (b as i32),
                    BranchFunc3::Bltu => a < b,
                    BranchFunc3::Bgeu => a >= b,
                };

                if taken {
                    self.pc = self.pc.wrapping_add_signed(inst.signed_imm());
                } else {
                    self.pc = self.pc.wrapping_add(4);
                }
            }
            Instruction::Load(inst) => {
                let base = self.get_reg(inst.rs1());
                let src = base.wrapping_add_signed(inst.signed_imm());

                let res = match inst.funct3().expect("Invalid funct3") {
                    LoadStoreFunc3::IByte => self.mem.load_byte(src) as i8 as i32 as u32,
                    LoadStoreFunc3::IHalf => self.mem.load_half(src) as i16 as i32 as u32,
                    LoadStoreFunc3::UByte => self.mem.load_byte(src) as u32,
                    LoadStoreFunc3::UHalf => self.mem.load_half(src) as u32,
                    LoadStoreFunc3::IWord | LoadStoreFunc3::UWord => self.mem.load_word(src),
                };

                self.set_reg(inst.rd(), res);
                self.pc = self.pc.wrapping_add(4);
            }
            Instruction::Store(inst) => {
                let base = self.get_reg(inst.rs1());
                let dest = base.wrapping_add_signed(inst.signed_imm());
                let src = self.get_reg(inst.rs2());

                match inst.funct3().expect("Invalid funct3") {
                    LoadStoreFunc3::IByte => self.mem.store_byte(dest, src as u8),
                    LoadStoreFunc3::IHalf => self.mem.store_half(dest, src as u16),
                    LoadStoreFunc3::IWord => self.mem.store_word(dest, src),
                    _ => panic!("Invalid funct3"),
                }

                self.pc = self.pc.wrapping_add(4);
            }
            Instruction::Fence(_) => {
                // no-op
                self.pc = self.pc.wrapping_add(4);
            }
            Instruction::Ecall(inst) => {
                if inst.is_ebreak() {
                    println!("ebreak at 0x{:08x}", self.pc);
                    self.running = false;
                } else {
                    let a0 = self.get_reg(10);
                    let a1 = self.get_reg(11);
                    println!("ecall {}: {:x}", a0, a1);
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
