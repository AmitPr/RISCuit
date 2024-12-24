macro_rules! impl_from_bits {
    (pub enum $enum_name:ident { $($variant:ident = $value:expr),* $(,)? }) => {
        #[derive(Debug, PartialEq, Eq)]
        pub enum $enum_name {
            $( $variant = $value ),*
        }

        impl $enum_name {
            pub const fn from(value: u8) -> Option<Self> {
                match value {
                    $( $value => Some($enum_name::$variant), )*
                    _ => None,
                }
            }
        }
    };
}

impl_from_bits! {
    pub enum Opcode {
        Load = 0b000_0011,
        Store = 0b010_0011,
        IntReg = 0b011_0011,
        IntImm = 0b001_0011,
        Lui = 0b011_0111,
        Auipc = 0b001_0111,
        Jal = 0b110_1111,
        Jalr = 0b110_0111,
        Branch = 0b110_0011,
        Fence = 0b000_1111,
        Ecall = 0b111_0011,
    }
}

impl_from_bits!(
    pub enum LoadStoreFunc3 {
        IByte = 0b000,
        IHalf = 0b001,
        IWord = 0b010,
        UByte = 0b100,
        UHalf = 0b101,
        UWord = 0b110,
    }
);

impl_from_bits!(
    pub enum IntRegFunc3 {
        AddSub = 0b000,
        Sll = 0b001,
        Slt = 0b010,
        Sltu = 0b011,
        Xor = 0b100,
        SrlSra = 0b101,
        Or = 0b110,
        And = 0b111,
    }
);

impl_from_bits!(
    pub enum IntImmFunc3 {
        Addi = 0b000,
        Slli = 0b001,
        Slti = 0b010,
        Sltiu = 0b011,
        Xori = 0b100,
        SrliSrai = 0b101,
        Ori = 0b110,
        Andi = 0b111,
    }
);

impl_from_bits!(
    pub enum BranchFunc3 {
        Beq = 0b000,
        Bne = 0b001,
        Blt = 0b100,
        Bge = 0b101,
        Bltu = 0b110,
        Bgeu = 0b111,
    }
);

pub struct Instruction(pub u32);

impl Instruction {
    pub const fn new(inst: u32) -> Self {
        Self(inst)
    }

    pub const fn opcode(&self) -> Option<Opcode> {
        let opcode = self.0 & 0b1111111;
        Opcode::from(opcode as u8)
    }

    pub const fn rd(&self) -> u32 {
        (self.0 >> 7) & 0b1_1111
    }

    pub const fn funct3(&self) -> u8 {
        ((self.0 >> 12) & 0b111) as u8
    }

    pub const fn rs1(&self) -> u32 {
        (self.0 >> 15) & 0b1_1111
    }

    pub const fn rs2(&self) -> u32 {
        (self.0 >> 20) & 0b1_1111
    }

    pub const fn funct7(&self) -> u8 {
        ((self.0 >> 25) & 0b111_1111) as u8
    }

    pub const fn signed_imm(&self) -> i32 {
        // cast to i32 to sign-extend
        self.0 as i32 >> 20
    }

    pub const fn unsigned_imm(&self) -> u32 {
        self.0 >> 20
    }

    pub const fn imm20(&self) -> u32 {
        self.0 >> 12
    }
}
