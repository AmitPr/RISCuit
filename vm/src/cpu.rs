use riscv_inst::rv64::{
    BOpcode::*,
    IOpcode::*,
    JOpcode::*,
    Opcode::{self, *},
    ROpcode::*,
    SOpcode::*,
    UOpcode::*,
};

// use crate::memory::Memory;
use crate::memory64::Memory64;

/// A simple CPU for RV32I instructions
pub struct Cpu {
    regs: [u64; 32],
    pub pc: u64,
    pub mem: Memory64,
    pub running: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            regs: [0; 32],
            pc: 0,
            mem: Memory64::new(),
            running: true,
        }
    }

    /// Read a register. x0 is always 0 in RISC-V.
    fn get_reg(&self, r: u32) -> u64 {
        if r == 0 {
            0
        } else {
            self.regs[r as usize]
        }
    }

    /// Write a register (except x0).
    fn set_reg(&mut self, r: u32, val: u64) {
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
        let (op, inc) =
            Opcode::decode(inst).unwrap_or_else(|| panic!("Invalid instruction: {:08x}", inst));

        let mut next_pc = self.pc.wrapping_add(inc as u64);

        println!("0x{:08x}:\t{inst:08x}\t{:?}", self.pc, op);
        match op {
            R { inst, op } => {
                let a = self.get_reg(inst.rs1() as u32);
                let b = self.get_reg(inst.rs2() as u32);
                let res = match op {
                    ADD => a.wrapping_add(b),
                    SUB => a.wrapping_sub(b),
                    SLL => a.wrapping_shl(b as u32 & 0x3f),
                    SRL => a.wrapping_shr(b as u32 & 0x3f),
                    SRA => (a as i64).wrapping_shr(b as u32 & 0x3f) as u64,
                    SLT => ((a as i64) < (b as i64)) as u64,
                    SLTU => (a < b) as u64,
                    XOR => a ^ b,
                    OR => a | b,
                    AND => a & b,

                    ADDW => (a as i32).wrapping_add(b as i32) as i64 as u64,
                    SUBW => (a as i32).wrapping_sub(b as i32) as i64 as u64,
                    SLLW => ((a as i32).wrapping_shl(b as u32 & 0x1f)) as i64 as u64,
                    SRLW => ((a as i32).wrapping_shr(b as u32 & 0x1f)) as i64 as u64,
                    SRAW => ((a as i32 as i64).wrapping_shr(b as u32 & 0x1f)) as u64,

                    MUL => a.wrapping_mul(b),
                    MULH => ((a as i128 * b as i128) >> 64) as u64,
                    MULHSU => (((a as i64 as i128) * (b as i128)) >> 64) as u64,
                    MULHU => ((a as u128 * b as u128) >> 64) as u64,
                    DIV => {
                        let a = a as i64;
                        let b = b as i64;
                        if b == 0 {
                            // Division by zero returns -1
                            u64::MAX
                        } else if a == i64::MIN && b == -1 {
                            // Handle signed division overflow
                            a as u64
                        } else {
                            a.wrapping_div(b) as u64
                        }
                    }
                    DIVU => {
                        if b == 0 {
                            // Division by zero returns MAX
                            u64::MAX
                        } else {
                            a.wrapping_div(b)
                        }
                    }
                    REM => {
                        let a = a as i64;
                        let b = b as i64;
                        if b == 0 {
                            // Remainder of division by zero returns the dividend
                            a as u64
                        } else if a == i64::MIN && b == -1 {
                            // Handle signed division overflow - remainder is 0
                            0
                        } else {
                            a.wrapping_rem(b) as u64
                        }
                    }
                    REMU => {
                        if b == 0 {
                            // Remainder of division by zero returns the dividend
                            a
                        } else {
                            a.wrapping_rem(b)
                        }
                    }

                    MULW => (a as i32).wrapping_mul(b as i32) as i64 as u64,
                    DIVW => {
                        let a = a as i32;
                        let b = b as i32;
                        if b == 0 {
                            // Division by zero returns -1
                            u64::MAX
                        } else if a == i32::MIN && b == -1 {
                            // Handle signed division overflow
                            a as u64
                        } else {
                            a.wrapping_div(b) as i64 as u64
                        }
                    }
                    REMW => {
                        let a = a as i32;
                        let b = b as i32;
                        if b == 0 {
                            // Remainder of division by zero returns the dividend
                            a as u64
                        } else if a == i32::MIN && b == -1 {
                            // Handle signed division overflow - remainder is 0
                            0
                        } else {
                            a.wrapping_rem(b) as i64 as u64
                        }
                    }
                    DIVUW => {
                        if b == 0 {
                            // Division by zero returns MAX
                            u64::MAX
                        } else {
                            (a as u32).wrapping_div(b as u32) as i64 as u64
                        }
                    }
                    REMUW => {
                        if b == 0 {
                            // Remainder of division by zero returns the dividend
                            a
                        } else {
                            (a as u32).wrapping_rem(b as u32) as i64 as u64
                        }
                    }
                };
                self.set_reg(inst.rd() as u32, res);
            }
            I { inst, op } => {
                let a = self.get_reg(inst.rs1() as u32);
                let imm = inst.imm();

                let res = match op {
                    ADDI => a.wrapping_add_signed(imm),
                    ADDIW => (a as i32).wrapping_add(imm as i32) as i64 as u64,
                    SLTI => ((a as i64) < imm) as u64,
                    SLTIU => (a < (imm as u64)) as u64,
                    XORI => a ^ (imm as u64),
                    ORI => a | (imm as u64),
                    ANDI => a & (imm as u64),
                    SLLI => a.wrapping_shl(inst.shamt() as u32),
                    SLLIW => (a as i32).wrapping_shl(inst.shamt() as u32) as i64 as u64,
                    SRLI => a.wrapping_shr(inst.shamt() as u32),
                    SRLIW => (a as i32).wrapping_shr(inst.shamt() as u32) as i64 as u64,
                    SRAI => (a as i64).wrapping_shr(inst.shamt() as u32) as u64,
                    SRAIW => ((a as i32).wrapping_shr(inst.shamt() as u32)) as i64 as u64,
                    LB => self.mem.load_byte(a.wrapping_add_signed(imm)) as i8 as i64 as u64,
                    LH => self.mem.load_half(a.wrapping_add_signed(imm)) as i16 as i64 as u64,
                    LW => self.mem.load_word(a.wrapping_add_signed(imm)) as i32 as i64 as u64,
                    LD => self.mem.load_dword(a.wrapping_add_signed(imm)),
                    LBU => self.mem.load_byte(a.wrapping_add_signed(imm)) as u64,
                    LHU => self.mem.load_half(a.wrapping_add_signed(imm)) as u64,
                    JALR => next_pc,
                    SYSTEM => {
                        self.syscall(a, imm);
                        // these don't modify rd1
                        0
                    }
                };

                // Apply the result
                match op {
                    JALR => {
                        self.set_reg(inst.rd() as u32, res);
                        next_pc = a.wrapping_add_signed(imm);
                    }
                    SYSTEM => {}
                    _ => {
                        self.set_reg(inst.rd() as u32, res);
                    }
                }
            }
            S { inst, op } => {
                let base = self.get_reg(inst.rs1() as u32);
                let src = self.get_reg(inst.rs2() as u32);
                let addr = base.wrapping_add_signed(inst.imm());
                match op {
                    SB => self.mem.store_byte(addr, src as u8),
                    SH => self.mem.store_half(addr, src as u16),
                    SW => self.mem.store_word(addr, src as u32),
                    SD => self.mem.store_dword(addr, src),
                }
            }
            B { inst, op } => {
                let a = self.get_reg(inst.rs1() as u32);
                let b = self.get_reg(inst.rs2() as u32);
                let taken = match op {
                    BEQ => a == b,
                    BNE => a != b,
                    BLT => (a as i64) < (b as i64),
                    BGE => (a as i64) >= (b as i64),
                    BLTU => a < b,
                    BGEU => a >= b,
                };
                if taken {
                    next_pc = self.pc.wrapping_add_signed(inst.imm());
                }
            }
            U { inst, op } => {
                let imm = inst.imm();
                let res = match op {
                    LUI => imm as i32 as i64 as u64,
                    AUIPC => self.pc.wrapping_add_signed(imm as i32 as i64),
                };
                self.set_reg(inst.rd() as u32, res);
            }
            J { inst, op } => match op {
                JAL => {
                    self.set_reg(inst.rd() as u32, next_pc);
                    next_pc = self.pc.wrapping_add_signed(inst.imm());
                }
            },
        }

        self.pc = next_pc;
    }

    pub fn syscall(&mut self, _rs1: u64, imm: i64) {
        if imm == 0 {
            // ecall
            let a0 = self.get_reg(10);
            let a1 = self.get_reg(11);
            println!("ecall: {} {}", a0, a1);
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
                _ => {
                    println!("Unknown syscall: {}", a0);
                    self.running = false;
                }
            }
        } else {
            // ebreak
            println!("ebreak");
            self.running = false;
        }
    }

    /// Run until halted
    pub fn run(&mut self) {
        while self.running {
            self.step();
        }
    }
}
