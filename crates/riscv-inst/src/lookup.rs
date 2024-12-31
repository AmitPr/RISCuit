use riscv_inst_macros::{bits, instructions};

#[instructions]
pub enum Opcode {
    // Load instructions
    #[fields(rd, rs1, oimm12)]
    #[isa(base = "RV32I")]
    Lb,
    #[fields(rd, rs1, oimm12)]
    #[isa(base = "RV32I")]
    Lh,
    #[fields(rd, rs1, oimm12)]
    #[isa(base = "RV32I")]
    Lw,
    #[fields(rd, rs1, oimm12)]
    #[isa(base = "RV64I")]
    Ld,
    #[fields(rd, rs1, oimm12)]
    #[isa(base = "RV128I")]
    Lq,
    #[fields(rd, rs1, oimm12)]
    #[isa(base = "RV32I")]
    Lbu,
    #[fields(rd, rs1, oimm12)]
    #[isa(base = "RV32I")]
    Lhu,
    #[fields(rd, rs1, oimm12)]
    #[isa(base = "RV64I")]
    Lwu,
    #[fields(rd, rs1, oimm12)]
    #[isa(base = "RV128I")]
    Ldu,

    // Floating point loads
    #[fields(frd, rs1, oimm12)]
    #[isa(base = "RV32I", ext = "F")]
    Flw,
    #[fields(frd, rs1, oimm12)]
    #[isa(base = "RV32I", ext = "D")]
    Fld,
    #[fields(frd, rs1, oimm12)]
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
    #[fields(rd, rs1, imm12)]
    #[isa(base = "RV32I")]
    Addi,
    #[fields(rd, rs1, imm12)]
    #[isa(base = "RV32I")]
    Slti,
    #[fields(rd, rs1, imm12)]
    #[isa(base = "RV32I")]
    Sltiu,
    #[fields(rd, rs1, imm12)]
    #[isa(base = "RV32I")]
    Xori,
    #[fields(rd, rs1, imm12)]
    #[isa(base = "RV32I")]
    Ori,
    #[fields(rd, rs1, imm12)]
    #[isa(base = "RV32I")]
    Andi,
    #[fields(rd, rs1, shamt5)]
    #[isa(base = "RV32I")]
    Slli,
    #[fields(rd, rs1, shamt5)]
    #[isa(base = "RV32I")]
    Srli,
    #[fields(rd, rs1, shamt5)]
    #[isa(base = "RV32I")]
    Srai,

    #[fields(rd, oimm20)]
    #[isa(base = "RV32I")]
    Auipc,

    // RV64I immediate arithmetic
    #[fields(rd, rs1, imm12)]
    #[isa(base = "RV64I")]
    Addiw,
    #[fields(rd, rs1, shamt5)]
    #[isa(base = "RV64I")]
    Slliw,
    #[fields(rd, rs1, shamt5)]
    #[isa(base = "RV64I")]
    Srliw,
    #[fields(rd, rs1, shamt5)]
    #[isa(base = "RV64I")]
    Sraiw,

    // Store instructions
    #[fields(rs1, rs2, simm12)]
    #[isa(base = "RV32I")]
    Sb,
    #[fields(rs1, rs2, simm12)]
    #[isa(base = "RV32I")]
    Sh,
    #[fields(rs1, rs2, simm12)]
    #[isa(base = "RV32I")]
    Sw,
    #[fields(rs1, rs2, simm12)]
    #[isa(base = "RV64I")]
    Sd,
    #[fields(rs1, rs2, simm12)]
    #[isa(base = "RV128I")]
    Sq,

    // Floating point stores
    #[fields(rs1, frs2, simm12)]
    #[isa(base = "RV32I", ext = "F")]
    Fsw,
    #[fields(rs1, frs2, simm12)]
    #[isa(base = "RV32I", ext = "D")]
    Fsd,
    #[fields(rs1, frs2, simm12)]
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

    #[fields(rd, imm20)]
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
    #[fields(rd, rs1, imm12)]
    #[isa(base = "RV128I")]
    Addid,
    #[fields(rd, rs1, shamt6)]
    #[isa(base = "RV128I")]
    Sllid,
    #[fields(rd, rs1, shamt6)]
    #[isa(base = "RV128I")]
    Srlid,
    #[fields(rd, rs1, shamt6)]
    #[isa(base = "RV128I")]
    Sraid,

