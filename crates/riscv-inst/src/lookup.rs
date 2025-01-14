use riscv_inst_macros::{bits, instructions};

#[instructions]
pub enum Opcode {
    // Load instructions
    #[fields(rd, rs1, oimm12 as imm)]
    #[isa(base = "RV32I")]
    Lb,
    #[fields(rd, rs1, oimm12 as imm)]
    #[isa(base = "RV32I")]
    Lh,
    #[fields(rd, rs1, oimm12 as imm)]
    #[isa(base = "RV32I")]
    Lw,
    #[fields(rd, rs1, oimm12 as imm)]
    #[isa(base = "RV64I")]
    Ld,
    #[fields(rd, rs1, oimm12 as imm)]
    #[isa(base = "RV128I")]
    Lq,
    #[fields(rd, rs1, oimm12 as imm)]
    #[isa(base = "RV32I")]
    Lbu,
    #[fields(rd, rs1, oimm12 as imm)]
    #[isa(base = "RV32I")]
    Lhu,
    #[fields(rd, rs1, oimm12 as imm)]
    #[isa(base = "RV64I")]
    Lwu,
    #[fields(rd, rs1, oimm12 as imm)]
    #[isa(base = "RV128I")]
    Ldu,

    // Floating point loads
    #[fields(frd, rs1, oimm12 as imm)]
    #[isa(base = "RV32I", ext = "F")]
    Flw,
    #[fields(frd, rs1, oimm12 as imm)]
    #[isa(base = "RV32I", ext = "D")]
    Fld,
    #[fields(frd, rs1, oimm12 as imm)]
    #[isa(base = "RV32I", ext = "Q")]
    Flq,

    // Fence instructions
    #[fields(pred, succ)]
    #[isa(base = "RV32I")]
    Fence,
    #[fields()]
    #[isa(base = "RV32I")]
    FenceI,

    // Immediate arithmetic
    #[fields(rd, rs1, imm12 as imm)]
    #[isa(base = "RV32I")]
    Addi,
    #[fields(rd, rs1, imm12 as imm)]
    #[isa(base = "RV32I")]
    Slti,
    #[fields(rd, rs1, imm12 as imm)]
    #[isa(base = "RV32I")]
    Sltiu,
    #[fields(rd, rs1, imm12 as imm)]
    #[isa(base = "RV32I")]
    Xori,
    #[fields(rd, rs1, imm12 as imm)]
    #[isa(base = "RV32I")]
    Ori,
    #[fields(rd, rs1, imm12 as imm)]
    #[isa(base = "RV32I")]
    Andi,
    #[fields(rd, rs1, shamt5 as shamt)]
    #[isa(base = "RV32I")]
    Slli,
    #[fields(rd, rs1, shamt5 as shamt)]
    #[isa(base = "RV32I")]
    Srli,
    #[fields(rd, rs1, shamt5 as shamt)]
    #[isa(base = "RV32I")]
    Srai,

    #[fields(rd, oimm20 as imm)]
    #[isa(base = "RV32I")]
    Auipc,

    // RV64I immediate arithmetic
    #[fields(rd, rs1, imm12 as imm)]
    #[isa(base = "RV64I")]
    Addiw,
    #[fields(rd, rs1, shamt5 as shamt)]
    #[isa(base = "RV64I")]
    Slliw,
    #[fields(rd, rs1, shamt5 as shamt)]
    #[isa(base = "RV64I")]
    Srliw,
    #[fields(rd, rs1, shamt5 as shamt)]
    #[isa(base = "RV64I")]
    Sraiw,

    // Store instructions
    #[fields(rs1, rs2, simm12 as imm)]
    #[isa(base = "RV32I")]
    Sb,
    #[fields(rs1, rs2, simm12 as imm)]
    #[isa(base = "RV32I")]
    Sh,
    #[fields(rs1, rs2, simm12 as imm)]
    #[isa(base = "RV32I")]
    Sw,
    #[fields(rs1, rs2, simm12 as imm)]
    #[isa(base = "RV64I")]
    Sd,
    #[fields(rs1, rs2, simm12 as imm)]
    #[isa(base = "RV128I")]
    Sq,

    // Floating point stores
    #[fields(rs1, frs2, simm12 as imm)]
    #[isa(base = "RV32I", ext = "F")]
    Fsw,
    #[fields(rs1, frs2, simm12 as imm)]
    #[isa(base = "RV32I", ext = "D")]
    Fsd,
    #[fields(rs1, frs2, simm12 as imm)]
    #[isa(base = "RV32I", ext = "Q")]
    Fsq,

    // RV32A atomic instructions
    #[fields(rd, rs1, aq, rl)]
    #[isa(base = "RV32I", ext = "A")]
    LrW,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV32I", ext = "A")]
    ScW,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV32I", ext = "A")]
    AmoswapW,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV32I", ext = "A")]
    AmoaddW,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV32I", ext = "A")]
    AmoxorW,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV32I", ext = "A")]
    AmoandW,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV32I", ext = "A")]
    AmoorW,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV32I", ext = "A")]
    AmominW,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV32I", ext = "A")]
    AmomaxW,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV32I", ext = "A")]
    AmominuW,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV32I", ext = "A")]
    AmomaxuW,

    // RV64A atomic instructions
    #[fields(rd, rs1, aq, rl)]
    #[isa(base = "RV64I", ext = "A")]
    LrD,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV64I", ext = "A")]
    ScD,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV64I", ext = "A")]
    AmoswapD,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV64I", ext = "A")]
    AmoaddD,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV64I", ext = "A")]
    AmoxorD,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV64I", ext = "A")]
    AmoandD,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV64I", ext = "A")]
    AmoorD,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV64I", ext = "A")]
    AmominD,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV64I", ext = "A")]
    AmomaxD,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV64I", ext = "A")]
    AmominuD,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV64I", ext = "A")]
    AmomaxuD,

    // RV128A atomic instructions
    #[fields(rd, rs1, aq, rl)]
    #[isa(base = "RV128I", ext = "A")]
    LrQ,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV128I", ext = "A")]
    ScQ,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV128I", ext = "A")]
    AmoswapQ,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV128I", ext = "A")]
    AmoaddQ,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV128I", ext = "A")]
    AmoxorQ,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV128I", ext = "A")]
    AmoandQ,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV128I", ext = "A")]
    AmoorQ,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV128I", ext = "A")]
    AmominQ,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV128I", ext = "A")]
    AmomaxQ,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV128I", ext = "A")]
    AmominuQ,
    #[fields(rd, rs1, rs2, aq, rl)]
    #[isa(base = "RV128I", ext = "A")]
    AmomaxuQ,

    // Register arithmetic
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I")]
    Add,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I")]
    Sub,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I")]
    Sll,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I")]
    Slt,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I")]
    Sltu,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I")]
    Xor,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I")]
    Srl,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I")]
    Sra,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I")]
    Or,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I")]
    And,

    // RV32M multiply/divide
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I", ext = "M")]
    Mul,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I", ext = "M")]
    Mulh,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I", ext = "M")]
    Mulhsu,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I", ext = "M")]
    Mulhu,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I", ext = "M")]
    Div,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I", ext = "M")]
    Divu,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I", ext = "M")]
    Rem,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV32I", ext = "M")]
    Remu,

    #[fields(rd, imm20 as imm)]
    #[isa(base = "RV32I")]
    Lui,

    // RV64I register arithmetic
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV64I")]
    Addw,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV64I")]
    Subw,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV64I")]
    Sllw,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV64I")]
    Srlw,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV64I")]
    Sraw,

