use std::fmt::Debug;

use crate::*;

impl Debug for RType<IntRegFunc3> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RType")
            .field("rd", &self.rd())
            .field("rs1", &self.rs1())
            .field("rs2", &self.rs2())
            .field("funct3", &self.funct3())
            .field("funct7", &self.funct7())
            .finish()
    }
}

impl Debug for IType<IntImmFunc3> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IType")
            .field("rd", &self.rd())
            .field("rs1", &self.rs1())
            .field("signed_imm", &self.signed_imm())
            .field("funct3", &self.funct3())
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

impl Debug for SType<LoadStoreFunc3, LImm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SType")
            .field("rs1", &self.rs1())
            .field("rs2", &self.rs2())
            .field("signed_imm", &self.signed_imm())
            .field("funct3", &self.funct3())
            .finish()
    }
}

impl Debug for SType<LoadStoreFunc3, SImm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SType")
            .field("rs1", &self.rs1())
            .field("rs2", &self.rs2())
            .field("signed_imm", &self.signed_imm())
            .field("funct3", &self.funct3())
            .finish()
    }
}

impl Debug for SType<BranchFunc3, BImm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SType")
            .field("rs1", &self.rs1())
            .field("rs2", &self.rs2())
            .field("signed_imm", &self.signed_imm())
            .field("funct3", &self.funct3())
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
