//! RV32IMASC execution.
use riscv_inst::codegen::rv32imasc::Rv32IMASC;
use riscv_inst::Reg;

use super::{mem_fault, take_err, Exec, Execute, Hart, X32};
use crate::{
    error::{HartError, MachineError, MemoryAccess, MemoryError},
    machine::{Kernel, StepResult},
    memory::Memory,
};

impl Execute for X32 {
    fn run<K: Kernel<Xlen = Self>>(
        hart: &mut Hart<Self>,
        mem: &mut K::Memory,
        kernel: &mut K,
    ) -> Result<(), MachineError<K::Error>> {
        // pc and the instruction count live in registers: hart.pc is stored
        // per instruction but never reloaded (a per-instruction RMW is a
        // loop-carried dependence); inst_count is flushed before kernel
        // entry and on exit, so the kernel always observes an exact count.
        let mut pc = hart.pc;
        let mut count = 0u64;
        let mut err = None;
        let result = loop {
            let Ok(inst) = mem.load::<u32>(pc) else {
                break Err(mem_fault(MemoryAccess::Load, pc as u64));
            };
            let Some(op) = Rv32IMASC::parse(inst) else {
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
        let op = Rv32IMASC::parse(inst).ok_or_else(|| HartError::invalid(pc, inst))?;

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
fn exec_op_at<M: Memory<Addr = u32>, E: std::error::Error>(
    hart: &mut Hart<X32>,
    op: Rv32IMASC,
    inst: u32,
    pc: &mut u32,
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
            hart.set_reg($reg, $val as u32)
        };
    }

    macro_rules! reg_imm_op {
        (|$inst:ident.$rs1:ident, $inst2:ident.$imm:ident| $body:expr) => {{
            let $rs1 = reg!($inst.$rs1(inst));
            let $imm = $inst2.$imm(inst);
            reg!($inst.rd(inst), { $body } as u32);
        }};
    }

    macro_rules! imm_op {
        (|$inst:ident.$imm:ident| $body:expr) => {{
            let $imm = $inst.$imm(inst);
            reg!($inst.rd(inst), { $body } as u32);
        }};
    }

    macro_rules! reg_reg_op {
        (|$inst:ident.$rs1:ident, $inst2:ident.$rs2:ident| $body:expr) => {{
            let $rs1 = reg!($inst.$rs1(inst));
            let $rs2 = reg!($inst2.$rs2(inst));
            reg!($inst.rd(inst), { $body } as u32);
        }};
    }

    macro_rules! store_op {
        (|$inst:ident.$rs1:ident, $inst2:ident.$rs2:ident, $addr:ident| $body:expr) => {{
            let $rs1 = reg!($inst.$rs1(inst));
            let $rs2 = reg!($inst2.$rs2(inst));
            let $addr = $rs1.wrapping_add_signed($inst.imm(inst));
            $body;
        }};
    }

    macro_rules! branch_op {
        (|$inst:ident.$rs1:ident, $inst2:ident.$rs2:ident| $body:expr) => {{
            let $rs1 = reg!($inst.$rs1(inst));
            let $rs2 = reg!($inst2.$rs2(inst));
            let offset = $inst.imm(inst);
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
            let $imm = $inst2.$imm(inst);
            reg!($inst.rd(inst), hart.csrs[$csr]);
            $body;
        }};
    }

    macro_rules! amo_op {
        (|$inst:ident, $old:ident, $rs2:ident| $body:expr) => {{
            let addr = reg!($inst.rs1(inst));
            if addr & 3 != 0 {
                fail!(MemoryError::UnalignedMemoryAccess {
                    access: MemoryAccess::Load,
                    addr: addr as u64,
                    required: 4,
                }
                .into());
            }
            let $old = load!(u32, addr);
            reg!($inst.rd(inst), $old);
            let $rs2 = reg!($inst.rs2(inst));
            store!(u32, addr, $body as u32);
        }};
    }

    match op {
        // --- RV32I ---
        Rv32IMASC::Lui(lui) => imm_op!(|lui.imm| imm),
        Rv32IMASC::Auipc(auipc) => imm_op!(|auipc.imm| pc.wrapping_add_signed(imm)),
        Rv32IMASC::Jal(jal) => imm_op!(|jal.imm| {
            let res = next_pc;
            next_pc = pc.wrapping_add_signed(imm);
            res
        }),
        Rv32IMASC::Jalr(jalr) => reg_imm_op!(|jalr.rs1, jalr.imm| {
            let res = next_pc;
            next_pc = rs1.wrapping_add_signed(imm);
            res
        }),
        Rv32IMASC::Beq(beq) => branch_op!(|beq.rs1, beq.rs2| rs1 == rs2),
        Rv32IMASC::Bne(bne) => branch_op!(|bne.rs1, bne.rs2| rs1 != rs2),
        Rv32IMASC::Blt(blt) => branch_op!(|blt.rs1, blt.rs2| (rs1 as i32) < (rs2 as i32)),
        Rv32IMASC::Bge(bge) => branch_op!(|bge.rs1, bge.rs2| (rs1 as i32) >= (rs2 as i32)),
        Rv32IMASC::Bltu(bltu) => branch_op!(|bltu.rs1, bltu.rs2| rs1 < rs2),
        Rv32IMASC::Bgeu(bgeu) => branch_op!(|bgeu.rs1, bgeu.rs2| rs1 >= rs2),
        Rv32IMASC::Lb(lb) => {
            reg_imm_op!(|lb.rs1, lb.imm| load!(i8, rs1.wrapping_add_signed(imm)) as i32)
        }
        Rv32IMASC::Lh(lh) => {
            reg_imm_op!(|lh.rs1, lh.imm| load!(i16, rs1.wrapping_add_signed(imm)) as i32)
        }
        Rv32IMASC::Lw(lw) => {
            reg_imm_op!(|lw.rs1, lw.imm| load!(u32, rs1.wrapping_add_signed(imm)))
        }
        Rv32IMASC::Lbu(lbu) => {
            reg_imm_op!(|lbu.rs1, lbu.imm| load!(u8, rs1.wrapping_add_signed(imm)))
        }
        Rv32IMASC::Lhu(lhu) => {
            reg_imm_op!(|lhu.rs1, lhu.imm| load!(u16, rs1.wrapping_add_signed(imm)))
        }
        Rv32IMASC::Sb(sb) => {
            store_op!(|sb.rs1, sb.rs2, addr| store!(u8, addr, rs2 as u8))
        }
        Rv32IMASC::Sh(sh) => {
            store_op!(|sh.rs1, sh.rs2, addr| store!(u16, addr, rs2 as u16))
        }
        Rv32IMASC::Sw(sw) => {
            store_op!(|sw.rs1, sw.rs2, addr| store!(u32, addr, rs2))
        }
        Rv32IMASC::Addi(addi) => {
            reg_imm_op!(|addi.rs1, addi.imm| rs1.wrapping_add_signed(imm))
        }
        Rv32IMASC::Slti(slti) => reg_imm_op!(|slti.rs1, slti.imm| (rs1 as i32) < imm),
        Rv32IMASC::Sltiu(sltiu) => reg_imm_op!(|sltiu.rs1, sltiu.imm| rs1 < (imm as u32)),
        Rv32IMASC::Xori(xori) => reg_imm_op!(|xori.rs1, xori.imm| rs1 ^ (imm as u32)),
        Rv32IMASC::Ori(ori) => reg_imm_op!(|ori.rs1, ori.imm| rs1 | (imm as u32)),
        Rv32IMASC::Andi(andi) => reg_imm_op!(|andi.rs1, andi.imm| rs1 & (imm as u32)),
        Rv32IMASC::Slli(slli) => reg_imm_op!(|slli.rs1, slli.shamt| rs1 << shamt),
        Rv32IMASC::Srli(srli) => reg_imm_op!(|srli.rs1, srli.shamt| rs1 >> shamt),
        Rv32IMASC::Srai(srai) => reg_imm_op!(|srai.rs1, srai.shamt| rs1 as i32 >> shamt),
        Rv32IMASC::Add(add) => reg_reg_op!(|add.rs1, add.rs2| rs1.wrapping_add(rs2)),
        Rv32IMASC::Sub(sub) => reg_reg_op!(|sub.rs1, sub.rs2| rs1.wrapping_sub(rs2)),
        Rv32IMASC::Sll(sll) => reg_reg_op!(|sll.rs1, sll.rs2| rs1 << (rs2 & 0x1f)),
        Rv32IMASC::Slt(slt) => reg_reg_op!(|slt.rs1, slt.rs2| (rs1 as i32) < (rs2 as i32)),
        Rv32IMASC::Sltu(sltu) => reg_reg_op!(|sltu.rs1, sltu.rs2| rs1 < rs2),
        Rv32IMASC::Xor(xor) => reg_reg_op!(|xor.rs1, xor.rs2| rs1 ^ rs2),
        Rv32IMASC::Srl(srl) => reg_reg_op!(|srl.rs1, srl.rs2| rs1 >> (rs2 & 0x1f)),
        Rv32IMASC::Sra(sra) => {
            reg_reg_op!(|sra.rs1, sra.rs2| (rs1 as i32 >> (rs2 & 0x1f)) as u32)
        }
        Rv32IMASC::Or(or) => reg_reg_op!(|or.rs1, or.rs2| rs1 | rs2),
        Rv32IMASC::And(and) => reg_reg_op!(|and.rs1, and.rs2| rs1 & rs2),
        Rv32IMASC::Fence(_) => {}
        Rv32IMASC::FenceI(_) => {}
        Rv32IMASC::Ecall(_) => return Exec::Syscall,
        Rv32IMASC::Ebreak(_) => return Exec::Ebreak,
        Rv32IMASC::Unimp(_) => fail!(HartError::illegal(pc, inst).into()),

        // --- M ---
        Rv32IMASC::Mul(mul) => reg_reg_op!(|mul.rs1, mul.rs2| rs1.wrapping_mul(rs2)),
        Rv32IMASC::Mulh(mulh) => reg_reg_op!(
            |mulh.rs1, mulh.rs2| (rs1 as i32 as i64).wrapping_mul(rs2 as i32 as i64) >> 32
        ),
        Rv32IMASC::Mulhsu(mulhsu) => reg_reg_op!(
            |mulhsu.rs1, mulhsu.rs2| (((rs1 as i32 as i64) * (rs2 as i64)) >> 32) as u32
        ),
        Rv32IMASC::Mulhu(mulhu) => {
            reg_reg_op!(|mulhu.rs1, mulhu.rs2| ((rs1 as u64 * rs2 as u64) >> 32) as u32)
        }
        Rv32IMASC::Div(div) => reg_reg_op!(|div.rs1, div.rs2| {
            let rs1 = rs1 as i32;
            let rs2 = rs2 as i32;
            if rs2 == 0 {
                // Division by zero returns -1
                u32::MAX
            } else if rs1 == i32::MIN && rs2 == -1 {
                // Handle signed division overflow
                rs1 as u32
            } else {
                rs1.wrapping_div(rs2) as u32
            }
        }),
        Rv32IMASC::Divu(divu) => reg_reg_op!(|divu.rs1, divu.rs2| {
            if rs2 == 0 {
                // Division by zero returns MAX
                u32::MAX
            } else {
                rs1.wrapping_div(rs2)
            }
        }),
        Rv32IMASC::Rem(rem) => reg_reg_op!(|rem.rs1, rem.rs2| {
            let rs1 = rs1 as i32;
            let rs2 = rs2 as i32;
            if rs2 == 0 {
                // Remainder of division by zero returns the dividend
                rs1 as u32
            } else if rs1 == i32::MIN && rs2 == -1 {
                // Handle signed division overflow - remainder is 0
                0
            } else {
                rs1.wrapping_rem(rs2) as u32
            }
        }),
        Rv32IMASC::Remu(remu) => reg_reg_op!(|remu.rs1, remu.rs2| {
            if rs2 == 0 {
                // Remainder of division by zero returns the dividend
                rs1
            } else {
                rs1.wrapping_rem(rs2)
            }
        }),

        // --- System ---
        Rv32IMASC::Uret(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv32IMASC::Sret(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv32IMASC::Hret(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv32IMASC::Mret(_) => {
            // TODO: Not erroring because the ISA tests use this.
            // But we haven't implemented privilege levels yet.
        }
        Rv32IMASC::Dret(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv32IMASC::SfenceVm(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv32IMASC::SfenceVma(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv32IMASC::Wfi(_) => fail!(HartError::unimplemented(pc, inst).into()),
        Rv32IMASC::Csrrw(rw) => csr_op!(|rw.csr12, rw.rs1| hart.csrs[csr12] = rs1),
        Rv32IMASC::Csrrs(rs) => csr_op!(|rs.csr12, rs.rs1| hart.csrs[csr12] |= rs1),
        Rv32IMASC::Csrrc(rc) => csr_op!(|rc.csr12, rc.rs1| hart.csrs[csr12] &= !rs1),
        Rv32IMASC::Csrrwi(wi) => csr_imm_op!(|wi.csr12, wi.imm| hart.csrs[csr12] = imm),
        Rv32IMASC::Csrrsi(ri) => csr_imm_op!(|ri.csr12, ri.imm| hart.csrs[csr12] |= imm),
        Rv32IMASC::Csrrci(ci) => csr_imm_op!(|ci.csr12, ci.imm| hart.csrs[csr12] &= !imm),

        // --- A ---
        // We don't care about reservation set on single-hart ( i think )
        Rv32IMASC::LrW(lr_w) => {
            let addr = reg!(lr_w.rs1(inst));
            if addr & 3 != 0 {
                fail!(MemoryError::UnalignedMemoryAccess {
                    access: MemoryAccess::Load,
                    addr: addr as u64,
                    required: 4,
                }
                .into());
            }
            // TODO: Not sure if this is how the spec defines the
            // "reservation set" for lr/sc
            hart.amo_rsv = Some(addr);
            reg!(lr_w.rd(inst), load!(u32, addr));
        }
        Rv32IMASC::ScW(sc_w) => {
            let addr = reg!(sc_w.rs1(inst));
            if addr & 3 != 0 {
                fail!(MemoryError::UnalignedMemoryAccess {
                    access: MemoryAccess::Store,
                    addr: addr as u64,
                    required: 4,
                }
                .into());
            }
            if hart.amo_rsv.take() == Some(addr) {
                store!(u32, addr, reg!(sc_w.rs2(inst)));
                reg!(sc_w.rd(inst), 0);
            } else {
                reg!(sc_w.rd(inst), 1);
            }
        }
        Rv32IMASC::AmoswapW(swap) => amo_op!(|swap, old, rs2| rs2),
        Rv32IMASC::AmoaddW(add) => amo_op!(|add, old, rs2| old.wrapping_add(rs2)),
        Rv32IMASC::AmoxorW(xor) => amo_op!(|xor, old, rs2| old ^ rs2),
        Rv32IMASC::AmoorW(or) => amo_op!(|or, old, rs2| old | rs2),
        Rv32IMASC::AmoandW(and) => amo_op!(|and, old, rs2| old & rs2),
        Rv32IMASC::AmominW(min) => amo_op!(|min, old, rs2| (old as i32).min(rs2 as i32)),
        Rv32IMASC::AmomaxW(max) => amo_op!(|max, old, rs2| (old as i32).max(rs2 as i32)),
        Rv32IMASC::AmominuW(minu) => amo_op!(|minu, old, rs2| old.min(rs2)),
        Rv32IMASC::AmomaxuW(maxu) => amo_op!(|maxu, old, rs2| old.max(rs2)),

        // --- C ---
        Rv32IMASC::CAddi4spn(addi4spn) => {
            let imm = addi4spn.imm(inst);
            let rd = addi4spn.rd(inst);
            hart.set_reg(rd, reg!(Reg::Sp).wrapping_add(imm));
        }
        Rv32IMASC::CLw(lw) => {
            let addr = reg!(lw.rs1(inst)).wrapping_add(lw.imm(inst));
            reg!(lw.rd(inst), load!(u32, addr));
        }
        Rv32IMASC::CSw(sw) => {
            let addr = reg!(sw.rs1(inst)).wrapping_add(sw.imm(inst));
            store!(u32, addr, reg!(sw.rs2(inst)));
        }
        Rv32IMASC::CAddi(caddi) => {
            let rs1rd = caddi.rs1rd(inst);
            hart.set_reg(rs1rd, reg!(rs1rd).wrapping_add_signed(caddi.imm(inst)));
        }
        Rv32IMASC::CAddi16sp(caddi16sp) => {
            let imm = caddi16sp.imm(inst);
            let rs1rd = caddi16sp.rs1rd(inst);
            hart.set_reg(rs1rd, reg!(Reg::Sp).wrapping_add_signed(imm));
        }
        Rv32IMASC::CLwsp(lwsp) => {
            let addr = reg!(Reg::Sp).wrapping_add(lwsp.imm(inst));
            reg!(lwsp.rd(inst), load!(u32, addr));
        }
        Rv32IMASC::CSwsp(swsp) => {
            let addr = reg!(Reg::Sp).wrapping_add(swsp.imm(inst));
            store!(u32, addr, reg!(swsp.rs2(inst)));
        }
        Rv32IMASC::CNop(_) => {}
        Rv32IMASC::CJal(cjal) => {
            reg!(Reg::Ra, next_pc);
            next_pc = pc.wrapping_add_signed(cjal.imm(inst));
        }
        Rv32IMASC::CLi(cli) => reg!(cli.rs1rd(inst), cli.imm(inst)),
        Rv32IMASC::CLui(clui) => reg!(clui.rd(inst), clui.imm(inst)),
        Rv32IMASC::CSrli(csrli) => {
            let rd = csrli.rs1rd(inst);
            reg!(rd, reg!(rd) >> csrli.shamt(inst));
        }
        Rv32IMASC::CSrai(csrai) => {
            let rd = csrai.rs1rd(inst);
            reg!(rd, (reg!(rd) as i32) >> csrai.shamt(inst));
        }
        Rv32IMASC::CAndi(candi) => {
            let rd = candi.rs1rd(inst);
            reg!(rd, reg!(rd) & candi.imm(inst) as u32);
        }
        Rv32IMASC::CSub(csub) => {
            let rs1rd = csub.rs1rd(inst);
            let rs2 = reg!(csub.rs2(inst));
            reg!(rs1rd, reg!(rs1rd).wrapping_sub(rs2));
        }
        Rv32IMASC::CXor(cxor) => {
            let rs1rd = cxor.rs1rd(inst);
            let rs2 = reg!(cxor.rs2(inst));
            reg!(rs1rd, reg!(rs1rd) ^ rs2);
        }
        Rv32IMASC::COr(cor) => {
            let rs1rd = cor.rs1rd(inst);
            let rs2 = reg!(cor.rs2(inst));
            reg!(rs1rd, reg!(rs1rd) | rs2);
        }
        Rv32IMASC::CAnd(cand) => {
            let rs1rd = cand.rs1rd(inst);
            let rs2 = reg!(cand.rs2(inst));
            reg!(rs1rd, reg!(rs1rd) & rs2);
        }
        Rv32IMASC::CJ(cj) => {
            next_pc = pc.wrapping_add_signed(cj.imm(inst));
        }
        Rv32IMASC::CBeqz(cbeqz) => {
            if reg!(cbeqz.rs1(inst)) == 0 {
                next_pc = pc.wrapping_add_signed(cbeqz.imm(inst));
            }
        }
        Rv32IMASC::CBnez(cbnez) => {
            if reg!(cbnez.rs1(inst)) != 0 {
                next_pc = pc.wrapping_add_signed(cbnez.imm(inst));
            }
        }
        Rv32IMASC::CSlli(cslli) => {
            let rd = cslli.rs1rd(inst);
            reg!(rd, reg!(rd) << cslli.shamt(inst));
        }
        Rv32IMASC::CJr(cjr) => {
            next_pc = reg!(cjr.rs1(inst));
        }
        Rv32IMASC::CMv(cmv) => reg!(cmv.rd(inst), reg!(cmv.rs2(inst))),
        Rv32IMASC::CEbreak(_) => return Exec::Ebreak,
        Rv32IMASC::CJalr(cjalr) => {
            reg!(Reg::Ra, next_pc);
            next_pc = reg!(cjalr.rs1(inst));
        }
        Rv32IMASC::CAdd(cadd) => {
            let rs1rd = cadd.rs1rd(inst);
            reg!(rs1rd, reg!(rs1rd).wrapping_add(reg!(cadd.rs2(inst))));
        }
        Rv32IMASC::CUnimp(_) => fail!(HartError::illegal(pc, inst).into()),
    }

    hart.pc = next_pc;
    *pc_out = next_pc;

    Exec::Next
}