    // RV64M multiply/divide
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV64I", ext = "M")]
    Mulw,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV64I", ext = "M")]
    Divw,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV64I", ext = "M")]
    Divuw,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV64I", ext = "M")]
    Remw,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV64I", ext = "M")]
    Remuw,

    // RV32F floating point
    #[fields(frd, frs1, frs2, frs3, rm)]
    #[isa(base = "RV32I", ext = "F")]
    FmaddS,
    #[fields(frd, frs1, frs2, frs3, rm)]
    #[isa(base = "RV32I", ext = "F")]
    FmsubS,
    #[fields(frd, frs1, frs2, frs3, rm)]
    #[isa(base = "RV32I", ext = "F")]
    FnmsubS,
    #[fields(frd, frs1, frs2, frs3, rm)]
    #[isa(base = "RV32I", ext = "F")]
    FnmaddS,
    #[fields(frd, frs1, frs2, rm)]
    #[isa(base = "RV32I", ext = "F")]
    FaddS,
    #[fields(frd, frs1, frs2, rm)]
    #[isa(base = "RV32I", ext = "F")]
    FsubS,
    #[fields(frd, frs1, frs2, rm)]
    #[isa(base = "RV32I", ext = "F")]
    FmulS,
    #[fields(frd, frs1, frs2, rm)]
    #[isa(base = "RV32I", ext = "F")]
    FdivS,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "F")]
    FsgnjS,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "F")]
    FsgnjnS,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "F")]
    FsgnjxS,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "F")]
    FminS,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "F")]
    FmaxS,
    #[fields(frd, frs1, rm)]
    #[isa(base = "RV32I", ext = "F")]
    FsqrtS,
    #[fields(rd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "F")]
    FleS,
    #[fields(rd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "F")]
    FltS,
    #[fields(rd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "F")]
    FeqS,
    #[fields(rd, frs1, rm)]
    #[isa(base = "RV32I", ext = "F")]
    FcvtWS,
    #[fields(rd, frs1, rm)]
    #[isa(base = "RV32I", ext = "F")]
    FcvtWuS,
    #[fields(frd, rs1, rm)]
    #[isa(base = "RV32I", ext = "F")]
    FcvtSW,
    #[fields(frd, rs1, rm)]
    #[isa(base = "RV32I", ext = "F")]
    FcvtSWu,
    #[fields(rd, frs1)]
    #[isa(base = "RV32I", ext = "F")]
    FmvXS,
    #[fields(rd, frs1)]
    #[isa(base = "RV32I", ext = "F")]
    FclassS,
    #[fields(frd, rs1)]
    #[isa(base = "RV32I", ext = "F")]
    FmvSX,

    // RV64F floating point
    #[fields(rd, frs1, rm)]
    #[isa(base = "RV64I", ext = "F")]
    FcvtLS,
    #[fields(rd, frs1, rm)]
    #[isa(base = "RV64I", ext = "F")]
    FcvtLuS,
    #[fields(frd, rs1, rm)]
    #[isa(base = "RV64I", ext = "F")]
    FcvtSL,
    #[fields(frd, rs1, rm)]
    #[isa(base = "RV64I", ext = "F")]
    FcvtSLu,

    // RV32D floating point
    #[fields(frd, frs1, frs2, frs3, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FmaddD,
    #[fields(frd, frs1, frs2, frs3, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FmsubD,
    #[fields(frd, frs1, frs2, frs3, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FnmsubD,
    #[fields(frd, frs1, frs2, frs3, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FnmaddD,
    #[fields(frd, frs1, frs2, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FaddD,
    #[fields(frd, frs1, frs2, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FsubD,
    #[fields(frd, frs1, frs2, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FmulD,
    #[fields(frd, frs1, frs2, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FdivD,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "D")]
    FsgnjD,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "D")]
    FsgnjnD,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "D")]
    FsgnjxD,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "D")]
    FminD,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "D")]
    FmaxD,
    #[fields(frd, frs1, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FsqrtD,
    #[fields(rd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "D")]
    FleD,
    #[fields(rd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "D")]
    FltD,
    #[fields(rd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "D")]
    FeqD,
    #[fields(rd, frs1, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FcvtWD,
    #[fields(rd, frs1, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FcvtWuD,
    #[fields(frd, rs1, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FcvtDW,
    #[fields(frd, rs1, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FcvtDWu,
    #[fields(frd, frs1, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FcvtSD,
    #[fields(frd, frs1, rm)]
    #[isa(base = "RV32I", ext = "D")]
    FcvtDS,
    #[fields(rd, frs1)]
    #[isa(base = "RV32I", ext = "D")]
    FclassD,

    // RV64D floating point
    #[fields(rd, frs1, rm)]
    #[isa(base = "RV64I", ext = "D")]
    FcvtLD,
    #[fields(rd, frs1, rm)]
    #[isa(base = "RV64I", ext = "D")]
    FcvtLuD,
    #[fields(rd, frs1)]
    #[isa(base = "RV64I", ext = "D")]
    FmvXD,
    #[fields(frd, rs1)]
    #[isa(base = "RV64I", ext = "D")]
    FmvDX,
    #[fields(frd, rs1, rm)]
    #[isa(base = "RV64I", ext = "D")]
    FcvtDL,
    #[fields(frd, rs1, rm)]
    #[isa(base = "RV64I", ext = "D")]
    FcvtDLu,

    // RV32Q floating point
    #[fields(frd, frs1, frs2, frs3, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FmaddQ,
    #[fields(frd, frs1, frs2, frs3, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FmsubQ,
    #[fields(frd, frs1, frs2, frs3, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FnmsubQ,
    #[fields(frd, frs1, frs2, frs3, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FnmaddQ,
    #[fields(frd, frs1, frs2, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FaddQ,
    #[fields(frd, frs1, frs2, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FsubQ,
    #[fields(frd, frs1, frs2, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FmulQ,
    #[fields(frd, frs1, frs2, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FdivQ,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "Q")]
    FsgnjQ,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "Q")]
    FsgnjnQ,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "Q")]
    FsgnjxQ,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "Q")]
    FminQ,
    #[fields(frd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "Q")]
    FmaxQ,
    #[fields(frd, frs1, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FsqrtQ,
    #[fields(rd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "Q")]
    FleQ,
    #[fields(rd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "Q")]
    FltQ,
    #[fields(rd, frs1, frs2)]
    #[isa(base = "RV32I", ext = "Q")]
    FeqQ,
    #[fields(rd, frs1, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FcvtWQ,
    #[fields(rd, frs1, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FcvtWuQ,
    #[fields(frd, rs1, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FcvtQW,
    #[fields(frd, rs1, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FcvtQWu,
    #[fields(frd, frs1, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FcvtSQ,
    #[fields(frd, frs1, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FcvtQS,
    #[fields(frd, frs1, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FcvtDQ,
    #[fields(frd, frs1, rm)]
    #[isa(base = "RV32I", ext = "Q")]
    FcvtQD,
    #[fields(rd, frs1)]
    #[isa(base = "RV32I", ext = "Q")]
    FclassQ,

    // RV64Q floating point
    #[fields(rd, frs1, rm)]
    #[isa(base = "RV64I", ext = "Q")]
    FcvtLQ,
    #[fields(rd, frs1, rm)]
    #[isa(base = "RV64I", ext = "Q")]
    FcvtLuQ,
    #[fields(frd, rs1, rm)]
    #[isa(base = "RV64I", ext = "Q")]
    FcvtQL,
    #[fields(frd, rs1, rm)]
    #[isa(base = "RV64I", ext = "Q")]
    FcvtQLu,

    // RV128Q floating point
    #[fields(rd, frs1)]
    #[isa(base = "RV128I", ext = "Q")]
    FmvXQ,
    #[fields(frd, rs1)]
    #[isa(base = "RV128I", ext = "Q")]
    FmvQX,

    // RV128I immediate arithmetic
    #[fields(rd, rs1, imm12 as imm)]
    #[isa(base = "RV128I")]
    Addid,
    #[fields(rd, rs1, shamt6 as shamt)]
    #[isa(base = "RV128I")]
    Sllid,
    #[fields(rd, rs1, shamt6 as shamt)]
    #[isa(base = "RV128I")]
    Srlid,
    #[fields(rd, rs1, shamt6 as shamt)]
    #[isa(base = "RV128I")]
    Sraid,

    // Branch instructions
    #[fields(rs1, rs2, sbimm12 as imm)]
    #[isa(base = "RV32I")]
    Beq,
    #[fields(rs1, rs2, sbimm12 as imm)]
    #[isa(base = "RV32I")]
    Bne,
    #[fields(rs1, rs2, sbimm12 as imm)]
    #[isa(base = "RV32I")]
    Blt,
    #[fields(rs1, rs2, sbimm12 as imm)]
    #[isa(base = "RV32I")]
    Bge,
    #[fields(rs1, rs2, sbimm12 as imm)]
    #[isa(base = "RV32I")]
    Bltu,
    #[fields(rs1, rs2, sbimm12 as imm)]
    #[isa(base = "RV32I")]
    Bgeu,

    // Jump instructions
    #[fields(rd, rs1, oimm12 as imm)]
    #[isa(base = "RV32I")]
    Jalr,
    #[fields(rd, jimm20 as imm)]
    #[isa(base = "RV32I")]
    Jal,

    // System instructions
    #[fields(rs1, imm12 as imm)]
    #[isa(base = "RV32I")]
    Ecall,
    #[fields(rs1, imm12 as imm)]
    #[isa(base = "RV32I")]
    Ebreak,

    // RV128I register arithmetic
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV128I")]
    Addd,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV128I")]
    Subd,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV128I")]
    Slld,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV128I")]
    Srld,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV128I")]
    Srad,

    // RV128M multiply/divide
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV128I", ext = "M")]
    Muld,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV128I", ext = "M")]
    Divd,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV128I", ext = "M")]
    Divud,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV128I", ext = "M")]
    Remd,
    #[fields(rd, rs1, rs2)]
    #[isa(base = "RV128I", ext = "M")]
    Remud,

    // RV Compressed instructions
    #[fields(crdq, cimm4spn)]
    #[isa(base = "RV32I", ext = "C")]
    CAddi4spn,
    #[fields(cfrdq, crs1q, cimmd)]
    #[isa(base = "RV32I", ext = "C")]
    CFld,
    #[fields(crdq, crs1q, cimmw)]
    #[isa(base = "RV32I", ext = "C")]
    CLw,
    #[fields(cfrdq, crs1q, cimmw)]
    #[isa(base = "RV32I", ext = "C")]
    CFlw,
    #[fields(crs1q, cfrs2q, cimmd)]
    #[isa(base = "RV32I", ext = "C")]
    CFsd,
    #[fields(crs1q, crs2q, cimmw)]
    #[isa(base = "RV32I", ext = "C")]
    CSw,
    #[fields(crs1q, cfrs2q, cimmw)]
    #[isa(base = "RV32I", ext = "C")]
    CFsw,
    #[fields()]
    #[isa(base = "RV32I", ext = "C")]
    CNop,
    #[fields(crs1rd, cnzimmi)]
    #[isa(base = "RV32I", ext = "C")]
    CAddi,
    #[fields(cimmj)]
    #[isa(base = "RV32I", ext = "C")]
    CJal,
    #[fields(crs1rd, cimmi)]
    #[isa(base = "RV32I", ext = "C")]
    CLi,
    #[fields(crs1rd, cimm16sp)]
    #[isa(base = "RV32I", ext = "C")]
    CAddi16sp,
    #[fields(crd, cimmui)]
    #[isa(base = "RV32I", ext = "C")]
    CLui,
    #[fields(crs1rdq, cnzimmi)]
    #[isa(base = "RV32I", ext = "C")]
    CAndi,
    #[fields(crs1rdq, crs2q)]
    #[isa(base = "RV32I", ext = "C")]
    CSub,
    #[fields(crs1rdq, crs2q)]
    #[isa(base = "RV32I", ext = "C")]
    CXor,
    #[fields(crs1rdq, crs2q)]
    #[isa(base = "RV32I", ext = "C")]
    COr,
    #[fields(crs1rdq, crs2q)]
    #[isa(base = "RV32I", ext = "C")]
    CAnd,
    #[fields(crs1rdq, crs2q)]
    #[isa(base = "RV32I", ext = "C")]
    CSubw,
    #[fields(crs1rdq, crs2q)]
    #[isa(base = "RV32I", ext = "C")]
    CAddw,
    #[fields(cimmj)]
    #[isa(base = "RV32I", ext = "C")]
    CJ,
    #[fields(crs1q, cimmb)]
    #[isa(base = "RV32I", ext = "C")]
    CBeqz,
    #[fields(crs1q, cimmb)]
    #[isa(base = "RV32I", ext = "C")]
    CBnez,
    #[fields(cfrd, cimmldsp)]
    #[isa(base = "RV32I", ext = "C")]
    CFldsp,
    #[fields(crd, cimmlwsp)]
    #[isa(base = "RV32I", ext = "C")]
    CLwsp,
    #[fields(cfrd, cimmlwsp)]
    #[isa(base = "RV32I", ext = "C")]
    CFlwsp,
    #[fields(crd0, crs1)]
    #[isa(base = "RV32I", ext = "C")]
    CJr,
    #[fields(crd, crs2)]
    #[isa(base = "RV32I", ext = "C")]
    CMv,
    #[fields()]
    #[isa(base = "RV32I", ext = "C")]
    CEbreak,
    #[fields(crd0, crs1)]
    #[isa(base = "RV32I", ext = "C")]
    CJalr,
    #[fields(crs1rd, crs2)]
    #[isa(base = "RV32I", ext = "C")]
    CAdd,
    #[fields(cfrs2, cimmsdsp)]
    #[isa(base = "RV32I", ext = "C")]
    CFsdsp,
    #[fields(crs2, cimmswsp)]
    #[isa(base = "RV32I", ext = "C")]
    CSwsp,
    #[fields(cfrs2, cimmswsp)]
    #[isa(base = "RV32I", ext = "C")]
    CFswsp,

    // Shared RV32C and RV64C compressed instructions (choose cimmsh5, cimmsh6)
    #[fields(crs1rdq, cimmsh5, cimmsh6)]
    #[isa(base = "RV32I", ext = "C")]
    CSrli,
    #[fields(crs1rdq, cimmsh5, cimmsh6)]
    #[isa(base = "RV32I", ext = "C")]
    CSrai,
    #[fields(crs1rdq, cimmsh5, cimmsh6)]
    #[isa(base = "RV32I", ext = "C")]
    CSlli,

    // RV64C compressed instructions
    #[fields(crdq, crs1q, cimmd)]
    #[isa(base = "RV64I", ext = "C")]
    CLd,
    #[fields(crs1q, crs2q, cimmd)]
    #[isa(base = "RV64I", ext = "C")]
    CSd,
    #[fields(crs1rd, cimmi)]
    #[isa(base = "RV64I", ext = "C")]
    CAddiw,
    #[fields(crd, cimmldsp)]
    #[isa(base = "RV64I", ext = "C")]
    CLdsp,
    #[fields(crs2, cimmsdsp)]
    #[isa(base = "RV64I", ext = "C")]
    CSdsp,

    // RV128C compressed instructions
    #[fields(crdq, crs1q, cimmq)]
    #[isa(base = "RV128I", ext = "C")]
    CLq,
    #[fields(crs1q, crs2q, cimmq)]
    #[isa(base = "RV128I", ext = "C")]
    CSq,
    #[fields(crd, cimmlqsp)]
    #[isa(base = "RV128I", ext = "C")]
    CLqsp,
    #[fields(crs2, cimmsqsp)]
    #[isa(base = "RV128I", ext = "C")]
    CSqsp,
}

