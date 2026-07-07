//! RV64IMASC execution.
//!
//! Width-sensitive semantics relative to rv32: loads narrower than 64 bits
//! sign-extend unless the `u` variant is used, `*w` instructions operate on
//! the low 32 bits and sign-extend their result, and register shift amounts
//! take 6 bits. `as`-casts from signed types sign-extend, so `i32 as u64`
//! is the idiomatic sext32 here.
use riscv_inst::codegen::rv64imasc::Rv64IMASC;
use riscv_inst::Reg;

use super::{mem_fault, take_err, Exec, Execute, Hart, X64};
use crate::{
    error::{HartError, MachineError, MemoryAccess, MemoryError},
    machine::{Kernel, StepResult},
    memory::Memory,
};

impl Execute for X64 {
    fn run<K: Kernel<Xlen = Self>>(
        hart: &mut Hart<Self>,
        mem: &mut K::Memory,
        kernel: &mut K,
    ) -> Result<(), MachineError<K::Error>> {
        // Same discipline as rv32: pc and the instruction count live in
        // registers; inst_count is flushed before kernel entry and on exit.
        let mut pc = hart.pc;
        let mut count = 0u64;
        let mut err = None;
        let result = loop {
            let Ok(inst) = mem.load::<u32>(pc) else {
                break Err(mem_fault(MemoryAccess::Load, pc as u64));
            };
            let Some(op) = Rv64IMASC::parse(inst) else {
                break Err(HartError::invalid(pc, inst).into());
            };
            match exec_op_at(hart, op, inst, &mut pc, mem, &mut err) {
                Exec::Next => count += 1,
                Exec::Error => break Err(take_err(&mut err)),
                trap => {
                    hart.inst_count += count;
                    count = 0;
                    let res = match trap {
                        Exec::Syscall => kernel.syscall(hart, mem),
                        _ => kernel.ebreak(hart, mem),
                    };
                    match res {
                        Ok(StepResult::Ok) => {
                            count += 1;
                            pc = pc.wrapping_add(if inst & 0b11 == 0b11 { 4 } else { 2 });
                            hart.pc = pc;
                        }
                        Ok(StepResult::Halt) => break Ok(()),
                        Err(e) => break Err(e),
                    }
                }
            }
        };
        hart.inst_count += count;
        result
    }

    fn step<K: Kernel<Xlen = Self>>(
        hart: &mut Hart<Self>,
        mem: &mut K::Memory,
        kernel: &mut K,
    ) -> Result<StepResult, MachineError<K::Error>> {
        let mut pc = hart.pc;
        let Ok(inst) = mem.load::<u32>(pc) else {
            return Err(mem_fault(MemoryAccess::Load, pc as u64));
        };
        let op = Rv64IMASC::parse(inst).ok_or_else(|| HartError::invalid(pc, inst))?;

        let mut err = None;
        match exec_op_at(hart, op, inst, &mut pc, mem, &mut err) {
            Exec::Next => {
                hart.inst_count += 1;
                Ok(StepResult::Ok)
            }
            Exec::Error => Err(take_err(&mut err)),
            trap => {
                let res = match trap {
                    Exec::Syscall => kernel.syscall(hart, mem)?,
                    _ => kernel.ebreak(hart, mem)?,
                };
                if let StepResult::Ok = res {
                    hart.inst_count += 1;
                    hart.pc = pc.wrapping_add(if inst & 0b11 == 0b11 { 4 } else { 2 });
                }
                Ok(res)
            }
        }
    }
}

