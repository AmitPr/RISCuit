use riscv_inst::lookup::Opcode::*;

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
    const fn get_reg(&self, r: u32) -> u32 {
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
    #[allow(unused)]
    pub fn run(&mut self) {
        if !self.running {
            return;
        }

        let inst = self.mem.load::<u32>(self.pc);
        let (op, inc) = riscv_inst::lookup::decode(inst);
        let op = op.unwrap_or_else(|| panic!("Invalid instruction: {:08x}", inst));

        let mut next_pc = self.pc.wrapping_add(inc);
        println!("0x{:08x}:\t{inst:08x}\t{op:?}", self.pc,);
        // println!("0x{:08x}:\t{inst:08x}\n\t{op:?}\n\t{try_lookup:?}", self.pc);

        macro_rules! reg_imm_op {
            (|$inst:ident.$rs1:ident, $inst2:ident.$imm:ident| $body:expr) => {{
                let $rs1 = self.get_reg($inst.$rs1());
                let $imm = $inst2.$imm();
                let res = { $body };
                self.set_reg($inst.rd(), res as u32);
            }};
        }

        macro_rules! imm_op {
            (|$inst:ident.$imm:ident| $body:expr) => {{
                let $imm = $inst.$imm();
                let res = { $body };
                self.set_reg($inst.rd(), res as u32);
            }};
        }

        macro_rules! reg_reg_op {
            (|$inst:ident.$rs1:ident, $inst2:ident.$rs2:ident| $body:expr) => {{
                let $rs1 = self.get_reg($inst.$rs1());
                let $rs2 = self.get_reg($inst2.$rs2());
                let res = { $body };
                self.set_reg($inst.rd(), res as u32);
            }};
        }

        macro_rules! store_op {
            (|$inst:ident.$rs1:ident, $inst2:ident.$rs2:ident, $addr:ident| $body:expr) => {{
                let $rs1 = self.get_reg($inst.$rs1());
                let $rs2 = self.get_reg($inst2.$rs2());
                let $addr = $rs1.wrapping_add_signed($inst.imm());
                $body;
            }};
        }

        macro_rules! branch_op {
            (|$inst:ident.$rs1:ident, $inst2:ident.$rs2:ident| $body:expr) => {{
                let $rs1 = self.get_reg($inst.$rs1());
                let $rs2 = self.get_reg($inst2.$rs2());
                let offset = $inst.imm();
                if $body {
                    next_pc = self.pc.wrapping_add_signed(offset);
                }
            }};
        }

        match op {
            Lb(lb) => reg_imm_op!(
                |lb.rs1, lb.imm| self.mem.load::<i8>(rs1.wrapping_add_signed(imm)) as i32
            ),
            Lh(lh) => reg_imm_op!(
                |lh.rs1, lh.imm| self.mem.load::<i16>(rs1.wrapping_add_signed(imm)) as i32
            ),
            Lw(lw) => reg_imm_op!(
                |lw.rs1, lw.imm| self.mem.load::<u32>(rs1.wrapping_add_signed(imm))
            ),
            Ld(ld) => todo!(),
            Lq(lq) => todo!(),
            Lbu(lbu) => reg_imm_op!(
                |lbu.rs1, lbu.imm| self.mem.load::<u8>(rs1.wrapping_add_signed(imm))
            ),
            Lhu(lhu) => reg_imm_op!(
                |lhu.rs1, lhu.imm| self.mem.load::<u16>(rs1.wrapping_add_signed(imm))
            ),
            Lwu(lwu) => reg_imm_op!(
                |lwu.rs1, lwu.imm| self.mem.load::<u32>(rs1.wrapping_add_signed(imm))
            ),
            Ldu(ldu) => todo!(),
            Flw(flw) => todo!(),
            Fld(fld) => todo!(),
            Flq(flq) => todo!(),
            Fence(fence) => todo!(),
            FenceI(fence_i) => todo!(),
            Addi(addi) => reg_imm_op!(|addi.rs1, addi.imm| rs1.wrapping_add_signed(imm)),
            Slti(slti) => reg_imm_op!(|slti.rs1, slti.imm| (rs1 as i32) < imm),
            Sltiu(sltiu) => reg_imm_op!(|sltiu.rs1, sltiu.imm| rs1 < (imm as u32)),
            Xori(xori) => reg_imm_op!(|xori.rs1, xori.imm| rs1 ^ (imm as u32)),
            Ori(ori) => reg_imm_op!(|ori.rs1, ori.imm| rs1 | (imm as u32)),
            Andi(andi) => reg_imm_op!(|andi.rs1, andi.imm| rs1 & (imm as u32)),
            Slli(slli) => reg_imm_op!(|slli.rs1, slli.shamt| rs1 << shamt),
            Srli(srli) => reg_imm_op!(|srli.rs1, srli.shamt| rs1 >> shamt),
            Srai(srai) => reg_imm_op!(|srai.rs1, srai.shamt| rs1 as i32 >> shamt),
            Auipc(auipc) => imm_op!(|auipc.imm| self.pc.wrapping_add_signed(imm as i32)),
            Addiw(addiw) => todo!(),
            Slliw(slliw) => todo!(),
            Srliw(srliw) => todo!(),
            Sraiw(sraiw) => todo!(),
            Sb(sb) => store_op!(|sb.rs1, sb.rs2, addr| self.mem.store::<u8>(addr, rs2 as u8)),
            Sh(sh) => store_op!(|sh.rs1, sh.rs2, addr| self.mem.store::<u16>(addr, rs2 as u16)),
            Sw(sw) => store_op!(|sw.rs1, sw.rs2, addr| self.mem.store::<u32>(addr, rs2)),
            Sd(sd) => todo!(),
            Sq(sq) => todo!(),
            Fsw(fsw) => todo!(),
            Fsd(fsd) => todo!(),
            Fsq(fsq) => todo!(),
            LrW(lr_w) => todo!(),
            ScW(sc_w) => todo!(),
            AmoswapW(amoswap_w) => todo!(),
            AmoaddW(amoadd_w) => todo!(),
            AmoxorW(amoxor_w) => todo!(),
            AmoandW(amoand_w) => todo!(),
            AmoorW(amoor_w) => todo!(),
            AmominW(amomin_w) => todo!(),
            AmomaxW(amomax_w) => todo!(),
            AmominuW(amominu_w) => todo!(),
            AmomaxuW(amomaxu_w) => todo!(),
            LrD(lr_d) => todo!(),
            ScD(sc_d) => todo!(),
            AmoswapD(amoswap_d) => todo!(),
            AmoaddD(amoadd_d) => todo!(),
            AmoxorD(amoxor_d) => todo!(),
            AmoandD(amoand_d) => todo!(),
            AmoorD(amoor_d) => todo!(),
            AmominD(amomin_d) => todo!(),
            AmomaxD(amomax_d) => todo!(),
            AmominuD(amominu_d) => todo!(),
            AmomaxuD(amomaxu_d) => todo!(),
            LrQ(lr_q) => todo!(),
            ScQ(sc_q) => todo!(),
            AmoswapQ(amoswap_q) => todo!(),
            AmoaddQ(amoadd_q) => todo!(),
            AmoxorQ(amoxor_q) => todo!(),
            AmoandQ(amoand_q) => todo!(),
            AmoorQ(amoor_q) => todo!(),
            AmominQ(amomin_q) => todo!(),
            AmomaxQ(amomax_q) => todo!(),
            AmominuQ(amominu_q) => todo!(),
            AmomaxuQ(amomaxu_q) => todo!(),
            Add(add) => reg_reg_op!(|add.rs1, add.rs2| rs1.wrapping_add(rs2)),
            Sub(sub) => reg_reg_op!(|sub.rs1, sub.rs2| rs1.wrapping_sub(rs2)),
            Sll(sll) => reg_reg_op!(|sll.rs1, sll.rs2| rs1 << (rs2 & 0x1f)),
            Slt(slt) => reg_reg_op!(|slt.rs1, slt.rs2| (rs1 as i32) < (rs2 as i32)),
            Sltu(sltu) => reg_reg_op!(|sltu.rs1, sltu.rs2| rs1 < rs2),
            Xor(xor) => reg_reg_op!(|xor.rs1, xor.rs2| rs1 ^ rs2),
            Srl(srl) => reg_reg_op!(|srl.rs1, srl.rs2| rs1 >> (rs2 & 0x1f)),
            Sra(sra) => reg_reg_op!(|sra.rs1, sra.rs2| (rs1 as i32 >> (rs2 & 0x1f)) as u32),
            Or(or) => reg_reg_op!(|or.rs1, or.rs2| rs1 | rs2),
            And(and) => reg_reg_op!(|and.rs1, and.rs2| rs1 & rs2),
            Mul(mul) => reg_reg_op!(|mul.rs1, mul.rs2| rs1.wrapping_mul(rs2)),
            Mulh(mulh) => reg_reg_op!(
                |mulh.rs1, mulh.rs2| ((rs1 as i64 * rs2 as i64) >> 32) as u32
            ),
            Mulhsu(mulhsu) => reg_reg_op!(
                |mulhsu.rs1, mulhsu.rs2| (((rs1 as i32 as i64) * (rs2 as i64)) >> 32) as u32
            ),
            Mulhu(mulhu) => reg_reg_op!(
                |mulhu.rs1, mulhu.rs2| ((rs1 as u64 * rs2 as u64) >> 32) as u32
            ),
            Div(div) => reg_reg_op!(
                |div.rs1, div.rs2| {
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
                }
            ),
            Divu(divu) => reg_reg_op!(
                |divu.rs1, divu.rs2| {
                    let rs2 = rs2 as u32;
                    if rs2 == 0 {
                        // Division by zero returns MAX
                        u32::MAX
                    } else {
                        rs1.wrapping_div(rs2)
                    }
                }
            ),
            Rem(rem) => reg_reg_op!(
                |rem.rs1, rem.rs2| {
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
                }
            ),
            Remu(remu) => reg_reg_op!(
                |remu.rs1, remu.rs2| {
                    let rs2 = rs2 as u32;
                    if rs2 == 0 {
                        // Remainder of division by zero returns the dividend
                        rs1
                    } else {
                        rs1.wrapping_rem(rs2)
                    }
                }
            ),
            Lui(lui) => imm_op!(|lui.imm| imm),
            Addw(addw) => todo!(),
            Subw(subw) => todo!(),
            Sllw(sllw) => todo!(),
            Srlw(srlw) => todo!(),
            Sraw(sraw) => todo!(),
            Mulw(mulw) => todo!(),
            Divw(divw) => todo!(),
            Divuw(divuw) => todo!(),
            Remw(remw) => todo!(),
            Remuw(remuw) => todo!(),
            FmaddS(fmadd_s) => todo!(),
            FmsubS(fmsub_s) => todo!(),
            FnmsubS(fnmsub_s) => todo!(),
            FnmaddS(fnmadd_s) => todo!(),
            FaddS(fadd_s) => todo!(),
            FsubS(fsub_s) => todo!(),
            FmulS(fmul_s) => todo!(),
            FdivS(fdiv_s) => todo!(),
            FsgnjS(fsgnj_s) => todo!(),
            FsgnjnS(fsgnjn_s) => todo!(),
            FsgnjxS(fsgnjx_s) => todo!(),
            FminS(fmin_s) => todo!(),
            FmaxS(fmax_s) => todo!(),
            FsqrtS(fsqrt_s) => todo!(),
            FleS(fle_s) => todo!(),
            FltS(flt_s) => todo!(),
            FeqS(feq_s) => todo!(),
            FcvtWS(fcvt_ws) => todo!(),
            FcvtWuS(fcvt_wu_s) => todo!(),
            FcvtSW(fcvt_sw) => todo!(),
            FcvtSWu(fcvt_swu) => todo!(),
            FmvXS(fmv_xs) => todo!(),
            FclassS(fclass_s) => todo!(),
            FmvSX(fmv_sx) => todo!(),
            FcvtLS(fcvt_ls) => todo!(),
            FcvtLuS(fcvt_lu_s) => todo!(),
            FcvtSL(fcvt_sl) => todo!(),
            FcvtSLu(fcvt_slu) => todo!(),
            FmaddD(fmadd_d) => todo!(),
            FmsubD(fmsub_d) => todo!(),
            FnmsubD(fnmsub_d) => todo!(),
            FnmaddD(fnmadd_d) => todo!(),
            FaddD(fadd_d) => todo!(),
            FsubD(fsub_d) => todo!(),
            FmulD(fmul_d) => todo!(),
            FdivD(fdiv_d) => todo!(),
            FsgnjD(fsgnj_d) => todo!(),
            FsgnjnD(fsgnjn_d) => todo!(),
            FsgnjxD(fsgnjx_d) => todo!(),
            FminD(fmin_d) => todo!(),
            FmaxD(fmax_d) => todo!(),
            FsqrtD(fsqrt_d) => todo!(),
            FleD(fle_d) => todo!(),
            FltD(flt_d) => todo!(),
            FeqD(feq_d) => todo!(),
            FcvtWD(fcvt_wd) => todo!(),
            FcvtWuD(fcvt_wu_d) => todo!(),
            FcvtDW(fcvt_dw) => todo!(),
            FcvtDWu(fcvt_dwu) => todo!(),
            FcvtSD(fcvt_sd) => todo!(),
            FcvtDS(fcvt_ds) => todo!(),
            FclassD(fclass_d) => todo!(),
            FcvtLD(fcvt_ld) => todo!(),
            FcvtLuD(fcvt_lu_d) => todo!(),
            FmvXD(fmv_xd) => todo!(),
            FmvDX(fmv_dx) => todo!(),
            FcvtDL(fcvt_dl) => todo!(),
            FcvtDLu(fcvt_dlu) => todo!(),
            FmaddQ(fmadd_q) => todo!(),
            FmsubQ(fmsub_q) => todo!(),
            FnmsubQ(fnmsub_q) => todo!(),
            FnmaddQ(fnmadd_q) => todo!(),
            FaddQ(fadd_q) => todo!(),
            FsubQ(fsub_q) => todo!(),
            FmulQ(fmul_q) => todo!(),
            FdivQ(fdiv_q) => todo!(),
            FsgnjQ(fsgnj_q) => todo!(),
            FsgnjnQ(fsgnjn_q) => todo!(),
            FsgnjxQ(fsgnjx_q) => todo!(),
            FminQ(fmin_q) => todo!(),
            FmaxQ(fmax_q) => todo!(),
            FsqrtQ(fsqrt_q) => todo!(),
            FleQ(fle_q) => todo!(),
            FltQ(flt_q) => todo!(),
            FeqQ(feq_q) => todo!(),
            FcvtWQ(fcvt_wq) => todo!(),
            FcvtWuQ(fcvt_wu_q) => todo!(),
            FcvtQW(fcvt_qw) => todo!(),
            FcvtQWu(fcvt_qwu) => todo!(),
            FcvtSQ(fcvt_sq) => todo!(),
            FcvtQS(fcvt_qs) => todo!(),
            FcvtDQ(fcvt_dq) => todo!(),
            FcvtQD(fcvt_qd) => todo!(),
            FclassQ(fclass_q) => todo!(),
            FcvtLQ(fcvt_lq) => todo!(),
            FcvtLuQ(fcvt_lu_q) => todo!(),
            FcvtQL(fcvt_ql) => todo!(),
            FcvtQLu(fcvt_qlu) => todo!(),
            FmvXQ(fmv_xq) => todo!(),
            FmvQX(fmv_qx) => todo!(),
            Addid(addid) => todo!(),
            Sllid(sllid) => todo!(),
            Srlid(srlid) => todo!(),
            Sraid(sraid) => todo!(),
            Beq(beq) => branch_op!(|beq.rs1, beq.rs2| rs1 == rs2),
            Bne(bne) => branch_op!(|bne.rs1, bne.rs2| rs1 != rs2),
            Blt(blt) => branch_op!(|blt.rs1, blt.rs2| (rs1 as i32) < (rs2 as i32)),
            Bge(bge) => branch_op!(|bge.rs1, bge.rs2| (rs1 as i32) >= (rs2 as i32)),
            Bltu(bltu) => branch_op!(|bltu.rs1, bltu.rs2| rs1 < rs2),
            Bgeu(bgeu) => branch_op!(|bgeu.rs1, bgeu.rs2| rs1 >= rs2),
            Jalr(jalr) => reg_imm_op!(|jalr.rs1, jalr.imm| {
                let res = next_pc;
                next_pc = rs1.wrapping_add_signed(imm);
                res
            }),
            Jal(jal) => imm_op!(|jal.imm| {
                let res = next_pc;
                next_pc = self.pc.wrapping_add_signed(imm as i32);
                res
            }),
            Ecall(ecall) => {
                self.syscall(ecall.rs1(), ecall.imm());
            }
            Ebreak(ebreak) => {
                self.syscall(ebreak.rs1(), ebreak.imm());
            }
            Addd(addd) => todo!(),
            Subd(subd) => todo!(),
            Slld(slld) => todo!(),
            Srld(srld) => todo!(),
            Srad(srad) => todo!(),
            Muld(muld) => todo!(),
            Divd(divd) => todo!(),
            Divud(divud) => todo!(),
            Remd(remd) => todo!(),
            Remud(remud) => todo!(),
            _ => panic!("Unsupported instruction: {:?}", op),
        }

        self.pc = next_pc;

        self.run();
    }

    pub fn syscall(&mut self, _rs1: u32, imm: i32) {
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
                        let b = self.mem.load::<u8>(addr);
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
}
