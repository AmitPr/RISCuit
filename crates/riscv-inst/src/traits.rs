use std::fmt::Debug;

use crate::*;

impl Debug for RType<IntRegOp> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RType")
            .field("rd", &self.rd())
            .field("rs1", &self.rs1())
            .field("rs2", &self.rs2())
            .field("op", &self.op())
            .finish()
    }
}

impl Debug for IType<IntImmOp> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IType")
            .field("rd", &self.rd())
            .field("rs1", &self.rs1())
            .field("signed_imm", &self.signed_imm())
            .field("op", &self.op())
            .finish()
    }
}

impl Debug for IType<JalrInst> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IType")
            .field("rd", &self.rd())
            .field("rs1", &self.rs1())
            .field("signed_imm", &self.signed_imm())
            .finish()
    }
}

impl Debug for IType<EcallInst> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IType")
            .field("rd", &self.rd())
            .field("rs1", &self.rs1())
            .field("signed_imm", &self.signed_imm())
            .finish()
    }
}

impl Debug for SType<LoadSize, LImm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SType")
            .field("rs1", &self.rs1())
            .field("rs2", &self.rs2())
            .field("signed_imm", &self.signed_imm())
            .field("op", &self.op())
            .finish()
    }
}

impl Debug for SType<StoreSize, SImm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SType")
            .field("rs1", &self.rs1())
            .field("rs2", &self.rs2())
            .field("signed_imm", &self.signed_imm())
            .field("op", &self.op())
            .finish()
    }
}

impl Debug for SType<BranchOp, BImm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SType")
            .field("rs1", &self.rs1())
            .field("rs2", &self.rs2())
            .field("signed_imm", &self.signed_imm())
            .field("op", &self.op())
            .finish()
    }
}

impl Debug for UType<UImm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UType")
            .field("rd", &self.rd())
            .field("signed_imm", &self.signed_imm())
            .finish()
    }
}

impl Debug for UType<JImm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UType")
            .field("rd", &self.rd())
            .field("signed_imm", &self.signed_imm())
            .finish()
    }
}

impl Debug for UType<UpperImm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UType")
            .field("rd", &self.rd())
            .field("signed_imm", &self.signed_imm())
            .finish()
    }
}