    // Branch instructions
    #[fields(rs1, rs2, sbimm12)]
    #[isa(base = "RV32I")]
    Beq,
    #[fields(rs1, rs2, sbimm12)]
    #[isa(base = "RV32I")]
    Bne,
    #[fields(rs1, rs2, sbimm12)]
    #[isa(base = "RV32I")]
    Blt,
    #[fields(rs1, rs2, sbimm12)]
    #[isa(base = "RV32I")]
    Bge,
    #[fields(rs1, rs2, sbimm12)]
    #[isa(base = "RV32I")]
    Bltu,
    #[fields(rs1, rs2, sbimm12)]
    #[isa(base = "RV32I")]
    Bgeu,

    // Jump instructions
    #[fields(rd, rs1, oimm12)]
    #[isa(base = "RV32I")]
    Jalr,
    #[fields(rd, jimm20)]
    #[isa(base = "RV32I")]
    Jal,

    // System instructions
    #[fields()]
    #[isa(base = "RV32I")]
    Ecall,
    #[fields()]
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
}

// Decode instructions in the fourth quadrant, e.g. ending in 0b11:
// These are the full 32-bit instructions.
pub fn decode_q4(inst: u32) -> Option<Opcode> {
    match bits!(inst[7:2]) {
        0x0 => match bits!(inst[14:12]) {
            0x0 => Some(Lb.into()),
            0x1 => Some(Lh.into()),
            0x2 => Some(Lw.into()),
            0x3 => Some(Ld.into()),
            0x4 => Some(Lbu.into()),
            0x5 => Some(Lhu.into()),
            0x6 => Some(Lwu.into()),
            0x7 => Some(Ldu.into()),
            _ => unreachable!(),
        },
        0x1 => match bits!(inst[14:12]) {
            0x2 => Some(Flw.into()),
            0x3 => Some(Fld.into()),
            0x4 => Some(Flq.into()),
            0x0..=0x1 | 0x5..=0x7 => None,
            _ => unreachable!(),
        },
        0x2 => None,
        0x3 => match bits!(inst[14:12]) {
            0x0 => Some(Fence.into()),
            0x1 => Some(FenceI.into()),
            0x2 => Some(Lq.into()),
            0x3..=0x7 => None,
            _ => unreachable!(),
        },
        0x4 => match bits!(inst[14:12]) {
            0x0 => Some(Addi.into()),
            0x1 if bits!(inst[31:27]) == 0 => Some(Slli.into()),
            0x2 => Some(Slti.into()),
            0x3 => Some(Sltiu.into()),
            0x4 => Some(Xori.into()),
            0x5 if bits!(inst[31:27]) == 0 => Some(Srli.into()),
            0x5 if bits!(inst[31:27]) == 8 => Some(Srai.into()),
            0x6 => Some(Ori.into()),
            0x7 => Some(Andi.into()),
            _ => unreachable!(),
        },
        0x5 => Some(Auipc.into()),
        0x6 => match bits!(inst[14:12]) {
            0x0 => Some(Addiw.into()),
            0x1 if (inst >> 25) == 0 => Some(Slliw.into()),
            0x5 if (inst >> 25) == 0 => Some(Srliw.into()),
            0x5 if (inst >> 25) == 8 => Some(Sraiw.into()),
            0x2..=0x7 => None,
            _ => unreachable!(),
        },
        0x7 => None,
        0x8 => match bits!(inst[14:12]) {
            0x0 => Some(Sb.into()),
            0x1 => Some(Sh.into()),
            0x2 => Some(Sw.into()),
            0x3 => Some(Sd.into()),
            0x4 => Some(Sq.into()),
            0x5..=0x7 => None,
            _ => unreachable!(),
        },
        0x9 => match bits!(inst[14:12]) {
            0x2 => Some(Fsw.into()),
            0x3 => Some(Fsd.into()),
            0x4 => Some(Fsq.into()),
            0x0..=0x1 | 0x5..=0x7 => None,
            _ => unreachable!(),
        },
        0xa => None,
        0xb => {
            match bits!(inst[31:27|14:12]) {
                2 => Some(AmoaddW.into()),
                3 => Some(AmoaddD.into()),
                4 => Some(AmoaddQ.into()),
                10 => Some(AmoswapW.into()),
                11 => Some(AmoswapD.into()),
                12 => Some(AmoswapQ.into()),

                // LR instructions need to check rs2 field (inst[24:20]) is zero
                18 if bits!(inst[24:20]) == 0 => Some(LrW.into()),
                19 if bits!(inst[24:20]) == 0 => Some(LrD.into()),
                20 if bits!(inst[24:20]) == 0 => Some(LrQ.into()),

                26 => Some(ScW.into()),
                27 => Some(ScD.into()),
                28 => Some(ScQ.into()),

                34 => Some(AmoxorW.into()),
                35 => Some(AmoxorD.into()),
                36 => Some(AmoxorQ.into()),

                66 => Some(AmoorW.into()),
                67 => Some(AmoorD.into()),
                68 => Some(AmoorQ.into()),

                98 => Some(AmoandW.into()),
                99 => Some(AmoandD.into()),
                100 => Some(AmoandQ.into()),

                130 => Some(AmominW.into()),
                131 => Some(AmominD.into()),
                132 => Some(AmominQ.into()),

                162 => Some(AmomaxW.into()),
                163 => Some(AmomaxD.into()),
                164 => Some(AmomaxQ.into()),

                194 => Some(AmominuW.into()),
                195 => Some(AmominuD.into()),
                196 => Some(AmominuQ.into()),

                226 => Some(AmomaxuW.into()),
                227 => Some(AmomaxuD.into()),
                228 => Some(AmomaxuQ.into()),

                _ => None,
            }
        }
        0xc => match bits!(inst[31:25|14:12]) {
            0 => Some(Add.into()),
            1 => Some(Sll.into()),
            2 => Some(Slt.into()),
            3 => Some(Sltu.into()),
            4 => Some(Xor.into()),
            5 => Some(Srl.into()),
            6 => Some(Or.into()),
            7 => Some(And.into()),
            8 => Some(Mul.into()),
            9 => Some(Mulh.into()),
            10 => Some(Mulhsu.into()),
            11 => Some(Mulhu.into()),
            12 => Some(Div.into()),
            13 => Some(Divu.into()),
            14 => Some(Rem.into()),
            15 => Some(Remu.into()),
            256 => Some(Sub.into()),
            261 => Some(Sra.into()),
            _ => None,
        },
        0xd => Some(Lui.into()),
        0xe => match bits!(inst[31:25|14:12]) {
            0 => Some(Addw.into()),
            1 => Some(Sllw.into()),
            5 => Some(Srlw.into()),
            8 => Some(Mulw.into()),
            12 => Some(Divw.into()),
            13 => Some(Divuw.into()),
            14 => Some(Remw.into()),
            15 => Some(Remuw.into()),
            256 => Some(Subw.into()),
            261 => Some(Sraw.into()),
            _ => None,
        },
        0xf => None,
        ty @ 0x10..=0x13 => match (ty, bits!(inst[26:5])) {
            (0x10, 0x0) => Some(FmaddS.into()),
            (0x10, 0x1) => Some(FmaddD.into()),
            (0x10, 0x3) => Some(FmaddQ.into()),

            (0x11, 0x0) => Some(FmsubS.into()),
            (0x11, 0x1) => Some(FmsubD.into()),
            (0x11, 0x3) => Some(FmsubQ.into()),

            (0x12, 0x0) => Some(FnmsubS.into()),
            (0x12, 0x1) => Some(FnmsubD.into()),
            (0x12, 0x3) => Some(FnmsubQ.into()),

            (0x13, 0x0) => Some(FnmaddS.into()),
            (0x13, 0x1) => Some(FnmaddD.into()),
            (0x13, 0x3) => Some(FnmaddQ.into()),

            _ => None,
        },
        0x14 => match bits!(inst[31:25]) {
            0 => Some(FaddS.into()),
            1 => Some(FaddD.into()),
            3 => Some(FaddQ.into()),
            4 => Some(FsubS.into()),
            5 => Some(FsubD.into()),
            7 => Some(FsubQ.into()),
            8 => Some(FmulS.into()),
            9 => Some(FmulD.into()),
            11 => Some(FmulQ.into()),
            12 => Some(FdivS.into()),
            13 => Some(FdivD.into()),
            15 => Some(FdivQ.into()),
            16 if bits!(inst[14:12]) == 0 => Some(FsgnjS.into()),
            16 if bits!(inst[14:12]) == 1 => Some(FsgnjnS.into()),
            16 if bits!(inst[14:12]) == 2 => Some(FsgnjxS.into()),
            17 if bits!(inst[14:12]) == 0 => Some(FsgnjD.into()),
            17 if bits!(inst[14:12]) == 1 => Some(FsgnjnD.into()),
            17 if bits!(inst[14:12]) == 2 => Some(FsgnjxD.into()),
            19 if bits!(inst[14:12]) == 0 => Some(FsgnjQ.into()),
            19 if bits!(inst[14:12]) == 1 => Some(FsgnjnQ.into()),
            19 if bits!(inst[14:12]) == 2 => Some(FsgnjxQ.into()),
            20 if bits!(inst[14:12]) == 0 => Some(FminS.into()),
            20 if bits!(inst[14:12]) == 1 => Some(FmaxS.into()),
            21 if bits!(inst[14:12]) == 0 => Some(FminD.into()),
            21 if bits!(inst[14:12]) == 1 => Some(FmaxD.into()),
            23 if bits!(inst[14:12]) == 0 => Some(FminQ.into()),
            23 if bits!(inst[14:12]) == 1 => Some(FmaxQ.into()),
            32 if bits!(inst[24:20]) == 1 => Some(FcvtSD.into()),
            32 if bits!(inst[24:20]) == 3 => Some(FcvtSQ.into()),
            33 if bits!(inst[24:20]) == 0 => Some(FcvtDS.into()),
            33 if bits!(inst[24:20]) == 3 => Some(FcvtDQ.into()),
            35 if bits!(inst[24:20]) == 0 => Some(FcvtQS.into()),
            35 if bits!(inst[24:20]) == 1 => Some(FcvtQD.into()),
            44 if bits!(inst[24:20]) == 0 => Some(FsqrtS.into()),
            45 if bits!(inst[24:20]) == 0 => Some(FsqrtD.into()),
            47 if bits!(inst[24:20]) == 0 => Some(FsqrtQ.into()),
            80 if bits!(inst[14:12]) == 0 => Some(FleS.into()),
            80 if bits!(inst[14:12]) == 1 => Some(FltS.into()),
            80 if bits!(inst[14:12]) == 2 => Some(FeqS.into()),
            81 if bits!(inst[14:12]) == 0 => Some(FleD.into()),
            81 if bits!(inst[14:12]) == 1 => Some(FltD.into()),
            81 if bits!(inst[14:12]) == 2 => Some(FeqD.into()),
            83 if bits!(inst[14:12]) == 0 => Some(FleQ.into()),
            83 if bits!(inst[14:12]) == 1 => Some(FltQ.into()),
            83 if bits!(inst[14:12]) == 2 => Some(FeqQ.into()),
            96 if bits!(inst[24:20]) == 0 => Some(FcvtWS.into()),
            96 if bits!(inst[24:20]) == 1 => Some(FcvtWuS.into()),
            96 if bits!(inst[24:20]) == 2 => Some(FcvtLS.into()),
            96 if bits!(inst[24:20]) == 3 => Some(FcvtLuS.into()),
            97 if bits!(inst[24:20]) == 0 => Some(FcvtWD.into()),
            97 if bits!(inst[24:20]) == 1 => Some(FcvtWuD.into()),
            97 if bits!(inst[24:20]) == 2 => Some(FcvtLD.into()),
            97 if bits!(inst[24:20]) == 3 => Some(FcvtLuD.into()),
            99 if bits!(inst[24:20]) == 0 => Some(FcvtWQ.into()),
            99 if bits!(inst[24:20]) == 1 => Some(FcvtWuQ.into()),
            99 if bits!(inst[24:20]) == 2 => Some(FcvtLQ.into()),
            99 if bits!(inst[24:20]) == 3 => Some(FcvtLuQ.into()),
            104 if bits!(inst[24:20]) == 0 => Some(FcvtSW.into()),
            104 if bits!(inst[24:20]) == 1 => Some(FcvtSWu.into()),
            104 if bits!(inst[24:20]) == 2 => Some(FcvtSL.into()),
            104 if bits!(inst[24:20]) == 3 => Some(FcvtSLu.into()),
            105 if bits!(inst[24:20]) == 0 => Some(FcvtDW.into()),
            105 if bits!(inst[24:20]) == 1 => Some(FcvtDWu.into()),
            105 if bits!(inst[24:20]) == 2 => Some(FcvtDL.into()),
            105 if bits!(inst[24:20]) == 3 => Some(FcvtDLu.into()),
            107 if bits!(inst[24:20]) == 0 => Some(FcvtQW.into()),
            107 if bits!(inst[24:20]) == 1 => Some(FcvtQWu.into()),
            107 if bits!(inst[24:20]) == 2 => Some(FcvtQL.into()),
            107 if bits!(inst[24:20]) == 3 => Some(FcvtQLu.into()),
            112 if bits!(inst[24:20|14:12]) == 0 => Some(FmvXS.into()),
            112 if bits!(inst[24:20|14:12]) == 1 => Some(FclassS.into()),
            113 if bits!(inst[24:20|14:12]) == 0 => Some(FmvXD.into()),
            113 if bits!(inst[24:20|14:12]) == 1 => Some(FclassD.into()),
            115 if bits!(inst[24:20|14:12]) == 0 => Some(FmvXQ.into()),
            115 if bits!(inst[24:20|14:12]) == 1 => Some(FclassQ.into()),
            120 if bits!(inst[24:20|14:12]) == 0 => Some(FmvSX.into()),
            121 if bits!(inst[24:20|14:12]) == 0 => Some(FmvDX.into()),
            123 if bits!(inst[24:20|14:12]) == 0 => Some(FmvQX.into()),
            _ => None,
        },
        0x15 => None,
        0x16 => match bits!(inst[14:12]) {
            0 => Some(Addid.into()),
            1 if bits!(inst[31:26]) == 0 => Some(Sllid.into()),
            5 if bits!(inst[31:26]) == 0 => Some(Srlid.into()),
            5 if bits!(inst[31:26]) == 16 => Some(Sraid.into()),
            2..=4 | 6..=7 => None,
            _ => unreachable!(),
        },
        0x17 => None,
        0x18 => match bits!(inst[14:12]) {
            0x0 => Some(Beq.into()),
            0x1 => Some(Bne.into()),
            0x4 => Some(Blt.into()),
            0x5 => Some(Bge.into()),
            0x6 => Some(Bltu.into()),
            0x7 => Some(Bgeu.into()),

            0x2..=0x3 => None,
            _ => unreachable!(),
        },
        0x19 => {
            if bits!(inst[14:12]) != 0 {
                None
            } else {
                Some(Jalr.into())
            }
        }
        0x1a => None,
        0x1b => Some(Jal.into()),
        0x1c => {
            if bits!(inst[14:12|31:25|11:7]) != 0 {
                None
            } else {
                match bits!(inst[24:15]) {
                    0 => Some(Ecall.into()),
                    32 => Some(Ebreak.into()),
                    _ => None,
                }
            }
        }
        0x1d => None,
        0x1e => match bits!(inst[31:25|14:12]) {
            0 => Some(Addd.into()),
            1 => Some(Slld.into()),
            5 => Some(Srld.into()),
            8 => Some(Muld.into()),
            12 => Some(Divd.into()),
            13 => Some(Divud.into()),
            14 => Some(Remd.into()),
            15 => Some(Remud.into()),
            256 => Some(Subd.into()),
            261 => Some(Srad.into()),
            _ => None,
        },
        0x1f => None,
        _ => unreachable!(),
    }
}
