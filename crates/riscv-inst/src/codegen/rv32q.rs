//! Autogenerated by riscv-inst-codegen
//! DO NOT EDIT

#[allow(unused_imports)]
use super::{FReg, Reg};
pub enum Rv32q {
    Flq(Flq),
    Fsq(Fsq),
    FmaddQ(FmaddQ),
    FmsubQ(FmsubQ),
    FnmsubQ(FnmsubQ),
    FnmaddQ(FnmaddQ),
    FaddQ(FaddQ),
    FsubQ(FsubQ),
    FmulQ(FmulQ),
    FdivQ(FdivQ),
    FsgnjQ(FsgnjQ),
    FsgnjnQ(FsgnjnQ),
    FsgnjxQ(FsgnjxQ),
    FminQ(FminQ),
    FmaxQ(FmaxQ),
    FcvtSQ(FcvtSQ),
    FcvtQS(FcvtQS),
    FcvtDQ(FcvtDQ),
    FcvtQD(FcvtQD),
    FsqrtQ(FsqrtQ),
    FleQ(FleQ),
    FltQ(FltQ),
    FeqQ(FeqQ),
    FcvtWQ(FcvtWQ),
    FcvtWuQ(FcvtWuQ),
    FcvtQW(FcvtQW),
    FcvtQWu(FcvtQWu),
    FclassQ(FclassQ),
}
pub struct Flq(pub u32);
impl Flq {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        ((((self.0 >> 20) & 0b111111111111) << 20) as i32) >> 20
    }
}
impl std::fmt::Debug for Flq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("flq"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(imm), &self.imm())
            .finish()
    }
}
impl std::fmt::Display for Flq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "flq")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.imm())?;
        Ok(())
    }
}
pub struct Fsq(pub u32);
impl Fsq {
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn imm(&self) -> i32 {
        (((((self.0 >> 20) & 0b111111100000) | ((self.0 >> 7) & 0b11111)) << 20)
            as i32)
            >> 20
    }
}
impl std::fmt::Debug for Fsq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fsq"))
            .field("inst", &self.0)
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(frs2), &self.frs2())
            .field(stringify!(imm), &self.imm())
            .finish()
    }
}
impl std::fmt::Display for Fsq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fsq")?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.frs2())?;
        write!(f, " {:?}", self.imm())?;
        Ok(())
    }
}
pub struct FmaddQ(pub u32);
impl FmaddQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs3(&self) -> FReg {
        {
            let acc = (self.0 >> 27) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FmaddQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fmadd.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .field(stringify!(frs3), &self.frs3())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FmaddQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fmadd.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        write!(f, " {:?}", self.frs3())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FmsubQ(pub u32);
impl FmsubQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs3(&self) -> FReg {
        {
            let acc = (self.0 >> 27) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FmsubQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fmsub.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .field(stringify!(frs3), &self.frs3())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FmsubQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fmsub.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        write!(f, " {:?}", self.frs3())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FnmsubQ(pub u32);
impl FnmsubQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs3(&self) -> FReg {
        {
            let acc = (self.0 >> 27) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FnmsubQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fnmsub.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .field(stringify!(frs3), &self.frs3())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FnmsubQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fnmsub.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        write!(f, " {:?}", self.frs3())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FnmaddQ(pub u32);
impl FnmaddQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs3(&self) -> FReg {
        {
            let acc = (self.0 >> 27) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FnmaddQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fnmadd.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .field(stringify!(frs3), &self.frs3())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FnmaddQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fnmadd.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        write!(f, " {:?}", self.frs3())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FaddQ(pub u32);
impl FaddQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FaddQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fadd.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FaddQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fadd.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FsubQ(pub u32);
impl FsubQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FsubQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fsub.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FsubQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fsub.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FmulQ(pub u32);
impl FmulQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FmulQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fmul.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FmulQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fmul.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FdivQ(pub u32);
impl FdivQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FdivQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fdiv.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FdivQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fdiv.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FsgnjQ(pub u32);
impl FsgnjQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
impl std::fmt::Debug for FsgnjQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fsgnj.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .finish()
    }
}
impl std::fmt::Display for FsgnjQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fsgnj.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        Ok(())
    }
}
pub struct FsgnjnQ(pub u32);
impl FsgnjnQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
impl std::fmt::Debug for FsgnjnQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fsgnjn.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .finish()
    }
}
impl std::fmt::Display for FsgnjnQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fsgnjn.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        Ok(())
    }
}
pub struct FsgnjxQ(pub u32);
impl FsgnjxQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
impl std::fmt::Debug for FsgnjxQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fsgnjx.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .finish()
    }
}
impl std::fmt::Display for FsgnjxQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fsgnjx.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        Ok(())
    }
}
pub struct FminQ(pub u32);
impl FminQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
impl std::fmt::Debug for FminQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fmin.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .finish()
    }
}
impl std::fmt::Display for FminQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fmin.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        Ok(())
    }
}
pub struct FmaxQ(pub u32);
impl FmaxQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
impl std::fmt::Debug for FmaxQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fmax.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .finish()
    }
}
impl std::fmt::Display for FmaxQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fmax.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        Ok(())
    }
}
pub struct FcvtSQ(pub u32);
impl FcvtSQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FcvtSQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fcvt.s.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FcvtSQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fcvt.s.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FcvtQS(pub u32);
impl FcvtQS {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FcvtQS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fcvt.q.s"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FcvtQS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fcvt.q.s")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FcvtDQ(pub u32);
impl FcvtDQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FcvtDQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fcvt.d.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FcvtDQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fcvt.d.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FcvtQD(pub u32);
impl FcvtQD {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FcvtQD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fcvt.q.d"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FcvtQD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fcvt.q.d")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FsqrtQ(pub u32);
impl FsqrtQ {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FsqrtQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fsqrt.q"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FsqrtQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fsqrt.q")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FleQ(pub u32);
impl FleQ {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
impl std::fmt::Debug for FleQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fle.q"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .finish()
    }
}
impl std::fmt::Display for FleQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fle.q")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        Ok(())
    }
}
pub struct FltQ(pub u32);
impl FltQ {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
impl std::fmt::Debug for FltQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("flt.q"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .finish()
    }
}
impl std::fmt::Display for FltQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "flt.q")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        Ok(())
    }
}
pub struct FeqQ(pub u32);
impl FeqQ {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs2(&self) -> FReg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
impl std::fmt::Debug for FeqQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("feq.q"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(frs2), &self.frs2())
            .finish()
    }
}
impl std::fmt::Display for FeqQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "feq.q")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.frs2())?;
        Ok(())
    }
}
pub struct FcvtWQ(pub u32);
impl FcvtWQ {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FcvtWQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fcvt.w.q"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FcvtWQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fcvt.w.q")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FcvtWuQ(pub u32);
impl FcvtWuQ {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FcvtWuQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fcvt.wu.q"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(frs1), &self.frs1())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FcvtWuQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fcvt.wu.q")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.frs1())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FcvtQW(pub u32);
impl FcvtQW {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FcvtQW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fcvt.q.w"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FcvtQW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fcvt.q.w")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FcvtQWu(pub u32);
impl FcvtQWu {
    #[inline]
    pub const fn frd(&self) -> FReg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rs1(&self) -> Reg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn rm(&self) -> u32 {
        (self.0 >> 12) & 0b111
    }
}
impl std::fmt::Debug for FcvtQWu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fcvt.q.wu"))
            .field("inst", &self.0)
            .field(stringify!(frd), &self.frd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rm), &self.rm())
            .finish()
    }
}
impl std::fmt::Display for FcvtQWu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fcvt.q.wu")?;
        write!(f, " {:?}", self.frd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rm())?;
        Ok(())
    }
}
pub struct FclassQ(pub u32);
impl FclassQ {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn frs1(&self) -> FReg {
        {
            let acc = (self.0 >> 15) & 0b11111;
            unsafe { FReg::from_u5(acc as u8) }
        }
    }
}
impl std::fmt::Debug for FclassQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("fclass.q"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(frs1), &self.frs1())
            .finish()
    }
}
impl std::fmt::Display for FclassQ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fclass.q")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.frs1())?;
        Ok(())
    }
}
impl std::fmt::Debug for Rv32q {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rv32q::Flq(inst) => write!(f, "{inst:?}"),
            Rv32q::Fsq(inst) => write!(f, "{inst:?}"),
            Rv32q::FmaddQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FmsubQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FnmsubQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FnmaddQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FaddQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FsubQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FmulQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FdivQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FsgnjQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FsgnjnQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FsgnjxQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FminQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FmaxQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FcvtSQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FcvtQS(inst) => write!(f, "{inst:?}"),
            Rv32q::FcvtDQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FcvtQD(inst) => write!(f, "{inst:?}"),
            Rv32q::FsqrtQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FleQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FltQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FeqQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FcvtWQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FcvtWuQ(inst) => write!(f, "{inst:?}"),
            Rv32q::FcvtQW(inst) => write!(f, "{inst:?}"),
            Rv32q::FcvtQWu(inst) => write!(f, "{inst:?}"),
            Rv32q::FclassQ(inst) => write!(f, "{inst:?}"),
        }
    }
}
impl std::fmt::Display for Rv32q {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rv32q::Flq(inst) => write!(f, "{inst}"),
            Rv32q::Fsq(inst) => write!(f, "{inst}"),
            Rv32q::FmaddQ(inst) => write!(f, "{inst}"),
            Rv32q::FmsubQ(inst) => write!(f, "{inst}"),
            Rv32q::FnmsubQ(inst) => write!(f, "{inst}"),
            Rv32q::FnmaddQ(inst) => write!(f, "{inst}"),
            Rv32q::FaddQ(inst) => write!(f, "{inst}"),
            Rv32q::FsubQ(inst) => write!(f, "{inst}"),
            Rv32q::FmulQ(inst) => write!(f, "{inst}"),
            Rv32q::FdivQ(inst) => write!(f, "{inst}"),
            Rv32q::FsgnjQ(inst) => write!(f, "{inst}"),
            Rv32q::FsgnjnQ(inst) => write!(f, "{inst}"),
            Rv32q::FsgnjxQ(inst) => write!(f, "{inst}"),
            Rv32q::FminQ(inst) => write!(f, "{inst}"),
            Rv32q::FmaxQ(inst) => write!(f, "{inst}"),
            Rv32q::FcvtSQ(inst) => write!(f, "{inst}"),
            Rv32q::FcvtQS(inst) => write!(f, "{inst}"),
            Rv32q::FcvtDQ(inst) => write!(f, "{inst}"),
            Rv32q::FcvtQD(inst) => write!(f, "{inst}"),
            Rv32q::FsqrtQ(inst) => write!(f, "{inst}"),
            Rv32q::FleQ(inst) => write!(f, "{inst}"),
            Rv32q::FltQ(inst) => write!(f, "{inst}"),
            Rv32q::FeqQ(inst) => write!(f, "{inst}"),
            Rv32q::FcvtWQ(inst) => write!(f, "{inst}"),
            Rv32q::FcvtWuQ(inst) => write!(f, "{inst}"),
            Rv32q::FcvtQW(inst) => write!(f, "{inst}"),
            Rv32q::FcvtQWu(inst) => write!(f, "{inst}"),
            Rv32q::FclassQ(inst) => write!(f, "{inst}"),
        }
    }
}
