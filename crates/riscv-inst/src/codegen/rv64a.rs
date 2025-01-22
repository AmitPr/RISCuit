//! Autogenerated by riscv-inst-codegen
//! DO NOT EDIT

#[allow(unused_imports)]
use super::{FReg, Reg};
pub enum Rv64a {
    LrW(LrW),
    ScW(ScW),
    AmoswapW(AmoswapW),
    AmoaddW(AmoaddW),
    AmoxorW(AmoxorW),
    AmoorW(AmoorW),
    AmoandW(AmoandW),
    AmominW(AmominW),
    AmomaxW(AmomaxW),
    AmominuW(AmominuW),
    AmomaxuW(AmomaxuW),
    LrD(LrD),
    ScD(ScD),
    AmoswapD(AmoswapD),
    AmoaddD(AmoaddD),
    AmoxorD(AmoxorD),
    AmoorD(AmoorD),
    AmoandD(AmoandD),
    AmominD(AmominD),
    AmomaxD(AmomaxD),
    AmominuD(AmominuD),
    AmomaxuD(AmomaxuD),
}
pub struct LrW(pub u32);
impl LrW {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for LrW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("lr.w"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for LrW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lr.w")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct ScW(pub u32);
impl ScW {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for ScW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("sc.w"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for ScW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sc.w")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmoswapW(pub u32);
impl AmoswapW {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmoswapW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amoswap.w"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmoswapW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amoswap.w")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmoaddW(pub u32);
impl AmoaddW {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmoaddW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amoadd.w"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmoaddW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amoadd.w")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmoxorW(pub u32);
impl AmoxorW {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmoxorW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amoxor.w"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmoxorW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amoxor.w")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmoorW(pub u32);
impl AmoorW {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmoorW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amoor.w"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmoorW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amoor.w")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmoandW(pub u32);
impl AmoandW {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmoandW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amoand.w"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmoandW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amoand.w")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmominW(pub u32);
impl AmominW {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmominW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amomin.w"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmominW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amomin.w")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmomaxW(pub u32);
impl AmomaxW {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmomaxW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amomax.w"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmomaxW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amomax.w")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmominuW(pub u32);
impl AmominuW {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmominuW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amominu.w"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmominuW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amominu.w")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmomaxuW(pub u32);
impl AmomaxuW {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmomaxuW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amomaxu.w"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmomaxuW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amomaxu.w")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct LrD(pub u32);
impl LrD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for LrD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("lr.d"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for LrD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lr.d")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct ScD(pub u32);
impl ScD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for ScD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("sc.d"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for ScD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sc.d")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmoswapD(pub u32);
impl AmoswapD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmoswapD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amoswap.d"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmoswapD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amoswap.d")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmoaddD(pub u32);
impl AmoaddD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmoaddD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amoadd.d"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmoaddD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amoadd.d")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmoxorD(pub u32);
impl AmoxorD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmoxorD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amoxor.d"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmoxorD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amoxor.d")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmoorD(pub u32);
impl AmoorD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmoorD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amoor.d"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmoorD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amoor.d")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmoandD(pub u32);
impl AmoandD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmoandD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amoand.d"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmoandD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amoand.d")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmominD(pub u32);
impl AmominD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmominD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amomin.d"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmominD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amomin.d")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmomaxD(pub u32);
impl AmomaxD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmomaxD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amomax.d"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmomaxD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amomax.d")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmominuD(pub u32);
impl AmominuD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmominuD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amominu.d"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmominuD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amominu.d")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
pub struct AmomaxuD(pub u32);
impl AmomaxuD {
    #[inline]
    pub const fn rd(&self) -> Reg {
        {
            let acc = (self.0 >> 7) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
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
    pub const fn rs2(&self) -> Reg {
        {
            let acc = (self.0 >> 20) & 0b11111;
            unsafe { Reg::from_u5(acc as u8) }
        }
    }
    #[inline]
    pub const fn aq(&self) -> u32 {
        (self.0 >> 26) & 0b1
    }
    #[inline]
    pub const fn rl(&self) -> u32 {
        (self.0 >> 25) & 0b1
    }
}
impl std::fmt::Debug for AmomaxuD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("amomaxu.d"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .field(stringify!(aq), &self.aq())
            .field(stringify!(rl), &self.rl())
            .finish()
    }
}
impl std::fmt::Display for AmomaxuD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "amomaxu.d")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        write!(f, " {:?}", self.aq())?;
        write!(f, " {:?}", self.rl())?;
        Ok(())
    }
}
impl std::fmt::Debug for Rv64a {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rv64a::LrW(inst) => write!(f, "{inst:?}"),
            Rv64a::ScW(inst) => write!(f, "{inst:?}"),
            Rv64a::AmoswapW(inst) => write!(f, "{inst:?}"),
            Rv64a::AmoaddW(inst) => write!(f, "{inst:?}"),
            Rv64a::AmoxorW(inst) => write!(f, "{inst:?}"),
            Rv64a::AmoorW(inst) => write!(f, "{inst:?}"),
            Rv64a::AmoandW(inst) => write!(f, "{inst:?}"),
            Rv64a::AmominW(inst) => write!(f, "{inst:?}"),
            Rv64a::AmomaxW(inst) => write!(f, "{inst:?}"),
            Rv64a::AmominuW(inst) => write!(f, "{inst:?}"),
            Rv64a::AmomaxuW(inst) => write!(f, "{inst:?}"),
            Rv64a::LrD(inst) => write!(f, "{inst:?}"),
            Rv64a::ScD(inst) => write!(f, "{inst:?}"),
            Rv64a::AmoswapD(inst) => write!(f, "{inst:?}"),
            Rv64a::AmoaddD(inst) => write!(f, "{inst:?}"),
            Rv64a::AmoxorD(inst) => write!(f, "{inst:?}"),
            Rv64a::AmoorD(inst) => write!(f, "{inst:?}"),
            Rv64a::AmoandD(inst) => write!(f, "{inst:?}"),
            Rv64a::AmominD(inst) => write!(f, "{inst:?}"),
            Rv64a::AmomaxD(inst) => write!(f, "{inst:?}"),
            Rv64a::AmominuD(inst) => write!(f, "{inst:?}"),
            Rv64a::AmomaxuD(inst) => write!(f, "{inst:?}"),
        }
    }
}
impl std::fmt::Display for Rv64a {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rv64a::LrW(inst) => write!(f, "{inst}"),
            Rv64a::ScW(inst) => write!(f, "{inst}"),
            Rv64a::AmoswapW(inst) => write!(f, "{inst}"),
            Rv64a::AmoaddW(inst) => write!(f, "{inst}"),
            Rv64a::AmoxorW(inst) => write!(f, "{inst}"),
            Rv64a::AmoorW(inst) => write!(f, "{inst}"),
            Rv64a::AmoandW(inst) => write!(f, "{inst}"),
            Rv64a::AmominW(inst) => write!(f, "{inst}"),
            Rv64a::AmomaxW(inst) => write!(f, "{inst}"),
            Rv64a::AmominuW(inst) => write!(f, "{inst}"),
            Rv64a::AmomaxuW(inst) => write!(f, "{inst}"),
            Rv64a::LrD(inst) => write!(f, "{inst}"),
            Rv64a::ScD(inst) => write!(f, "{inst}"),
            Rv64a::AmoswapD(inst) => write!(f, "{inst}"),
            Rv64a::AmoaddD(inst) => write!(f, "{inst}"),
            Rv64a::AmoxorD(inst) => write!(f, "{inst}"),
            Rv64a::AmoorD(inst) => write!(f, "{inst}"),
            Rv64a::AmoandD(inst) => write!(f, "{inst}"),
            Rv64a::AmominD(inst) => write!(f, "{inst}"),
            Rv64a::AmomaxD(inst) => write!(f, "{inst}"),
            Rv64a::AmominuD(inst) => write!(f, "{inst}"),
            Rv64a::AmomaxuD(inst) => write!(f, "{inst}"),
        }
    }
}
