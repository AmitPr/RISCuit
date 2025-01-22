use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Reg {
    Zero = 0,
    Ra = 1,
    Sp = 2,
    Gp = 3,
    Tp = 4,
    T0 = 5,
    T1 = 6,
    T2 = 7,
    S0 = 8,
    S1 = 9,
    A0 = 10,
    A1 = 11,
    A2 = 12,
    A3 = 13,
    A4 = 14,
    A5 = 15,
    A6 = 16,
    A7 = 17,
    S2 = 18,
    S3 = 19,
    S4 = 20,
    S5 = 21,
    S6 = 22,
    S7 = 23,
    S8 = 24,
    S9 = 25,
    S10 = 26,
    S11 = 27,
    T3 = 28,
    T4 = 29,
    T5 = 30,
    T6 = 31,
}

impl Reg {
    /// # Safety
    /// Assumes that reg is < 32
    pub const unsafe fn from_u5(reg: u8) -> Self {
        // assumes that reg is < 32
        std::mem::transmute::<u8, Self>(reg)
    }

    pub const fn checked_from(reg: u8) -> Option<Self> {
        if reg < 32 {
            Some(unsafe { Self::from_u5(reg) })
        } else {
            None
        }
    }
}

impl Debug for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(f, "zero"),
            Self::Ra => write!(f, "ra"),
            Self::Sp => write!(f, "sp"),
            Self::Gp => write!(f, "gp"),
            Self::Tp => write!(f, "tp"),
            Self::T0 => write!(f, "t0"),
            Self::T1 => write!(f, "t1"),
            Self::T2 => write!(f, "t2"),
            Self::S0 => write!(f, "s0"),
            Self::S1 => write!(f, "s1"),
            Self::A0 => write!(f, "a0"),
            Self::A1 => write!(f, "a1"),
            Self::A2 => write!(f, "a2"),
            Self::A3 => write!(f, "a3"),
            Self::A4 => write!(f, "a4"),
            Self::A5 => write!(f, "a5"),
            Self::A6 => write!(f, "a6"),
            Self::A7 => write!(f, "a7"),
            Self::S2 => write!(f, "s2"),
            Self::S3 => write!(f, "s3"),
            Self::S4 => write!(f, "s4"),
            Self::S5 => write!(f, "s5"),
            Self::S6 => write!(f, "s6"),
            Self::S7 => write!(f, "s7"),
            Self::S8 => write!(f, "s8"),
            Self::S9 => write!(f, "s9"),
            Self::S10 => write!(f, "s10"),
            Self::S11 => write!(f, "s11"),
            Self::T3 => write!(f, "t3"),
            Self::T4 => write!(f, "t4"),
            Self::T5 => write!(f, "t5"),
            Self::T6 => write!(f, "t6"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FReg {
    FT0 = 0,
    FT1 = 1,
    FT2 = 2,
    FT3 = 3,
    FT4 = 4,
    FT5 = 5,
    FT6 = 6,
    FT7 = 7,
    FS0 = 8,
    FS1 = 9,
    FA0 = 10,
    FA1 = 11,
    FA2 = 12,
    FA3 = 13,
    FA4 = 14,
    FA5 = 15,
    FA6 = 16,
    FA7 = 17,
    FS2 = 18,
    FS3 = 19,
    FS4 = 20,
    FS5 = 21,
    FS6 = 22,
    FS7 = 23,
    FS8 = 24,
    FS9 = 25,
    FS10 = 26,
    FS11 = 27,
    FT8 = 28,
    FT9 = 29,
    FT10 = 30,
    FT11 = 31,
}

impl FReg {
    /// # Safety
    /// Assumes that reg is < 32
    pub const unsafe fn from_u5(reg: u8) -> Self {
        // assumes that reg is < 32
        std::mem::transmute::<u8, Self>(reg)
    }

    pub const fn checked_from(reg: u8) -> Option<Self> {
        if reg < 32 {
            Some(unsafe { Self::from_u5(reg) })
        } else {
            None
        }
    }
}

impl Debug for FReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FT0 => write!(f, "ft0"),
            Self::FT1 => write!(f, "ft1"),
            Self::FT2 => write!(f, "ft2"),
            Self::FT3 => write!(f, "ft3"),
            Self::FT4 => write!(f, "ft4"),
            Self::FT5 => write!(f, "ft5"),
            Self::FT6 => write!(f, "ft6"),
            Self::FT7 => write!(f, "ft7"),
            Self::FS0 => write!(f, "fs0"),
            Self::FS1 => write!(f, "fs1"),
            Self::FA0 => write!(f, "fa0"),
            Self::FA1 => write!(f, "fa1"),
            Self::FA2 => write!(f, "fa2"),
            Self::FA3 => write!(f, "fa3"),
            Self::FA4 => write!(f, "fa4"),
            Self::FA5 => write!(f, "fa5"),
            Self::FA6 => write!(f, "fa6"),
            Self::FA7 => write!(f, "fa7"),
            Self::FS2 => write!(f, "fs2"),
            Self::FS3 => write!(f, "fs3"),
            Self::FS4 => write!(f, "fs4"),
            Self::FS5 => write!(f, "fs5"),
            Self::FS6 => write!(f, "fs6"),
            Self::FS7 => write!(f, "fs7"),
            Self::FS8 => write!(f, "fs8"),
            Self::FS9 => write!(f, "fs9"),
            Self::FS10 => write!(f, "fs10"),
            Self::FS11 => write!(f, "fs11"),
            Self::FT8 => write!(f, "ft8"),
            Self::FT9 => write!(f, "ft9"),
            Self::FT10 => write!(f, "ft10"),
            Self::FT11 => write!(f, "ft11"),
        }
    }
}
