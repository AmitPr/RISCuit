use riscv_inst::{codegen::rv32::Rv32, lookup::Opcode::*};

use crate::memory::Memory;

/// A simple CPU for RV32I instructions
pub struct Hart32 {
    regs: [u32; 32],
    pub pc: u32,
    pub mem: Memory,
    pub running: bool,
}

impl Hart32 {
    pub fn new() -> Self {
        Hart32 {
            regs: [0; 32],
            pc: 0,
            mem: Memory::new(),
            running: true,
        }
    }

    /// Read a register. x0 is always 0 in RISC-V.
    pub const fn get_reg(&self, r: u32) -> u32 {
        if r == 0 {
            0
        } else {
            self.regs[r as usize]
        }
    }

    /// Write a register (except x0).
    pub fn set_reg(&mut self, r: u32, val: u32) {
        if r != 0 {
            self.regs[r as usize] = val;
        }
    }

    /// Execute one instruction
    #[allow(unused)]
    pub fn run(&mut self, elf: goblin::elf::Elf<'_>) {
        let syms_by_pc: std::collections::HashMap<u32, &str> = elf
            .syms
            .iter()
            .filter_map(|sym| {
                if sym.is_function() {
                    elf.strtab
                        .get_at(sym.st_name)
                        .map(|name| (sym.st_value as u32, name))
                } else {
                    None
                }
            })
            .collect();
        let mut debug = false;
        while self.running {
            if let Some(sym) = syms_by_pc.get(&self.pc) {
                println!("0x{:08x}: {}", self.pc, sym);
                // wait for stdin
                let mut input = String::new();
                //std::io::stdin().read_line(&mut input).unwrap();
                match input.trim() {
                    "q" => {
                        self.running = false;
                        break;
                    }
                    "d" => {
                        debug = !debug;
                        if debug {
                            println!("Register states:");
                            for (i, reg) in self.regs.iter().enumerate() {
                                println!("x{:02}: 0x{:08x}", i, reg);
                            }
                        }
                    }
                    _ => {}
                }
            }

            let inst = self.mem.load::<u32>(self.pc);
            let op = Rv32::parse(inst);
            println!("0x{:08x}: {:08x} {:?}", self.pc, inst, op);
            let (op, inc) = riscv_inst::lookup::decode(inst);
            let op = op.unwrap_or_else(|| {
                panic!("Invalid instruction at 0x{:08x}: {:08x}", self.pc, inst)
            });

            let mut next_pc = self.pc.wrapping_add(inc);
            if debug {
                println!("0x{:08x}:\t{inst:08x}\t{op}", self.pc,);
            }

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
                Fence(fence) => {}
                FenceI(fence_i) => {}
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
                LrW(lr_w) => {
                    // We don't care about reservation set on single-hart ( i think )
                    let addr = self.get_reg(lr_w.rs1());
                    let value = self.mem.load::<u32>(addr);
                    self.set_reg(lr_w.rd(), value);
                }
                ScW(sc_w) => {
                    // Ignoring success/failure/reservation
                    let addr = self.get_reg(sc_w.rs1());
                    let value = self.get_reg(sc_w.rs2());
                    self.mem.store::<u32>(addr, value);
                    self.set_reg(sc_w.rd(), 0);
                }
                AmoswapW(amoswap_w) => {
                    let addr = self.get_reg(amoswap_w.rs1());
                    let rs2 = self.get_reg(amoswap_w.rs2());
                    let old = self.mem.load::<u32>(addr); // Load from memory
                    self.set_reg(amoswap_w.rd(), old); // Put old value in rd
                    self.mem.store::<u32>(addr, rs2); // Store new value to memory
                }
                AmoaddW(amoadd_w) => {
                    let addr = self.get_reg(amoadd_w.rs1());
                    let rs2 = self.get_reg(amoadd_w.rs2());
                    let old = self.mem.load::<u32>(addr); // Load from memory
                    self.set_reg(amoadd_w.rd(), old); // Put old value in rd
                    let result = (old as i32).wrapping_add(rs2 as i32) as u32;
                    self.mem.store::<u32>(addr, result); // Store sum back to memory
                }
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
                CAddi4spn(caddi4spn) => {
                    // addi rd', x2, imm
                    let rd = caddi4spn.crdq();
                    let x2 = self.get_reg(2);
                    self.set_reg(rd, x2.wrapping_add(caddi4spn.cimm4spn()));
                }
                CFld(cfld) => todo!(),
                CLw(clw) => {
                    // lw rd, offset(rs1)
                    let rd = clw.crdq();
                    let rs1 = self.get_reg(clw.crs1q());
                    let addr = rs1.wrapping_add(clw.cimmw());
                    self.set_reg(rd, self.mem.load::<u32>(addr));
                }
                CFlw(cflw) => todo!(),
                CFsd(cfsd) => todo!(),
                CSw(csw) => {
                    //sw rs2, offset(rs1)
                    let rs2 = self.get_reg(csw.crs2q());
                    let rs1 = self.get_reg(csw.crs1q());
                    let addr = rs1.wrapping_add(csw.cimmw());
                    self.mem.store::<u32>(addr, rs2);
                }
                CFsw(cfsw) => todo!(),
                CNop(cnop) => {}
                CAddi(caddi) => {
                    // addi rd, rd, imm
                    let rd = caddi.crs1rd();
                    let imm = caddi.cnzimmi();
                    self.set_reg(rd, self.get_reg(rd).wrapping_add_signed(imm));
                }
                CJal(cjal) => {
                    // jal x1, offset
                    let offset = cjal.cimmj();
                    self.set_reg(1, next_pc);
                    next_pc = self.pc.wrapping_add_signed(offset);
                }
                CLi(cli) => {
                    // addi rd, x0, imm
                    self.set_reg(cli.crs1rd(), cli.cimmi() as u32);
                }
                CAddi16sp(caddi16sp) => {
                    // addi x2, x2, nzimm[9:4]
                    let x2 = self.get_reg(2);
                    let imm = caddi16sp.cimm16sp();
                    self.set_reg(2, x2.wrapping_add_signed(imm));
                }
                CLui(clui) => {
                    // lui rd, imm
                    self.set_reg(clui.crd(), clui.cimmui() as u32);
                }
                CAndi(candi) => {
                    // andi rd, rd, imm
                    let rd = candi.crs1rdq();
                    let imm = candi.cnzimmi() as u32;
                    self.set_reg(rd, self.get_reg(rd) & imm);
                }
                CSub(csub) => {
                    // sub rd, rd, rs2
                    let rs2 = self.get_reg(csub.crs2q());
                    let rd = csub.crs1rdq();
                    self.set_reg(rd, self.get_reg(rd).wrapping_sub(rs2));
                }
                CXor(cxor) => {
                    // xor rd, rd, rs2
                    let rs2 = self.get_reg(cxor.crs2q());
                    let rd = cxor.crs1rdq();
                    self.set_reg(rd, self.get_reg(rd) ^ rs2);
                }
                COr(cor) => {
                    // or rd, rd, rs2
                    let rs2 = self.get_reg(cor.crs2q());
                    let rd = cor.crs1rdq();
                    self.set_reg(rd, self.get_reg(rd) | rs2);
                }
                CAnd(cand) => {
                    // and rd, rd, rs2
                    let rs2 = self.get_reg(cand.crs2q());
                    let rd = cand.crs1rdq();
                    self.set_reg(rd, self.get_reg(rd) & rs2);
                }
                CSubw(csubw) => todo!(),
                CAddw(caddw) => todo!(),
                CJ(cj) => {
                    // jal x0, offset
                    let offset = cj.cimmj();
                    next_pc = self.pc.wrapping_add_signed(offset);
                }
                CBeqz(cbeqz) => {
                    // beq rs1′, x0, offset
                    let rs1 = self.get_reg(cbeqz.crs1q());
                    let offset = cbeqz.cimmb();
                    if rs1 == 0 {
                        next_pc = self.pc.wrapping_add_signed(offset);
                    }
                }
                CBnez(cbnez) => {
                    // bne rs1′, x0, offset
                    let rs1 = self.get_reg(cbnez.crs1q());
                    let offset = cbnez.cimmb();
                    if rs1 != 0 {
                        next_pc = self.pc.wrapping_add_signed(offset);
                    }
                }
                CFldsp(cfldsp) => todo!(),
                CLwsp(clwsp) => {
                    // lw rd, imm(x2)
                    let rd = clwsp.crd();
                    let x2 = self.get_reg(2);
                    let addr = x2.wrapping_add(clwsp.cimmlwsp());
                    self.set_reg(rd, self.mem.load::<u32>(addr));
                }
                CFlwsp(cflwsp) => todo!(),
                CJr(cjr) => {
                    // jalr x0, 0(rs1)
                    let rs1 = self.get_reg(cjr.crs1());
                    next_pc = rs1;
                }
                CMv(cmv) => {
                    // add rd, x0, rs2
                    let rs2 = self.get_reg(cmv.crs2());
                    self.set_reg(cmv.crd(), rs2);
                }
                CEbreak(cebreak) => {
                    // ebreak
                    self.running = false;
                }
                CJalr(cjalr) => {
                    // jalr x1, 0(rs1)
                    let rs1 = self.get_reg(cjalr.crs1());
                    self.set_reg(1, next_pc);
                    next_pc = rs1;
                }
                CAdd(cadd) => {
                    // add rd, rd, rs2
                    let rs2 = self.get_reg(cadd.crs2());
                    let rd = cadd.crs1rd();
                    self.set_reg(rd, self.get_reg(rd).wrapping_add(rs2));
                }
                CFsdsp(cfsdsp) => todo!(),
                CSwsp(cswsp) => {
                    // sw rs2, offset(x2)
                    let rs2 = self.get_reg(cswsp.crs2());
                    let x2 = self.get_reg(2);
                    let addr = x2.wrapping_add(cswsp.cimmswsp());
                    self.mem.store::<u32>(addr, rs2);
                }
                CFswsp(cfswsp) => todo!(),
                CSrli(csrli) => {
                    // srli rd′, rd′, shamt
                    let rd = csrli.crs1rdq();
                    let shamt = csrli.cimmsh6();
                    self.set_reg(rd, self.get_reg(rd) >> shamt);
                }
                CSrai(csrai) => {
                    // srai rd′, rd′, shamt
                    let rd = csrai.crs1rdq();
                    let shamt = csrai.cimmsh6();
                    self.set_reg(rd, ((self.get_reg(rd) as i32) >> shamt) as u32);
                }
                CSlli(cslli) => {
                    // slli rd, rd, shamt[5:0]
                    let rd = cslli.crs1rdq();
                    let shamt = cslli.cimmsh6();
                    self.set_reg(rd, self.get_reg(rd) << shamt);
                }
                CLd(cld) => todo!(),
                CSd(csd) => todo!(),
                CAddiw(caddiw) => todo!(),
                CLdsp(cldsp) => panic!("RV64 only {cldsp}"),
                CSdsp(csdsp) => todo!(),
                CLq(clq) => todo!(),
                CSq(csq) => todo!(),
                CLqsp(clqsp) => todo!(),
                CSqsp(csqsp) => todo!(),
            }

            self.pc = next_pc;
        }
    }

    pub fn syscall(&mut self, _rs1: u32, imm: i32) {
        if imm == 0 {
            // ecall
            let call = self.get_reg(17);
            println!("syscall {call}");

            macro_rules! syscall {
                ($syscall:ident(
                    $($ty:ident $arg:ident),*
                )) => {{
                    syscall!(
                        @fetch_regs,
                        10,
                        $($ty, $arg),*
                    );
                    let res = crate::syscall::$syscall(self, $($arg as _),*);
                    self.set_reg(10, res as u32);
                }};
                (@fetch_regs, $counter: expr, $ty: ident, $arg:ident, $($rest_ty:ident, $rest:ident),+) => {
                    let $arg = self.get_reg($counter);
                    syscall!(@arg, $ty, $arg);
                    syscall!(@fetch_regs, $counter + 1, $($rest_ty, $rest),*);
                };
                (@fetch_regs, $counter: expr, $ty: ident, $arg:ident) => {
                    let $arg = self.get_reg($counter);
                    syscall!(@arg, $ty, $arg);
                };
                (@fetch_regs, $counter: expr,) => {};
                (@arg, ptr, $arg:ident) => {
                    let $arg = self.mem.pointer($arg);
                };
                (@arg, val, $arg:ident) => {};
            }

            match call {
                // write(int fd, const void* buf, size_t count)
                64 => syscall!(write(val fd, ptr buf, val count)),
                // writev(int fd, const struct iovec* iov, int iovcnt)
                66 => syscall!(writev(val fd, ptr iov, val iovcnt)),
                // readlinkat(int dirfd, const char* pathname, char* buf, size_t bufsiz)
                78 => syscall!(readlinkat(val dirfd, ptr pathname, ptr buf, val bufsiz)),
                // exit_group(int status)
                94 => syscall!(exit_group(val status)),
                // set_tid_address(int* tidptr)
                96 => syscall!(set_tid_address(ptr tidptr)),
                // set_robust_list(struct robust_list_head* head, size_t len)
                99 => syscall!(set_robust_list(ptr head, val len)),
                // tgkill(int tgid, int tid, int sig)
                131 => syscall!(tgkill(val tgid, val tid, val sig)),
                // rt_sigprocmask(int how, const sigset_t* set, sigset_t* oldset, size_t sigsetsize)
                135 => syscall!(rt_sigprocmask(val how, ptr set, ptr oldset, val sigsetsize)),
                // getpid()
                172 => syscall!(getpid()),
                // gettid()
                178 => syscall!(gettid()),
                // brk(void* addr)
                214 => syscall!(brk(ptr addr)),
                // mmap(void* addr, size_t length, int prot, int flags, int fd, off_t offset)
                222 => {
                    syscall!(mmap(val addr, val length, val prot, val flags, val fd, val offset))
                }
                // mprotect(void* addr, size_t len, int prot)
                226 => syscall!(mprotect(ptr addr, val len, val prot)),
                // riscv_hwprobe(struct riscv_hwprobe* pairs, size_t pair_count,
                //               size_t cpusetsize, cpu_set_t *cpus,
                //               unsigned int flags)
                258 => {
                    syscall!(riscv_hwprobe(ptr pairs, val pair_count, val cpusetsize, ptr cpus, val flags))
                }
                // getrlimit(int resource, struct rlimit* rlim)
                261 => syscall!(getrlimit(val resource, ptr rlim)),
                // getrandom(void* buf, size_t buflen, uint flags)
                278 => syscall!(getrandom(ptr buf, val buflen, val flags)),
                // statx
                291 => {
                    syscall!(statx(val dirfd, ptr pathname, val flags, val mask, ptr statxbuf))
                }
                // futex(int* uaddr, int futex_op, int val, size_t timeout, int* uaddr2, int val3)
                422 => {
                    syscall!(futex(ptr uaddr, val futex_op, val val, val timeout, ptr uaddr2, val val3))
                }

                _ => {
                    println!("Unknown syscall: {call}");
                    println!("Register states:");
                    for i in 10..=16 {
                        println!("x{}: {}", i, self.get_reg(i));
                    }
                    self.running = false;
                }
            }
            return;

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