// Decode instructions in the fourth quadrant, e.g. ending in 0b11:
// These are the full 32-bit instructions.
pub fn decode_q4(inst: u32) -> Option<Opcode> {
    match bits!(inst[6:2]) {
        0x0 => match bits!(inst[14:12]) {
            0x0 => Some(Lb(inst).into()),
            0x1 => Some(Lh(inst).into()),
            0x2 => Some(Lw(inst).into()),
            0x3 => Some(Ld(inst).into()),
            0x4 => Some(Lbu(inst).into()),
            0x5 => Some(Lhu(inst).into()),
            0x6 => Some(Lwu(inst).into()),
            0x7 => Some(Ldu(inst).into()),
            _ => unreachable!(),
        },
        0x1 => match bits!(inst[14:12]) {
            0x2 => Some(Flw(inst).into()),
            0x3 => Some(Fld(inst).into()),
            0x4 => Some(Flq(inst).into()),
            0x0..=0x1 | 0x5..=0x7 => None,
            _ => unreachable!(),
        },
        0x2 => None,
        0x3 => match bits!(inst[14:12]) {
            0x0 => Some(Fence(inst).into()),
            0x1 => Some(FenceI(inst).into()),
            0x2 => Some(Lq(inst).into()),
            0x3..=0x7 => None,
            _ => unreachable!(),
        },
        0x4 => match bits!(inst[14:12]) {
            0x0 => Some(Addi(inst).into()),
            0x1 if bits!(inst[31:27]) == 0 => Some(Slli(inst).into()),
            0x2 => Some(Slti(inst).into()),
            0x3 => Some(Sltiu(inst).into()),
            0x4 => Some(Xori(inst).into()),
            0x5 if bits!(inst[31:27]) == 0 => Some(Srli(inst).into()),
            0x5 if bits!(inst[31:27]) == 8 => Some(Srai(inst).into()),
            0x6 => Some(Ori(inst).into()),
            0x7 => Some(Andi(inst).into()),
            _ => unreachable!(),
        },
        0x5 => Some(Auipc(inst).into()),
        0x6 => match bits!(inst[14:12]) {
            0x0 => Some(Addiw(inst).into()),
            0x1 if (inst >> 25) == 0 => Some(Slliw(inst).into()),
            0x5 if (inst >> 25) == 0 => Some(Srliw(inst).into()),
            0x5 if (inst >> 25) == 8 => Some(Sraiw(inst).into()),
            0x2..=0x7 => None,
            _ => unreachable!(),
        },
        0x7 => None,
        0x8 => match bits!(inst[14:12]) {
            0x0 => Some(Sb(inst).into()),
            0x1 => Some(Sh(inst).into()),
            0x2 => Some(Sw(inst).into()),
            0x3 => Some(Sd(inst).into()),
            0x4 => Some(Sq(inst).into()),
            0x5..=0x7 => None,
            _ => unreachable!(),
        },
        0x9 => match bits!(inst[14:12]) {
            0x2 => Some(Fsw(inst).into()),
            0x3 => Some(Fsd(inst).into()),
            0x4 => Some(Fsq(inst).into()),
            0x0..=0x1 | 0x5..=0x7 => None,
            _ => unreachable!(),
        },
        0xa => None,
        0xb => {
            match bits!(inst[31:27|14:12]) {
                2 => Some(AmoaddW(inst).into()),
                3 => Some(AmoaddD(inst).into()),
                4 => Some(AmoaddQ(inst).into()),
                10 => Some(AmoswapW(inst).into()),
                11 => Some(AmoswapD(inst).into()),
                12 => Some(AmoswapQ(inst).into()),

                // LR instructions need to check rs2 field (inst[24:20]) is zero
                18 if bits!(inst[24:20]) == 0 => Some(LrW(inst).into()),
                19 if bits!(inst[24:20]) == 0 => Some(LrD(inst).into()),
                20 if bits!(inst[24:20]) == 0 => Some(LrQ(inst).into()),

                26 => Some(ScW(inst).into()),
                27 => Some(ScD(inst).into()),
                28 => Some(ScQ(inst).into()),

                34 => Some(AmoxorW(inst).into()),
                35 => Some(AmoxorD(inst).into()),
                36 => Some(AmoxorQ(inst).into()),

                66 => Some(AmoorW(inst).into()),
                67 => Some(AmoorD(inst).into()),
                68 => Some(AmoorQ(inst).into()),

                98 => Some(AmoandW(inst).into()),
                99 => Some(AmoandD(inst).into()),
                100 => Some(AmoandQ(inst).into()),

                130 => Some(AmominW(inst).into()),
                131 => Some(AmominD(inst).into()),
                132 => Some(AmominQ(inst).into()),

                162 => Some(AmomaxW(inst).into()),
                163 => Some(AmomaxD(inst).into()),
                164 => Some(AmomaxQ(inst).into()),

                194 => Some(AmominuW(inst).into()),
                195 => Some(AmominuD(inst).into()),
                196 => Some(AmominuQ(inst).into()),

                226 => Some(AmomaxuW(inst).into()),
                227 => Some(AmomaxuD(inst).into()),
                228 => Some(AmomaxuQ(inst).into()),

                _ => None,
            }
        }
        0xc => match bits!(inst[31:25|14:12]) {
            0 => Some(Add(inst).into()),
            1 => Some(Sll(inst).into()),
            2 => Some(Slt(inst).into()),
            3 => Some(Sltu(inst).into()),
            4 => Some(Xor(inst).into()),
            5 => Some(Srl(inst).into()),
            6 => Some(Or(inst).into()),
            7 => Some(And(inst).into()),
            8 => Some(Mul(inst).into()),
            9 => Some(Mulh(inst).into()),
            10 => Some(Mulhsu(inst).into()),
            11 => Some(Mulhu(inst).into()),
            12 => Some(Div(inst).into()),
            13 => Some(Divu(inst).into()),
            14 => Some(Rem(inst).into()),
            15 => Some(Remu(inst).into()),
            256 => Some(Sub(inst).into()),
            261 => Some(Sra(inst).into()),
            _ => None,
        },
        0xd => Some(Lui(inst).into()),
        0xe => match bits!(inst[31:25|14:12]) {
            0 => Some(Addw(inst).into()),
            1 => Some(Sllw(inst).into()),
            5 => Some(Srlw(inst).into()),
            8 => Some(Mulw(inst).into()),
            12 => Some(Divw(inst).into()),
            13 => Some(Divuw(inst).into()),
            14 => Some(Remw(inst).into()),
            15 => Some(Remuw(inst).into()),
            256 => Some(Subw(inst).into()),
            261 => Some(Sraw(inst).into()),
            _ => None,
        },
        0xf => None,
        ty @ 0x10..=0x13 => match (ty, bits!(inst[26:5])) {
            (0x10, 0x0) => Some(FmaddS(inst).into()),
            (0x10, 0x1) => Some(FmaddD(inst).into()),
            (0x10, 0x3) => Some(FmaddQ(inst).into()),

            (0x11, 0x0) => Some(FmsubS(inst).into()),
            (0x11, 0x1) => Some(FmsubD(inst).into()),
            (0x11, 0x3) => Some(FmsubQ(inst).into()),

            (0x12, 0x0) => Some(FnmsubS(inst).into()),
            (0x12, 0x1) => Some(FnmsubD(inst).into()),
            (0x12, 0x3) => Some(FnmsubQ(inst).into()),

            (0x13, 0x0) => Some(FnmaddS(inst).into()),
            (0x13, 0x1) => Some(FnmaddD(inst).into()),
            (0x13, 0x3) => Some(FnmaddQ(inst).into()),

            _ => None,
        },
        0x14 => match bits!(inst[31:25]) {
            0 => Some(FaddS(inst).into()),
            1 => Some(FaddD(inst).into()),
            3 => Some(FaddQ(inst).into()),
            4 => Some(FsubS(inst).into()),
            5 => Some(FsubD(inst).into()),
            7 => Some(FsubQ(inst).into()),
            8 => Some(FmulS(inst).into()),
            9 => Some(FmulD(inst).into()),
            11 => Some(FmulQ(inst).into()),
            12 => Some(FdivS(inst).into()),
            13 => Some(FdivD(inst).into()),
            15 => Some(FdivQ(inst).into()),
            16 if bits!(inst[14:12]) == 0 => Some(FsgnjS(inst).into()),
            16 if bits!(inst[14:12]) == 1 => Some(FsgnjnS(inst).into()),
            16 if bits!(inst[14:12]) == 2 => Some(FsgnjxS(inst).into()),
            17 if bits!(inst[14:12]) == 0 => Some(FsgnjD(inst).into()),
            17 if bits!(inst[14:12]) == 1 => Some(FsgnjnD(inst).into()),
            17 if bits!(inst[14:12]) == 2 => Some(FsgnjxD(inst).into()),
            19 if bits!(inst[14:12]) == 0 => Some(FsgnjQ(inst).into()),
            19 if bits!(inst[14:12]) == 1 => Some(FsgnjnQ(inst).into()),
            19 if bits!(inst[14:12]) == 2 => Some(FsgnjxQ(inst).into()),
            20 if bits!(inst[14:12]) == 0 => Some(FminS(inst).into()),
            20 if bits!(inst[14:12]) == 1 => Some(FmaxS(inst).into()),
            21 if bits!(inst[14:12]) == 0 => Some(FminD(inst).into()),
            21 if bits!(inst[14:12]) == 1 => Some(FmaxD(inst).into()),
            23 if bits!(inst[14:12]) == 0 => Some(FminQ(inst).into()),
            23 if bits!(inst[14:12]) == 1 => Some(FmaxQ(inst).into()),
            32 if bits!(inst[24:20]) == 1 => Some(FcvtSD(inst).into()),
            32 if bits!(inst[24:20]) == 3 => Some(FcvtSQ(inst).into()),
            33 if bits!(inst[24:20]) == 0 => Some(FcvtDS(inst).into()),
            33 if bits!(inst[24:20]) == 3 => Some(FcvtDQ(inst).into()),
            35 if bits!(inst[24:20]) == 0 => Some(FcvtQS(inst).into()),
            35 if bits!(inst[24:20]) == 1 => Some(FcvtQD(inst).into()),
            44 if bits!(inst[24:20]) == 0 => Some(FsqrtS(inst).into()),
            45 if bits!(inst[24:20]) == 0 => Some(FsqrtD(inst).into()),
            47 if bits!(inst[24:20]) == 0 => Some(FsqrtQ(inst).into()),
            80 if bits!(inst[14:12]) == 0 => Some(FleS(inst).into()),
            80 if bits!(inst[14:12]) == 1 => Some(FltS(inst).into()),
            80 if bits!(inst[14:12]) == 2 => Some(FeqS(inst).into()),
            81 if bits!(inst[14:12]) == 0 => Some(FleD(inst).into()),
            81 if bits!(inst[14:12]) == 1 => Some(FltD(inst).into()),
            81 if bits!(inst[14:12]) == 2 => Some(FeqD(inst).into()),
            83 if bits!(inst[14:12]) == 0 => Some(FleQ(inst).into()),
            83 if bits!(inst[14:12]) == 1 => Some(FltQ(inst).into()),
            83 if bits!(inst[14:12]) == 2 => Some(FeqQ(inst).into()),
            96 if bits!(inst[24:20]) == 0 => Some(FcvtWS(inst).into()),
            96 if bits!(inst[24:20]) == 1 => Some(FcvtWuS(inst).into()),
            96 if bits!(inst[24:20]) == 2 => Some(FcvtLS(inst).into()),
            96 if bits!(inst[24:20]) == 3 => Some(FcvtLuS(inst).into()),
            97 if bits!(inst[24:20]) == 0 => Some(FcvtWD(inst).into()),
            97 if bits!(inst[24:20]) == 1 => Some(FcvtWuD(inst).into()),
            97 if bits!(inst[24:20]) == 2 => Some(FcvtLD(inst).into()),
            97 if bits!(inst[24:20]) == 3 => Some(FcvtLuD(inst).into()),
            99 if bits!(inst[24:20]) == 0 => Some(FcvtWQ(inst).into()),
            99 if bits!(inst[24:20]) == 1 => Some(FcvtWuQ(inst).into()),
            99 if bits!(inst[24:20]) == 2 => Some(FcvtLQ(inst).into()),
            99 if bits!(inst[24:20]) == 3 => Some(FcvtLuQ(inst).into()),
            104 if bits!(inst[24:20]) == 0 => Some(FcvtSW(inst).into()),
            104 if bits!(inst[24:20]) == 1 => Some(FcvtSWu(inst).into()),
            104 if bits!(inst[24:20]) == 2 => Some(FcvtSL(inst).into()),
            104 if bits!(inst[24:20]) == 3 => Some(FcvtSLu(inst).into()),
            105 if bits!(inst[24:20]) == 0 => Some(FcvtDW(inst).into()),
            105 if bits!(inst[24:20]) == 1 => Some(FcvtDWu(inst).into()),
            105 if bits!(inst[24:20]) == 2 => Some(FcvtDL(inst).into()),
            105 if bits!(inst[24:20]) == 3 => Some(FcvtDLu(inst).into()),
            107 if bits!(inst[24:20]) == 0 => Some(FcvtQW(inst).into()),
            107 if bits!(inst[24:20]) == 1 => Some(FcvtQWu(inst).into()),
            107 if bits!(inst[24:20]) == 2 => Some(FcvtQL(inst).into()),
            107 if bits!(inst[24:20]) == 3 => Some(FcvtQLu(inst).into()),
            112 if bits!(inst[24:20|14:12]) == 0 => Some(FmvXS(inst).into()),
            112 if bits!(inst[24:20|14:12]) == 1 => Some(FclassS(inst).into()),
            113 if bits!(inst[24:20|14:12]) == 0 => Some(FmvXD(inst).into()),
            113 if bits!(inst[24:20|14:12]) == 1 => Some(FclassD(inst).into()),
            115 if bits!(inst[24:20|14:12]) == 0 => Some(FmvXQ(inst).into()),
            115 if bits!(inst[24:20|14:12]) == 1 => Some(FclassQ(inst).into()),
            120 if bits!(inst[24:20|14:12]) == 0 => Some(FmvSX(inst).into()),
            121 if bits!(inst[24:20|14:12]) == 0 => Some(FmvDX(inst).into()),
            123 if bits!(inst[24:20|14:12]) == 0 => Some(FmvQX(inst).into()),
            _ => None,
        },
        0x15 => None,
        0x16 => match bits!(inst[14:12]) {
            0 => Some(Addid(inst).into()),
            1 if bits!(inst[31:26]) == 0 => Some(Sllid(inst).into()),
            5 if bits!(inst[31:26]) == 0 => Some(Srlid(inst).into()),
            5 if bits!(inst[31:26]) == 16 => Some(Sraid(inst).into()),
            2..=4 | 6..=7 => None,
            _ => unreachable!(),
        },
        0x17 => None,
        0x18 => match bits!(inst[14:12]) {
            0x0 => Some(Beq(inst).into()),
            0x1 => Some(Bne(inst).into()),
            0x4 => Some(Blt(inst).into()),
            0x5 => Some(Bge(inst).into()),
            0x6 => Some(Bltu(inst).into()),
            0x7 => Some(Bgeu(inst).into()),

            0x2..=0x3 => None,
            _ => unreachable!(),
        },
        0x19 => {
            if bits!(inst[14:12]) != 0 {
                None
            } else {
                Some(Jalr(inst).into())
            }
        }
        0x1a => None,
        0x1b => Some(Jal(inst).into()),
        0x1c => {
            if bits!(inst[14:12|31:25|11:7]) != 0 {
                None
            } else {
                match bits!(inst[24:15]) {
                    0 => Some(Ecall(inst).into()),
                    32 => Some(Ebreak(inst).into()),
                    _ => None,
                }
            }
        }
        0x1d => None,
        0x1e => match bits!(inst[31:25|14:12]) {
            0 => Some(Addd(inst).into()),
            1 => Some(Slld(inst).into()),
            5 => Some(Srld(inst).into()),
            8 => Some(Muld(inst).into()),
            12 => Some(Divd(inst).into()),
            13 => Some(Divud(inst).into()),
            14 => Some(Remd(inst).into()),
            15 => Some(Remud(inst).into()),
            256 => Some(Subd(inst).into()),
            261 => Some(Srad(inst).into()),
            _ => None,
        },
        0x1f => None,
        _ => unreachable!("Invalid opcode: {:b} (inst: {:#x})", bits!(inst[6:2]), inst),
    }
}

