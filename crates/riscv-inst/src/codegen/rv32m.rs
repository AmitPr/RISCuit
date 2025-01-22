//! Autogenerated by riscv-inst-codegen
//! DO NOT EDIT

#[allow(unused_imports)]
use super::{FReg, Reg};
pub enum Rv32m {
    Mul(Mul),
    Mulh(Mulh),
    Mulhsu(Mulhsu),
    Mulhu(Mulhu),
    Div(Div),
    Divu(Divu),
    Rem(Rem),
    Remu(Remu),
}
pub struct Mul(pub u32);
impl Mul {
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
}
impl std::fmt::Debug for Mul {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("mul"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .finish()
    }
}
impl std::fmt::Display for Mul {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mul")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        Ok(())
    }
}
pub struct Mulh(pub u32);
impl Mulh {
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
}
impl std::fmt::Debug for Mulh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("mulh"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .finish()
    }
}
impl std::fmt::Display for Mulh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mulh")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        Ok(())
    }
}
pub struct Mulhsu(pub u32);
impl Mulhsu {
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
}
impl std::fmt::Debug for Mulhsu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("mulhsu"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .finish()
    }
}
impl std::fmt::Display for Mulhsu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mulhsu")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        Ok(())
    }
}
pub struct Mulhu(pub u32);
impl Mulhu {
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
}
impl std::fmt::Debug for Mulhu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("mulhu"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .finish()
    }
}
impl std::fmt::Display for Mulhu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mulhu")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        Ok(())
    }
}
pub struct Div(pub u32);
impl Div {
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
}
impl std::fmt::Debug for Div {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("div"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .finish()
    }
}
impl std::fmt::Display for Div {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "div")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        Ok(())
    }
}
pub struct Divu(pub u32);
impl Divu {
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
}
impl std::fmt::Debug for Divu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("divu"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .finish()
    }
}
impl std::fmt::Display for Divu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "divu")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        Ok(())
    }
}
pub struct Rem(pub u32);
impl Rem {
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
}
impl std::fmt::Debug for Rem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("rem"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .finish()
    }
}
impl std::fmt::Display for Rem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rem")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        Ok(())
    }
}
pub struct Remu(pub u32);
impl Remu {
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
}
impl std::fmt::Debug for Remu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!("remu"))
            .field("inst", &self.0)
            .field(stringify!(rd), &self.rd())
            .field(stringify!(rs1), &self.rs1())
            .field(stringify!(rs2), &self.rs2())
            .finish()
    }
}
impl std::fmt::Display for Remu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "remu")?;
        write!(f, " {:?}", self.rd())?;
        write!(f, " {:?}", self.rs1())?;
        write!(f, " {:?}", self.rs2())?;
        Ok(())
    }
}
impl std::fmt::Debug for Rv32m {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rv32m::Mul(inst) => write!(f, "{inst:?}"),
            Rv32m::Mulh(inst) => write!(f, "{inst:?}"),
            Rv32m::Mulhsu(inst) => write!(f, "{inst:?}"),
            Rv32m::Mulhu(inst) => write!(f, "{inst:?}"),
            Rv32m::Div(inst) => write!(f, "{inst:?}"),
            Rv32m::Divu(inst) => write!(f, "{inst:?}"),
            Rv32m::Rem(inst) => write!(f, "{inst:?}"),
            Rv32m::Remu(inst) => write!(f, "{inst:?}"),
        }
    }
}
impl std::fmt::Display for Rv32m {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rv32m::Mul(inst) => write!(f, "{inst}"),
            Rv32m::Mulh(inst) => write!(f, "{inst}"),
            Rv32m::Mulhsu(inst) => write!(f, "{inst}"),
            Rv32m::Mulhu(inst) => write!(f, "{inst}"),
            Rv32m::Div(inst) => write!(f, "{inst}"),
            Rv32m::Divu(inst) => write!(f, "{inst}"),
            Rv32m::Rem(inst) => write!(f, "{inst}"),
            Rv32m::Remu(inst) => write!(f, "{inst}"),
        }
    }
}