/// Execute an already-decoded instruction at `*pc`.
///
/// On [`Exec::Next`], the next pc is written through `pc` (an out-parameter
/// rather than an enum payload; see [`Exec`]) and `hart.pc` is updated. On
/// [`Exec::Error`], the error is deposited in `err`. On the trap variants
/// and on error, `*pc` and `hart.pc` are left untouched.
#[inline(always)]
fn exec_op_at<M: Memory<Addr = u64>, E: std::error::Error>(
    hart: &mut Hart<X64>,
    op: Rv64IMASC,
    inst: u32,
    pc: &mut u64,
    mem: &mut M,
    err: &mut Option<MachineError<E>>,
) -> Exec {
    let pc_out = pc;
    let pc = *pc_out;
    let pc_inc = if inst & 0b11 == 0b11 { 4 } else { 2 };
    let mut next_pc = pc.wrapping_add(pc_inc);

    // Error return: deposit the error in the caller's slot (see `Exec`).
    macro_rules! fail {
        ($e:expr) => {{
            *err = Some($e);
            return Exec::Error;
        }};
    }

    // Guest memory accessors: attach fault context (cold) at the call site
    // so the hot path only carries a zero-sized error.
    macro_rules! load {
        ($t:ty, $addr:expr) => {{
            let a = $addr;
            match mem.load::<$t>(a) {
                Ok(v) => v,
                Err(_) => fail!(mem_fault(MemoryAccess::Load, a as u64)),
            }
        }};
    }
    macro_rules! store {
        ($t:ty, $addr:expr, $val:expr) => {{
            let a = $addr;
            if mem.store::<$t>(a, $val).is_err() {
                fail!(mem_fault(MemoryAccess::Store, a as u64));
            }
        }};
    }

    macro_rules! reg {
        ($reg: expr) => {
            hart.get_reg($reg)
        };
        ($reg: expr, $val: expr) => {
            hart.set_reg($reg, $val as u64)
        };
    }

    macro_rules! reg_imm_op {
        (|$inst:ident.$rs1:ident, $inst2:ident.$imm:ident| $body:expr) => {{
            let $rs1 = reg!($inst.$rs1(inst));
            let $imm = $inst2.$imm(inst);
            reg!($inst.rd(inst), { $body } as u64);
        }};
    }

    macro_rules! imm_op {
        (|$inst:ident.$imm:ident| $body:expr) => {{
            let $imm = $inst.$imm(inst);
            reg!($inst.rd(inst), { $body } as u64);
        }};
    }

    macro_rules! reg_reg_op {
        (|$inst:ident.$rs1:ident, $inst2:ident.$rs2:ident| $body:expr) => {{
            let $rs1 = reg!($inst.$rs1(inst));
            let $rs2 = reg!($inst2.$rs2(inst));
            reg!($inst.rd(inst), { $body } as u64);
        }};
    }

    macro_rules! store_op {
        (|$inst:ident.$rs1:ident, $inst2:ident.$rs2:ident, $addr:ident| $body:expr) => {{
            let $rs1 = reg!($inst.$rs1(inst));
            let $rs2 = reg!($inst2.$rs2(inst));
            let $addr = $rs1.wrapping_add_signed($inst.imm(inst) as i64);
            $body;
        }};
    }

    macro_rules! branch_op {
        (|$inst:ident.$rs1:ident, $inst2:ident.$rs2:ident| $body:expr) => {{
            let $rs1 = reg!($inst.$rs1(inst));
            let $rs2 = reg!($inst2.$rs2(inst));
            let offset = $inst.imm(inst) as i64;
            // Select, not branch: guest branch outcomes are data-driven.
            next_pc = if $body {
                pc.wrapping_add_signed(offset)
            } else {
                next_pc
            };
        }};
    }

    macro_rules! csr_op {
        (|$inst:ident.$csr:ident, $inst2:ident.$rs1:ident| $body:expr) => {{
            let $csr = $inst.$csr(inst) as usize;
            let $rs1 = reg!($inst2.$rs1(inst));
            reg!($inst.rd(inst), hart.csrs[$csr]);
            $body;
        }};
    }

    macro_rules! csr_imm_op {
        (|$inst:ident.$csr:ident, $inst2:ident.$imm:ident| $body:expr) => {{
            let $csr = $inst.$csr(inst) as usize;
            let $imm = $inst2.$imm(inst) as u64;
            reg!($inst.rd(inst), hart.csrs[$csr]);
            $body;
        }};
    }

    /// 32-bit AMO: rd gets the sign-extended old word; the operation runs
    /// on the low 32 bits.
    macro_rules! amo_w_op {
        (|$inst:ident, $old:ident, $rs2:ident| $body:expr) => {{
            let addr = reg!($inst.rs1(inst));
            if addr & 3 != 0 {
                fail!(MemoryError::UnalignedMemoryAccess {
                    access: MemoryAccess::Load,
                    addr,
                    required: 4,
                }
                .into());
            }
            let $old = load!(i32, addr) as u32;
            reg!($inst.rd(inst), $old as i32 as i64);
            let $rs2 = reg!($inst.rs2(inst)) as u32;
            store!(u32, addr, { $body } as u32);
        }};
    }

    macro_rules! amo_d_op {
        (|$inst:ident, $old:ident, $rs2:ident| $body:expr) => {{
            let addr = reg!($inst.rs1(inst));
            if addr & 7 != 0 {
                fail!(MemoryError::UnalignedMemoryAccess {
                    access: MemoryAccess::Load,
                    addr,
                    required: 8,
                }
                .into());
            }
            let $old = load!(u64, addr);
            reg!($inst.rd(inst), $old);
            let $rs2 = reg!($inst.rs2(inst));
            store!(u64, addr, { $body } as u64);
        }};
    }

    match op {
        // --- RV64I ---
        Rv64IMASC::Lui(lui) => imm_op!(|lui.imm| imm as i64),
        Rv64IMASC::Auipc(auipc) => imm_op!(|auipc.imm| pc.wrapping_add_signed(imm as i64)),
        Rv64IMASC::Jal(jal) => imm_op!(|jal.imm| {
            let res = next_pc;
            next_pc = pc.wrapping_add_signed(imm as i64);
            res
        }),
        Rv64IMASC::Jalr(jalr) => reg_imm_op!(|jalr.rs1, jalr.imm| {
            let res = next_pc;
            next_pc = rs1.wrapping_add_signed(imm as i64);
            res
        }),
        Rv64IMASC::Beq(beq) => branch_op!(|beq.rs1, beq.rs2| rs1 == rs2),
        Rv64IMASC::Bne(bne) => branch_op!(|bne.rs1, bne.rs2| rs1 != rs2),
        Rv64IMASC::Blt(blt) => branch_op!(|blt.rs1, blt.rs2| (rs1 as i64) < (rs2 as i64)),
        Rv64IMASC::Bge(bge) => branch_op!(|bge.rs1, bge.rs2| (rs1 as i64) >= (rs2 as i64)),
        Rv64IMASC::Bltu(bltu) => branch_op!(|bltu.rs1, bltu.rs2| rs1 < rs2),
        Rv64IMASC::Bgeu(bgeu) => branch_op!(|bgeu.rs1, bgeu.rs2| rs1 >= rs2),
        Rv64IMASC::Lb(lb) => reg_imm_op!(
            |lb.rs1, lb.imm| load!(i8, rs1.wrapping_add_signed(imm as i64)) as i64
        ),
        Rv64IMASC::Lh(lh) => reg_imm_op!(
            |lh.rs1, lh.imm| load!(i16, rs1.wrapping_add_signed(imm as i64)) as i64
        ),
        Rv64IMASC::Lw(lw) => reg_imm_op!(
            |lw.rs1, lw.imm| load!(i32, rs1.wrapping_add_signed(imm as i64)) as i64
        ),
        Rv64IMASC::Lbu(lbu) => reg_imm_op!(
            |lbu.rs1, lbu.imm| load!(u8, rs1.wrapping_add_signed(imm as i64))
        ),
        Rv64IMASC::Lhu(lhu) => reg_imm_op!(
            |lhu.rs1, lhu.imm| load!(u16, rs1.wrapping_add_signed(imm as i64))
        ),
        Rv64IMASC::Lwu(lwu) => reg_imm_op!(
            |lwu.rs1, lwu.imm| load!(u32, rs1.wrapping_add_signed(imm as i64))
        ),
        Rv64IMASC::Ld(ld) => reg_imm_op!(
            |ld.rs1, ld.imm| load!(u64, rs1.wrapping_add_signed(imm as i64))
        ),
        Rv64IMASC::Sb(sb) => {
            store_op!(|sb.rs1, sb.rs2, addr| store!(u8, addr, rs2 as u8))
        }
        Rv64IMASC::Sh(sh) => {
            store_op!(|sh.rs1, sh.rs2, addr| store!(u16, addr, rs2 as u16))
        }
        Rv64IMASC::Sw(sw) => {
            store_op!(|sw.rs1, sw.rs2, addr| store!(u32, addr, rs2 as u32))
        }
        Rv64IMASC::Sd(sd) => {
            store_op!(|sd.rs1, sd.rs2, addr| store!(u64, addr, rs2))
        }
        Rv64IMASC::Addi(addi) => {
            reg_imm_op!(|addi.rs1, addi.imm| rs1.wrapping_add_signed(imm as i64))
        }
        Rv64IMASC::Slti(slti) => reg_imm_op!(|slti.rs1, slti.imm| (rs1 as i64) < (imm as i64)),
        Rv64IMASC::Sltiu(sltiu) => reg_imm_op!(|sltiu.rs1, sltiu.imm| rs1 < (imm as u64)),
        Rv64IMASC::Xori(xori) => reg_imm_op!(|xori.rs1, xori.imm| rs1 ^ (imm as u64)),
        Rv64IMASC::Ori(ori) => reg_imm_op!(|ori.rs1, ori.imm| rs1 | (imm as u64)),
        Rv64IMASC::Andi(andi) => reg_imm_op!(|andi.rs1, andi.imm| rs1 & (imm as u64)),
        Rv64IMASC::Slli(slli) => reg_imm_op!(|slli.rs1, slli.shamt| rs1 << shamt),
        Rv64IMASC::Srli(srli) => reg_imm_op!(|srli.rs1, srli.shamt| rs1 >> shamt),
        Rv64IMASC::Srai(srai) => reg_imm_op!(|srai.rs1, srai.shamt| rs1 as i64 >> shamt),
        Rv64IMASC::Add(add) => reg_reg_op!(|add.rs1, add.rs2| rs1.wrapping_add(rs2)),
        Rv64IMASC::Sub(sub) => reg_reg_op!(|sub.rs1, sub.rs2| rs1.wrapping_sub(rs2)),
        Rv64IMASC::Sll(sll) => reg_reg_op!(|sll.rs1, sll.rs2| rs1 << (rs2 & 0x3f)),
        Rv64IMASC::Slt(slt) => reg_reg_op!(|slt.rs1, slt.rs2| (rs1 as i64) < (rs2 as i64)),
        Rv64IMASC::Sltu(sltu) => reg_reg_op!(|sltu.rs1, sltu.rs2| rs1 < rs2),
        Rv64IMASC::Xor(xor) => reg_reg_op!(|xor.rs1, xor.rs2| rs1 ^ rs2),
        Rv64IMASC::Srl(srl) => reg_reg_op!(|srl.rs1, srl.rs2| rs1 >> (rs2 & 0x3f)),
        Rv64IMASC::Sra(sra) => {
            reg_reg_op!(|sra.rs1, sra.rs2| (rs1 as i64 >> (rs2 & 0x3f)) as u64)
        }
        Rv64IMASC::Or(or) => reg_reg_op!(|or.rs1, or.rs2| rs1 | rs2),
        Rv64IMASC::And(and) => reg_reg_op!(|and.rs1, and.rs2| rs1 & rs2),
        Rv64IMASC::Fence(_) => {}
        Rv64IMASC::FenceI(_) => {}
        Rv64IMASC::Ecall(_) => return Exec::Syscall,
        Rv64IMASC::Ebreak(_) => return Exec::Ebreak,
        Rv64IMASC::Unimp(_) => fail!(HartError::illegal(pc, inst).into()),

        // --- RV64I *W (operate on low 32 bits, sign-extend result) ---
        Rv64IMASC::Addiw(addiw) => reg_imm_op!(
            |addiw.rs1, addiw.imm| (rs1 as u32).wrapping_add_signed(imm) as i32 as i64
        ),
        Rv64IMASC::Slliw(slliw) => reg_imm_op!(
            |slliw.rs1, slliw.shamt| ((rs1 as u32) << shamt) as i32 as i64
        ),
        Rv64IMASC::Srliw(srliw) => reg_imm_op!(
            |srliw.rs1, srliw.shamt| ((rs1 as u32) >> shamt) as i32 as i64
        ),
        Rv64IMASC::Sraiw(sraiw) => reg_imm_op!(
            |sraiw.rs1, sraiw.shamt| ((rs1 as i32) >> shamt) as i64
        ),
        Rv64IMASC::Addw(addw) => reg_reg_op!(
            |addw.rs1, addw.rs2| (rs1 as u32).wrapping_add(rs2 as u32) as i32 as i64
        ),
        Rv64IMASC::Subw(subw) => reg_reg_op!(
            |subw.rs1, subw.rs2| (rs1 as u32).wrapping_sub(rs2 as u32) as i32 as i64
        ),
        Rv64IMASC::Sllw(sllw) => reg_reg_op!(
            |sllw.rs1, sllw.rs2| ((rs1 as u32) << (rs2 & 0x1f)) as i32 as i64
        ),
        Rv64IMASC::Srlw(srlw) => reg_reg_op!(
            |srlw.rs1, srlw.rs2| ((rs1 as u32) >> (rs2 & 0x1f)) as i32 as i64
        ),
        Rv64IMASC::Sraw(sraw) => reg_reg_op!(
            |sraw.rs1, sraw.rs2| ((rs1 as i32) >> (rs2 & 0x1f)) as i64
        ),

        // --- M ---
        Rv64IMASC::Mul(mul) => reg_reg_op!(|mul.rs1, mul.rs2| rs1.wrapping_mul(rs2)),
        Rv64IMASC::Mulh(mulh) => reg_reg_op!(
            |mulh.rs1, mulh.rs2| ((rs1 as i64 as i128).wrapping_mul(rs2 as i64 as i128) >> 64)
                as u64
        ),
        Rv64IMASC::Mulhsu(mulhsu) => reg_reg_op!(
            |mulhsu.rs1, mulhsu.rs2| ((rs1 as i64 as i128).wrapping_mul(rs2 as i128) >> 64) as u64
        ),
        Rv64IMASC::Mulhu(mulhu) => reg_reg_op!(
            |mulhu.rs1, mulhu.rs2| ((rs1 as u128 * rs2 as u128) >> 64) as u64
        ),
        Rv64IMASC::Div(div) => reg_reg_op!(|div.rs1, div.rs2| {
            let rs1 = rs1 as i64;
            let rs2 = rs2 as i64;
            if rs2 == 0 {
                u64::MAX
            } else if rs1 == i64::MIN && rs2 == -1 {
                rs1 as u64
            } else {
                rs1.wrapping_div(rs2) as u64
            }
        }),
        Rv64IMASC::Divu(divu) => reg_reg_op!(|divu.rs1, divu.rs2| {
            if rs2 == 0 {
                u64::MAX
            } else {
                rs1.wrapping_div(rs2)
            }
        }),
        Rv64IMASC::Rem(rem) => reg_reg_op!(|rem.rs1, rem.rs2| {
            let rs1 = rs1 as i64;
            let rs2 = rs2 as i64;
            if rs2 == 0 {
                rs1 as u64
            } else if rs1 == i64::MIN && rs2 == -1 {
                0
            } else {
                rs1.wrapping_rem(rs2) as u64
            }
        }),
        Rv64IMASC::Remu(remu) => reg_reg_op!(|remu.rs1, remu.rs2| {
            if rs2 == 0 {
                rs1
            } else {
                rs1.wrapping_rem(rs2)
            }
        }),
        Rv64IMASC::Mulw(mulw) => reg_reg_op!(
            |mulw.rs1, mulw.rs2| (rs1 as i32).wrapping_mul(rs2 as i32) as i64
        ),
        Rv64IMASC::Divw(divw) => reg_reg_op!(|divw.rs1, divw.rs2| {
            let rs1 = rs1 as i32;
            let rs2 = rs2 as i32;
            (if rs2 == 0 {
                -1
            } else if rs1 == i32::MIN && rs2 == -1 {
                rs1
            } else {
                rs1.wrapping_div(rs2)
            }) as i64
        }),
        Rv64IMASC::Divuw(divuw) => reg_reg_op!(|divuw.rs1, divuw.rs2| {
            let rs1 = rs1 as u32;
            let rs2 = rs2 as u32;
            (if rs2 == 0 { u32::MAX } else { rs1.wrapping_div(rs2) }) as i32 as i64
        }),
        Rv64IMASC::Remw(remw) => reg_reg_op!(|remw.rs1, remw.rs2| {
            let rs1 = rs1 as i32;
            let rs2 = rs2 as i32;
            (if rs2 == 0 {
                rs1
            } else if rs1 == i32::MIN && rs2 == -1 {
                0
            } else {
                rs1.wrapping_rem(rs2)
            }) as i64
        }),
        Rv64IMASC::Remuw(remuw) => reg_reg_op!(|remuw.rs1, remuw.rs2| {
            let rs1 = rs1 as u32;
            let rs2 = rs2 as u32;
            (if rs2 == 0 { rs1 } else { rs1.wrapping_rem(rs2) }) as i32 as i64
        }),

        // --- System ---
        Rv64IMASC::Uret(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv64IMASC::Sret(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv64IMASC::Hret(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv64IMASC::Mret(_) => {
            // TODO: Not erroring because the ISA tests use this.
            // But we haven't implemented privilege levels yet.
        }
        Rv64IMASC::Dret(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv64IMASC::SfenceVm(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv64IMASC::SfenceVma(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv64IMASC::Wfi(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv64IMASC::Csrrw(rw) => csr_op!(|rw.csr12, rw.rs1| hart.csrs[csr12] = rs1),
        Rv64IMASC::Csrrs(rs) => csr_op!(|rs.csr12, rs.rs1| hart.csrs[csr12] |= rs1),
        Rv64IMASC::Csrrc(rc) => csr_op!(|rc.csr12, rc.rs1| hart.csrs[csr12] &= !rs1),
        Rv64IMASC::Csrrwi(wi) => csr_imm_op!(|wi.csr12, wi.imm| hart.csrs[csr12] = imm),
        Rv64IMASC::Csrrsi(ri) => csr_imm_op!(|ri.csr12, ri.imm| hart.csrs[csr12] |= imm),
        Rv64IMASC::Csrrci(ci) => csr_imm_op!(|ci.csr12, ci.imm| hart.csrs[csr12] &= !imm),

        // --- A (32-bit) ---
        Rv64IMASC::LrW(lr_w) => {
            let addr = reg!(lr_w.rs1(inst));
            if addr & 3 != 0 {
                fail!(MemoryError::UnalignedMemoryAccess {
                    access: MemoryAccess::Load,
                    addr,
                    required: 4,
                }
                .into());
            }
            hart.amo_rsv = Some(addr);
            reg!(lr_w.rd(inst), load!(i32, addr) as i64);
        }
        Rv64IMASC::ScW(sc_w) => {
            let addr = reg!(sc_w.rs1(inst));
            if addr & 3 != 0 {
                fail!(MemoryError::UnalignedMemoryAccess {
                    access: MemoryAccess::Store,
                    addr,
                    required: 4,
                }
                .into());
            }
            if hart.amo_rsv.take() == Some(addr) {
                store!(u32, addr, reg!(sc_w.rs2(inst)) as u32);
                reg!(sc_w.rd(inst), 0);
            } else {
                reg!(sc_w.rd(inst), 1);
            }
        }
        Rv64IMASC::AmoswapW(swap) => amo_w_op!(|swap, old, rs2| rs2),
        Rv64IMASC::AmoaddW(add) => amo_w_op!(|add, old, rs2| old.wrapping_add(rs2)),
        Rv64IMASC::AmoxorW(xor) => amo_w_op!(|xor, old, rs2| old ^ rs2),
        Rv64IMASC::AmoorW(or) => amo_w_op!(|or, old, rs2| old | rs2),
        Rv64IMASC::AmoandW(and) => amo_w_op!(|and, old, rs2| old & rs2),
        Rv64IMASC::AmominW(min) => amo_w_op!(|min, old, rs2| (old as i32).min(rs2 as i32)),
        Rv64IMASC::AmomaxW(max) => amo_w_op!(|max, old, rs2| (old as i32).max(rs2 as i32)),
        Rv64IMASC::AmominuW(minu) => amo_w_op!(|minu, old, rs2| old.min(rs2)),
        Rv64IMASC::AmomaxuW(maxu) => amo_w_op!(|maxu, old, rs2| old.max(rs2)),

        // --- A (64-bit) ---
        Rv64IMASC::LrD(lr_d) => {
            let addr = reg!(lr_d.rs1(inst));
            if addr & 7 != 0 {
                fail!(MemoryError::UnalignedMemoryAccess {
                    access: MemoryAccess::Load,
                    addr,
                    required: 8,
                }
                .into());
            }
            hart.amo_rsv = Some(addr);
            reg!(lr_d.rd(inst), load!(u64, addr));
        }
        Rv64IMASC::ScD(sc_d) => {
            let addr = reg!(sc_d.rs1(inst));
            if addr & 7 != 0 {
                fail!(MemoryError::UnalignedMemoryAccess {
                    access: MemoryAccess::Store,
                    addr,
                    required: 8,
                }
                .into());
            }
            if hart.amo_rsv.take() == Some(addr) {
                store!(u64, addr, reg!(sc_d.rs2(inst)));
                reg!(sc_d.rd(inst), 0);
            } else {
                reg!(sc_d.rd(inst), 1);
            }
        }
        Rv64IMASC::AmoswapD(swap) => amo_d_op!(|swap, old, rs2| rs2),
        Rv64IMASC::AmoaddD(add) => amo_d_op!(|add, old, rs2| old.wrapping_add(rs2)),
        Rv64IMASC::AmoxorD(xor) => amo_d_op!(|xor, old, rs2| old ^ rs2),
        Rv64IMASC::AmoorD(or) => amo_d_op!(|or, old, rs2| old | rs2),
        Rv64IMASC::AmoandD(and) => amo_d_op!(|and, old, rs2| old & rs2),
        Rv64IMASC::AmominD(min) => amo_d_op!(|min, old, rs2| (old as i64).min(rs2 as i64)),
        Rv64IMASC::AmomaxD(max) => amo_d_op!(|max, old, rs2| (old as i64).max(rs2 as i64)),
        Rv64IMASC::AmominuD(minu) => amo_d_op!(|minu, old, rs2| old.min(rs2)),
        Rv64IMASC::AmomaxuD(maxu) => amo_d_op!(|maxu, old, rs2| old.max(rs2)),

        // --- C ---
        Rv64IMASC::CAddi4spn(addi4spn) => {
            let imm = addi4spn.imm(inst);
            let rd = addi4spn.rd(inst);
            hart.set_reg(rd, reg!(Reg::Sp).wrapping_add(imm as u64));
        }
        Rv64IMASC::CLw(lw) => {
            let addr = reg!(lw.rs1(inst)).wrapping_add(lw.imm(inst) as u64);
            reg!(lw.rd(inst), load!(i32, addr) as i64);
        }
        Rv64IMASC::CSw(sw) => {
            let addr = reg!(sw.rs1(inst)).wrapping_add(sw.imm(inst) as u64);
            store!(u32, addr, reg!(sw.rs2(inst)) as u32);
        }
        Rv64IMASC::CLd(ld) => {
            let addr = reg!(ld.rs1(inst)).wrapping_add(ld.imm(inst) as u64);
            reg!(ld.rd(inst), load!(u64, addr));
        }
        Rv64IMASC::CSd(sd) => {
            let addr = reg!(sd.rs1(inst)).wrapping_add(sd.imm(inst) as u64);
            store!(u64, addr, reg!(sd.rs2(inst)));
        }
        Rv64IMASC::CAddi(caddi) => {
            let rs1rd = caddi.rs1rd(inst);
            hart.set_reg(
                rs1rd,
                reg!(rs1rd).wrapping_add_signed(caddi.imm(inst) as i64),
            );
        }
        Rv64IMASC::CAddiw(caddiw) => {
            let rs1rd = caddiw.rs1rd(inst);
            let res = (reg!(rs1rd) as u32).wrapping_add_signed(caddiw.imm(inst));
            hart.set_reg(rs1rd, res as i32 as i64 as u64);
        }
        Rv64IMASC::CAddi16sp(caddi16sp) => {
            let imm = caddi16sp.imm(inst);
            let rs1rd = caddi16sp.rs1rd(inst);
            hart.set_reg(rs1rd, reg!(Reg::Sp).wrapping_add_signed(imm as i64));
        }
        Rv64IMASC::CLwsp(lwsp) => {
            let addr = reg!(Reg::Sp).wrapping_add(lwsp.imm(inst) as u64);
            reg!(lwsp.rd(inst), load!(i32, addr) as i64);
        }
        Rv64IMASC::CSwsp(swsp) => {
            let addr = reg!(Reg::Sp).wrapping_add(swsp.imm(inst) as u64);
            store!(u32, addr, reg!(swsp.rs2(inst)) as u32);
        }
        Rv64IMASC::CLdsp(ldsp) => {
            let addr = reg!(Reg::Sp).wrapping_add(ldsp.imm(inst) as u64);
            reg!(ldsp.rd(inst), load!(u64, addr));
        }
        Rv64IMASC::CSdsp(sdsp) => {
            let addr = reg!(Reg::Sp).wrapping_add(sdsp.imm(inst) as u64);
            store!(u64, addr, reg!(sdsp.rs2(inst)));
        }
        Rv64IMASC::CNop(_) => {}
        Rv64IMASC::CLi(cli) => reg!(cli.rs1rd(inst), cli.imm(inst) as i64),
        Rv64IMASC::CLui(clui) => reg!(clui.rd(inst), clui.imm(inst) as i64),
        Rv64IMASC::CSrli(csrli) => {
            let rd = csrli.rs1rd(inst);
            reg!(rd, reg!(rd) >> csrli.shamt(inst));
        }
        Rv64IMASC::CSrai(csrai) => {
            let rd = csrai.rs1rd(inst);
            reg!(rd, (reg!(rd) as i64) >> csrai.shamt(inst));
        }
        Rv64IMASC::CAndi(candi) => {
            let rd = candi.rs1rd(inst);
            reg!(rd, reg!(rd) & candi.imm(inst) as u64);
        }
        Rv64IMASC::CSub(csub) => {
            let rs1rd = csub.rs1rd(inst);
            let rs2 = reg!(csub.rs2(inst));
            reg!(rs1rd, reg!(rs1rd).wrapping_sub(rs2));
        }
        Rv64IMASC::CXor(cxor) => {
            let rs1rd = cxor.rs1rd(inst);
            let rs2 = reg!(cxor.rs2(inst));
            reg!(rs1rd, reg!(rs1rd) ^ rs2);
        }
        Rv64IMASC::COr(cor) => {
            let rs1rd = cor.rs1rd(inst);
            let rs2 = reg!(cor.rs2(inst));
            reg!(rs1rd, reg!(rs1rd) | rs2);
        }
        Rv64IMASC::CAnd(cand) => {
            let rs1rd = cand.rs1rd(inst);
            let rs2 = reg!(cand.rs2(inst));
            reg!(rs1rd, reg!(rs1rd) & rs2);
        }
        Rv64IMASC::CSubw(csubw) => {
            let rs1rd = csubw.rs1rd(inst);
            let rs2 = reg!(csubw.rs2(inst));
            let res = (reg!(rs1rd) as u32).wrapping_sub(rs2 as u32);
            hart.set_reg(rs1rd, res as i32 as i64 as u64);
        }
        Rv64IMASC::CAddw(caddw) => {
            let rs1rd = caddw.rs1rd(inst);
            let rs2 = reg!(caddw.rs2(inst));
            let res = (reg!(rs1rd) as u32).wrapping_add(rs2 as u32);
            hart.set_reg(rs1rd, res as i32 as i64 as u64);
        }
        Rv64IMASC::CJ(cj) => {
            next_pc = pc.wrapping_add_signed(cj.imm(inst) as i64);
        }
        Rv64IMASC::CBeqz(cbeqz) => {
            if reg!(cbeqz.rs1(inst)) == 0 {
                next_pc = pc.wrapping_add_signed(cbeqz.imm(inst) as i64);
            }
        }
        Rv64IMASC::CBnez(cbnez) => {
            if reg!(cbnez.rs1(inst)) != 0 {
                next_pc = pc.wrapping_add_signed(cbnez.imm(inst) as i64);
            }
        }
        Rv64IMASC::CSlli(cslli) => {
            let rd = cslli.rs1rd(inst);
            reg!(rd, reg!(rd) << cslli.shamt(inst));
        }
        Rv64IMASC::CJr(cjr) => {
            next_pc = reg!(cjr.rs1(inst));
        }
        Rv64IMASC::CMv(cmv) => reg!(cmv.rd(inst), reg!(cmv.rs2(inst))),
        Rv64IMASC::CEbreak(_) => return Exec::Ebreak,
        Rv64IMASC::CJalr(cjalr) => {
            reg!(Reg::Ra, next_pc);
            next_pc = reg!(cjalr.rs1(inst));
        }
        Rv64IMASC::CAdd(cadd) => {
            let rs1rd = cadd.rs1rd(inst);
            reg!(rs1rd, reg!(rs1rd).wrapping_add(reg!(cadd.rs2(inst))));
        }
        Rv64IMASC::CUnimp(_) => fail!(HartError::illegal(pc, inst).into()),
    }

    hart.pc = next_pc;
    *pc_out = next_pc;

    Exec::Next
}