pub fn decode(inst: u32) -> (Option<Opcode>, u32) {
    match bits!(inst[1:0]) {
        0 => (
            decode_q4(crate::rv_c::RISCV_C2G_C0[(inst as u16 >> 2) as usize]),
            2,
        ),
        1 => (
            decode_q4(crate::rv_c::RISCV_C2G_C1[(inst as u16 >> 2) as usize]),
            2,
        ),
        2 => (
            decode_q4(crate::rv_c::RISCV_C2G_C2[(inst as u16 >> 2) as usize]),
            2,
        ),
        3 => (decode_q4(inst), 4),
        _ => unreachable!(),
    }
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::Lb(lb) => lb.fmt(f),
            Opcode::Lh(lh) => lh.fmt(f),
            Opcode::Lw(lw) => lw.fmt(f),
            Opcode::Ld(ld) => ld.fmt(f),
            Opcode::Lq(lq) => lq.fmt(f),
            Opcode::Lbu(lbu) => lbu.fmt(f),
            Opcode::Lhu(lhu) => lhu.fmt(f),
            Opcode::Lwu(lwu) => lwu.fmt(f),
            Opcode::Ldu(ldu) => ldu.fmt(f),
            Opcode::Flw(flw) => flw.fmt(f),
            Opcode::Fld(fld) => fld.fmt(f),
            Opcode::Flq(flq) => flq.fmt(f),
            Opcode::Fence(fence) => fence.fmt(f),
            Opcode::FenceI(fence_i) => fence_i.fmt(f),
            Opcode::Addi(addi) => addi.fmt(f),
            Opcode::Slti(slti) => slti.fmt(f),
            Opcode::Sltiu(sltiu) => sltiu.fmt(f),
            Opcode::Xori(xori) => xori.fmt(f),
            Opcode::Ori(ori) => ori.fmt(f),
            Opcode::Andi(andi) => andi.fmt(f),
            Opcode::Slli(slli) => slli.fmt(f),
            Opcode::Srli(srli) => srli.fmt(f),
            Opcode::Srai(srai) => srai.fmt(f),
            Opcode::Auipc(auipc) => auipc.fmt(f),
            Opcode::Addiw(addiw) => addiw.fmt(f),
            Opcode::Slliw(slliw) => slliw.fmt(f),
            Opcode::Srliw(srliw) => srliw.fmt(f),
            Opcode::Sraiw(sraiw) => sraiw.fmt(f),
            Opcode::Sb(sb) => sb.fmt(f),
            Opcode::Sh(sh) => sh.fmt(f),
            Opcode::Sw(sw) => sw.fmt(f),
            Opcode::Sd(sd) => sd.fmt(f),
            Opcode::Sq(sq) => sq.fmt(f),
            Opcode::Fsw(fsw) => fsw.fmt(f),
            Opcode::Fsd(fsd) => fsd.fmt(f),
            Opcode::Fsq(fsq) => fsq.fmt(f),
            Opcode::LrW(lr_w) => lr_w.fmt(f),
            Opcode::ScW(sc_w) => sc_w.fmt(f),
            Opcode::AmoswapW(amoswap_w) => amoswap_w.fmt(f),
            Opcode::AmoaddW(amoadd_w) => amoadd_w.fmt(f),
            Opcode::AmoxorW(amoxor_w) => amoxor_w.fmt(f),
            Opcode::AmoandW(amoand_w) => amoand_w.fmt(f),
            Opcode::AmoorW(amoor_w) => amoor_w.fmt(f),
            Opcode::AmominW(amomin_w) => amomin_w.fmt(f),
            Opcode::AmomaxW(amomax_w) => amomax_w.fmt(f),
            Opcode::AmominuW(amominu_w) => amominu_w.fmt(f),
            Opcode::AmomaxuW(amomaxu_w) => amomaxu_w.fmt(f),
            Opcode::LrD(lr_d) => lr_d.fmt(f),
            Opcode::ScD(sc_d) => sc_d.fmt(f),
            Opcode::AmoswapD(amoswap_d) => amoswap_d.fmt(f),
            Opcode::AmoaddD(amoadd_d) => amoadd_d.fmt(f),
            Opcode::AmoxorD(amoxor_d) => amoxor_d.fmt(f),
            Opcode::AmoandD(amoand_d) => amoand_d.fmt(f),
            Opcode::AmoorD(amoor_d) => amoor_d.fmt(f),
            Opcode::AmominD(amomin_d) => amomin_d.fmt(f),
            Opcode::AmomaxD(amomax_d) => amomax_d.fmt(f),
            Opcode::AmominuD(amominu_d) => amominu_d.fmt(f),
            Opcode::AmomaxuD(amomaxu_d) => amomaxu_d.fmt(f),
            Opcode::LrQ(lr_q) => lr_q.fmt(f),
            Opcode::ScQ(sc_q) => sc_q.fmt(f),
            Opcode::AmoswapQ(amoswap_q) => amoswap_q.fmt(f),
            Opcode::AmoaddQ(amoadd_q) => amoadd_q.fmt(f),
            Opcode::AmoxorQ(amoxor_q) => amoxor_q.fmt(f),
            Opcode::AmoandQ(amoand_q) => amoand_q.fmt(f),
            Opcode::AmoorQ(amoor_q) => amoor_q.fmt(f),
            Opcode::AmominQ(amomin_q) => amomin_q.fmt(f),
            Opcode::AmomaxQ(amomax_q) => amomax_q.fmt(f),
            Opcode::AmominuQ(amominu_q) => amominu_q.fmt(f),
            Opcode::AmomaxuQ(amomaxu_q) => amomaxu_q.fmt(f),
            Opcode::Add(add) => add.fmt(f),
            Opcode::Sub(sub) => sub.fmt(f),
            Opcode::Sll(sll) => sll.fmt(f),
            Opcode::Slt(slt) => slt.fmt(f),
            Opcode::Sltu(sltu) => sltu.fmt(f),
            Opcode::Xor(xor) => xor.fmt(f),
            Opcode::Srl(srl) => srl.fmt(f),
            Opcode::Sra(sra) => sra.fmt(f),
            Opcode::Or(or) => or.fmt(f),
            Opcode::And(and) => and.fmt(f),
            Opcode::Mul(mul) => mul.fmt(f),
            Opcode::Mulh(mulh) => mulh.fmt(f),
            Opcode::Mulhsu(mulhsu) => mulhsu.fmt(f),
            Opcode::Mulhu(mulhu) => mulhu.fmt(f),
            Opcode::Div(div) => div.fmt(f),
            Opcode::Divu(divu) => divu.fmt(f),
            Opcode::Rem(rem) => rem.fmt(f),
            Opcode::Remu(remu) => remu.fmt(f),
            Opcode::Lui(lui) => lui.fmt(f),
            Opcode::Addw(addw) => addw.fmt(f),
            Opcode::Subw(subw) => subw.fmt(f),
            Opcode::Sllw(sllw) => sllw.fmt(f),
            Opcode::Srlw(srlw) => srlw.fmt(f),
            Opcode::Sraw(sraw) => sraw.fmt(f),
            Opcode::Mulw(mulw) => mulw.fmt(f),
            Opcode::Divw(divw) => divw.fmt(f),
            Opcode::Divuw(divuw) => divuw.fmt(f),
            Opcode::Remw(remw) => remw.fmt(f),
            Opcode::Remuw(remuw) => remuw.fmt(f),
            Opcode::FmaddS(fmadd_s) => fmadd_s.fmt(f),
            Opcode::FmsubS(fmsub_s) => fmsub_s.fmt(f),
            Opcode::FnmsubS(fnmsub_s) => fnmsub_s.fmt(f),
            Opcode::FnmaddS(fnmadd_s) => fnmadd_s.fmt(f),
            Opcode::FaddS(fadd_s) => fadd_s.fmt(f),
            Opcode::FsubS(fsub_s) => fsub_s.fmt(f),
            Opcode::FmulS(fmul_s) => fmul_s.fmt(f),
            Opcode::FdivS(fdiv_s) => fdiv_s.fmt(f),
            Opcode::FsgnjS(fsgnj_s) => fsgnj_s.fmt(f),
            Opcode::FsgnjnS(fsgnjn_s) => fsgnjn_s.fmt(f),
            Opcode::FsgnjxS(fsgnjx_s) => fsgnjx_s.fmt(f),
            Opcode::FminS(fmin_s) => fmin_s.fmt(f),
            Opcode::FmaxS(fmax_s) => fmax_s.fmt(f),
            Opcode::FsqrtS(fsqrt_s) => fsqrt_s.fmt(f),
            Opcode::FleS(fle_s) => fle_s.fmt(f),
            Opcode::FltS(flt_s) => flt_s.fmt(f),
            Opcode::FeqS(feq_s) => feq_s.fmt(f),
            Opcode::FcvtWS(fcvt_ws) => fcvt_ws.fmt(f),
            Opcode::FcvtWuS(fcvt_wu_s) => fcvt_wu_s.fmt(f),
            Opcode::FcvtSW(fcvt_sw) => fcvt_sw.fmt(f),
            Opcode::FcvtSWu(fcvt_swu) => fcvt_swu.fmt(f),
            Opcode::FmvXS(fmv_xs) => fmv_xs.fmt(f),
            Opcode::FclassS(fclass_s) => fclass_s.fmt(f),
            Opcode::FmvSX(fmv_sx) => fmv_sx.fmt(f),
            Opcode::FcvtLS(fcvt_ls) => fcvt_ls.fmt(f),
            Opcode::FcvtLuS(fcvt_lu_s) => fcvt_lu_s.fmt(f),
            Opcode::FcvtSL(fcvt_sl) => fcvt_sl.fmt(f),
            Opcode::FcvtSLu(fcvt_slu) => fcvt_slu.fmt(f),
            Opcode::FmaddD(fmadd_d) => fmadd_d.fmt(f),
            Opcode::FmsubD(fmsub_d) => fmsub_d.fmt(f),
            Opcode::FnmsubD(fnmsub_d) => fnmsub_d.fmt(f),
            Opcode::FnmaddD(fnmadd_d) => fnmadd_d.fmt(f),
            Opcode::FaddD(fadd_d) => fadd_d.fmt(f),
            Opcode::FsubD(fsub_d) => fsub_d.fmt(f),
            Opcode::FmulD(fmul_d) => fmul_d.fmt(f),
            Opcode::FdivD(fdiv_d) => fdiv_d.fmt(f),
            Opcode::FsgnjD(fsgnj_d) => fsgnj_d.fmt(f),
            Opcode::FsgnjnD(fsgnjn_d) => fsgnjn_d.fmt(f),
            Opcode::FsgnjxD(fsgnjx_d) => fsgnjx_d.fmt(f),
            Opcode::FminD(fmin_d) => fmin_d.fmt(f),
            Opcode::FmaxD(fmax_d) => fmax_d.fmt(f),
            Opcode::FsqrtD(fsqrt_d) => fsqrt_d.fmt(f),
            Opcode::FleD(fle_d) => fle_d.fmt(f),
            Opcode::FltD(flt_d) => flt_d.fmt(f),
            Opcode::FeqD(feq_d) => feq_d.fmt(f),
            Opcode::FcvtWD(fcvt_wd) => fcvt_wd.fmt(f),
            Opcode::FcvtWuD(fcvt_wu_d) => fcvt_wu_d.fmt(f),
            Opcode::FcvtDW(fcvt_dw) => fcvt_dw.fmt(f),
            Opcode::FcvtDWu(fcvt_dwu) => fcvt_dwu.fmt(f),
            Opcode::FcvtSD(fcvt_sd) => fcvt_sd.fmt(f),
            Opcode::FcvtDS(fcvt_ds) => fcvt_ds.fmt(f),
            Opcode::FclassD(fclass_d) => fclass_d.fmt(f),
            Opcode::FcvtLD(fcvt_ld) => fcvt_ld.fmt(f),
            Opcode::FcvtLuD(fcvt_lu_d) => fcvt_lu_d.fmt(f),
            Opcode::FmvXD(fmv_xd) => fmv_xd.fmt(f),
            Opcode::FmvDX(fmv_dx) => fmv_dx.fmt(f),
            Opcode::FcvtDL(fcvt_dl) => fcvt_dl.fmt(f),
            Opcode::FcvtDLu(fcvt_dlu) => fcvt_dlu.fmt(f),
            Opcode::FmaddQ(fmadd_q) => fmadd_q.fmt(f),
            Opcode::FmsubQ(fmsub_q) => fmsub_q.fmt(f),
            Opcode::FnmsubQ(fnmsub_q) => fnmsub_q.fmt(f),
            Opcode::FnmaddQ(fnmadd_q) => fnmadd_q.fmt(f),
            Opcode::FaddQ(fadd_q) => fadd_q.fmt(f),
            Opcode::FsubQ(fsub_q) => fsub_q.fmt(f),
            Opcode::FmulQ(fmul_q) => fmul_q.fmt(f),
            Opcode::FdivQ(fdiv_q) => fdiv_q.fmt(f),
            Opcode::FsgnjQ(fsgnj_q) => fsgnj_q.fmt(f),
            Opcode::FsgnjnQ(fsgnjn_q) => fsgnjn_q.fmt(f),
            Opcode::FsgnjxQ(fsgnjx_q) => fsgnjx_q.fmt(f),
            Opcode::FminQ(fmin_q) => fmin_q.fmt(f),
            Opcode::FmaxQ(fmax_q) => fmax_q.fmt(f),
            Opcode::FsqrtQ(fsqrt_q) => fsqrt_q.fmt(f),
            Opcode::FleQ(fle_q) => fle_q.fmt(f),
            Opcode::FltQ(flt_q) => flt_q.fmt(f),
            Opcode::FeqQ(feq_q) => feq_q.fmt(f),
            Opcode::FcvtWQ(fcvt_wq) => fcvt_wq.fmt(f),
            Opcode::FcvtWuQ(fcvt_wu_q) => fcvt_wu_q.fmt(f),
            Opcode::FcvtQW(fcvt_qw) => fcvt_qw.fmt(f),
            Opcode::FcvtQWu(fcvt_qwu) => fcvt_qwu.fmt(f),
            Opcode::FcvtSQ(fcvt_sq) => fcvt_sq.fmt(f),
            Opcode::FcvtQS(fcvt_qs) => fcvt_qs.fmt(f),
            Opcode::FcvtDQ(fcvt_dq) => fcvt_dq.fmt(f),
            Opcode::FcvtQD(fcvt_qd) => fcvt_qd.fmt(f),
            Opcode::FclassQ(fclass_q) => fclass_q.fmt(f),
            Opcode::FcvtLQ(fcvt_lq) => fcvt_lq.fmt(f),
            Opcode::FcvtLuQ(fcvt_lu_q) => fcvt_lu_q.fmt(f),
            Opcode::FcvtQL(fcvt_ql) => fcvt_ql.fmt(f),
            Opcode::FcvtQLu(fcvt_qlu) => fcvt_qlu.fmt(f),
            Opcode::FmvXQ(fmv_xq) => fmv_xq.fmt(f),
            Opcode::FmvQX(fmv_qx) => fmv_qx.fmt(f),
            Opcode::Addid(addid) => addid.fmt(f),
            Opcode::Sllid(sllid) => sllid.fmt(f),
            Opcode::Srlid(srlid) => srlid.fmt(f),
            Opcode::Sraid(sraid) => sraid.fmt(f),
            Opcode::Beq(beq) => beq.fmt(f),
            Opcode::Bne(bne) => bne.fmt(f),
            Opcode::Blt(blt) => blt.fmt(f),
            Opcode::Bge(bge) => bge.fmt(f),
            Opcode::Bltu(bltu) => bltu.fmt(f),
            Opcode::Bgeu(bgeu) => bgeu.fmt(f),
            Opcode::Jalr(jalr) => jalr.fmt(f),
            Opcode::Jal(jal) => jal.fmt(f),
            Opcode::Ecall(ecall) => ecall.fmt(f),
            Opcode::Ebreak(ebreak) => ebreak.fmt(f),
            Opcode::Addd(addd) => addd.fmt(f),
            Opcode::Subd(subd) => subd.fmt(f),
            Opcode::Slld(slld) => slld.fmt(f),
            Opcode::Srld(srld) => srld.fmt(f),
            Opcode::Srad(srad) => srad.fmt(f),
            Opcode::Muld(muld) => muld.fmt(f),
            Opcode::Divd(divd) => divd.fmt(f),
            Opcode::Divud(divud) => divud.fmt(f),
            Opcode::Remd(remd) => remd.fmt(f),
            Opcode::Remud(remud) => remud.fmt(f),
            Opcode::CAddi4spn(caddi4spn) => caddi4spn.fmt(f),
            Opcode::CFld(cfld) => cfld.fmt(f),
            Opcode::CLw(clw) => clw.fmt(f),
            Opcode::CFlw(cflw) => cflw.fmt(f),
            Opcode::CFsd(cfsd) => cfsd.fmt(f),
            Opcode::CSw(csw) => csw.fmt(f),
            Opcode::CFsw(cfsw) => cfsw.fmt(f),
            Opcode::CNop(cnop) => cnop.fmt(f),
            Opcode::CAddi(caddi) => caddi.fmt(f),
            Opcode::CJal(cjal) => cjal.fmt(f),
            Opcode::CLi(cli) => cli.fmt(f),
            Opcode::CAddi16sp(caddi16sp) => caddi16sp.fmt(f),
            Opcode::CLui(clui) => clui.fmt(f),
            Opcode::CAndi(candi) => candi.fmt(f),
            Opcode::CSub(csub) => csub.fmt(f),
            Opcode::CXor(cxor) => cxor.fmt(f),
            Opcode::COr(cor) => cor.fmt(f),
            Opcode::CAnd(cand) => cand.fmt(f),
            Opcode::CSubw(csubw) => csubw.fmt(f),
            Opcode::CAddw(caddw) => caddw.fmt(f),
            Opcode::CJ(cj) => cj.fmt(f),
            Opcode::CBeqz(cbeqz) => cbeqz.fmt(f),
            Opcode::CBnez(cbnez) => cbnez.fmt(f),
            Opcode::CFldsp(cfldsp) => cfldsp.fmt(f),
            Opcode::CLwsp(clwsp) => clwsp.fmt(f),
            Opcode::CFlwsp(cflwsp) => cflwsp.fmt(f),
            Opcode::CJr(cjr) => cjr.fmt(f),
            Opcode::CMv(cmv) => cmv.fmt(f),
            Opcode::CEbreak(cebreak) => cebreak.fmt(f),
            Opcode::CJalr(cjalr) => cjalr.fmt(f),
            Opcode::CAdd(cadd) => cadd.fmt(f),
            Opcode::CFsdsp(cfsdsp) => cfsdsp.fmt(f),
            Opcode::CSwsp(cswsp) => cswsp.fmt(f),
            Opcode::CFswsp(cfswsp) => cfswsp.fmt(f),
            Opcode::CSrli(csrli) => csrli.fmt(f),
            Opcode::CSrai(csrai) => csrai.fmt(f),
            Opcode::CSlli(cslli) => cslli.fmt(f),
            Opcode::CLd(cld) => cld.fmt(f),
            Opcode::CSd(csd) => csd.fmt(f),
            Opcode::CAddiw(caddiw) => caddiw.fmt(f),
            Opcode::CLdsp(cldsp) => cldsp.fmt(f),
            Opcode::CSdsp(csdsp) => csdsp.fmt(f),
            Opcode::CLq(clq) => clq.fmt(f),
            Opcode::CSq(csq) => csq.fmt(f),
            Opcode::CLqsp(clqsp) => clqsp.fmt(f),
            Opcode::CSqsp(csqsp) => csqsp.fmt(f),
        }
    }
}
