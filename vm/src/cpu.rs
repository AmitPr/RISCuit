use riscv_inst::{
    BOpcode::*, IOpcode::*, JOpcode::*, Opcode::*, ROpcode::*, SOpcode::*, UOpcode::*,
};

use crate::memory::Memory;

/// A simple CPU for RV32I instructions
pub struct Cpu32 {
    regs: [u32; 32],
    pub pc: u32,
    pub mem: Memory,
    pub running: bool,
}

impl Cpu32 {
    pub fn new() -> Self {
        Cpu32 {
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
        let op =
            riscv_inst::decode(inst).unwrap_or_else(|| panic!("Invalid instruction: {:08x}", inst));

        println!("0x{:08x}:\t{inst:08x}\t{:?}", self.pc, op);
        match op {
            R { inst, op } => {
                let a = self.get_reg(inst.rs1());
                // SLLI, SRLI, SRAI don't use b/rs2, so only load it when needed
                macro_rules! b {
                    () => {
                        self.get_reg(inst.rs2())
                    };
                }
                let res = match op {
                    ADD => a.wrapping_add(b!()),
                    SUB => a.wrapping_sub(b!()),
                    SLL => a << (b!() & 0x1f),
                    SLT => ((a as i32) < (b!() as i32)) as u32,
                    SLTU => (a < b!()) as u32,
                    XOR => a ^ b!(),
                    SRL => a >> (b!() & 0x1f),
                    SRA => (a as i32 >> (b!() & 0x1f)) as u32,
                    OR => a | b!(),
                    AND => a & b!(),
                    SLLI => a << inst.rs2(),
                    SRLI => a >> inst.rs2(),
                    SRAI => (a as i32 >> inst.rs2()) as u32,

                    MUL => a.wrapping_mul(b!()),
                    MULH => ((a as i64 * b!() as i64) >> 32) as u32,
                    MULHSU => (((a as i32 as i64) * (b!() as i64)) >> 32) as u32,
                    MULHU => ((a as u64 * b!() as u64) >> 32) as u32,
                    DIV => {
                        let a = a as i32;
                        let b = b!() as i32;
                        if b == 0 {
                            // Division by zero returns -1
                            u32::MAX
                        } else if a == i32::MIN && b == -1 {
                            // Handle signed division overflow
                            a as u32
                        } else {
                            a.wrapping_div(b) as u32
                        }
                    }
                    DIVU => {
                        let b = b!();
                        if b == 0 {
                            // Division by zero returns MAX
                            u32::MAX
                        } else {
                            a.wrapping_div(b)
                        }
                    }
                    REM => {
                        let a = a as i32;
                        let b = b!() as i32;
                        if b == 0 {
                            // Remainder of division by zero returns the dividend
                            a as u32
                        } else if a == i32::MIN && b == -1 {
                            // Handle signed division overflow - remainder is 0
                            0
                        } else {
                            a.wrapping_rem(b) as u32
                        }
                    }
                    REMU => {
                        let b = b!();
                        if b == 0 {
                            // Remainder of division by zero returns the dividend
                            a
                        } else {
                            a.wrapping_rem(b)
                        }
                    }
                };
                self.set_reg(inst.rd(), res);
                self.pc = self.pc.wrapping_add(4);
            }
            I { inst, op } => {
                let a = self.get_reg(inst.rs1());
                let imm = inst.imm();

                let res = match op {
                    ADDI => a.wrapping_add_signed(imm),
                    SLTI => ((a as i32) < imm) as u32,
                    SLTIU => (a < (imm as u32)) as u32,
                    XORI => a ^ (imm as u32),
                    ORI => a | (imm as u32),
                    ANDI => a & (imm as u32),
                    LB => self.mem.load_byte(a.wrapping_add_signed(imm)) as i8 as i32 as u32,
                    LH => self.mem.load_half(a.wrapping_add_signed(imm)) as i16 as i32 as u32,
                    LW => self.mem.load_word(a.wrapping_add_signed(imm)),
                    LBU => self.mem.load_byte(a.wrapping_add_signed(imm)) as u32,
                    LHU => self.mem.load_half(a.wrapping_add_signed(imm)) as u32,
                    JALR => self.pc.wrapping_add(4),
                    SYSTEM => {
                        if imm == 0 {
                            // ecall
                            let a0 = self.get_reg(10);
                            let a1 = self.get_reg(11);
                            match a0 {
                                1 => print!("{}", a1 as i32),
                                2 => print!("{}", (a1 & 0xff) as u8 as char),
                                3 => {
                                    let mut addr = a1;
                                    loop {
                                        let b = self.mem.load_byte(addr);
                                        if b == 0 {
                                            break;
                                        }
                                        print!("{}", b as char);
                                        addr = addr.wrapping_add(1);
                                    }
                                }
                                93 => {
                                    println!("exit: {}", a1 as i32);
                                    self.running = false;
                                }
                                _ => {}
                            }
                        } else {
                            // ebreak
                            println!("ebreak");
                            self.running = false;
                        }
                        // these don't modify rd1()
                        0
                    }
                };

                // Apply the result
                match op {
                    JALR => {
                        self.set_reg(inst.rd(), res);
                        self.pc = a.wrapping_add_signed(imm);
                    }
                    SYSTEM => {
                        self.pc = self.pc.wrapping_add(4);
                    }
                    _ => {
                        self.set_reg(inst.rd(), res);
                        self.pc = self.pc.wrapping_add(4);
                    }
                }
            }
            S { inst, op } => {
                let base = self.get_reg(inst.rs1());
                let src = self.get_reg(inst.rs2());
                let addr = base.wrapping_add_signed(inst.imm());
                match op {
                    SB => self.mem.store_byte(addr, src as u8),
                    SH => self.mem.store_half(addr, src as u16),
                    SW => self.mem.store_word(addr, src),
                }

                self.pc = self.pc.wrapping_add(4);
            }
            B { inst, op } => {
                let a = self.get_reg(inst.rs1());
                let b = self.get_reg(inst.rs2());
                let taken = match op {
                    BEQ => a == b,
                    BNE => a != b,
                    BLT => (a as i32) < (b as i32),
                    BGE => (a as i32) >= (b as i32),
                    BLTU => a < b,
                    BGEU => a >= b,
                };
                self.pc = if taken {
                    self.pc.wrapping_add_signed(inst.imm())
                } else {
                    self.pc.wrapping_add(4)
                };
            }
            U { inst, op } => {
                let imm = inst.imm();
                let res = match op {
                    LUI => imm << 12,
                    AUIPC => self.pc.wrapping_add_signed((imm << 12) as i32),
                };
                self.set_reg(inst.rd(), res);
                self.pc = self.pc.wrapping_add(4);
            }
            J { inst, op } => match op {
                JAL => {
                    let next_pc = self.pc.wrapping_add(4);
                    self.set_reg(inst.rd(), next_pc);
                    self.pc = self.pc.wrapping_add_signed(inst.imm());
                }
            },
        }
    }

    /// Run until halted
    pub fn run(&mut self) {
        while self.running {
            self.step();
        }
    }
}
