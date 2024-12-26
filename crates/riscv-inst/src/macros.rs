/// Generates const functions to extract bits from a 32-bit integer according
/// to a specification of the form `[(sigext)? ((end:start | pos) \|)+]`.
///
/// After compilation, the generated functions are well-optimized, often consisting
/// of a single instruction (for simple cases) and no more than a few instructions (for complex cases).
///
/// ## Examples
/// For example, the following:
/// ```ignore
/// // Generate `rd()` which extracts bits 7-11 from the source and
/// // outputs them as bits 0-4 in the result.
/// bits!(rd: [11:7])
///
/// // Generate `imm()` which extracts bits 20-31 from the source and
/// // outputs them as bits 0-11 in the result, sign-extending the result.
/// bits!(imm: [sigext 31:20])
///
/// // An example of the a complex case (RISC-V J-type immediate):
/// // Generate `imm()` which extracts bits 31, 19-12, 20, and 30-21 from the source and
/// // outputs them as bits 0-19 in the result, sign-extending the result.
/// // The resultant function is less than 10 asm instructions.
/// bits!(imm: [sigext 31 | 19:12 | 20 | 30:21])
/// ```
macro_rules! bits {
    // Sign-extended extraction
    ($name:ident: [sigext $($spec:tt)+]) => {
        #[inline(never)]
        #[must_use = "this returns the result of the operation, without modifying the original"]
        pub const fn $name(&self) -> i32 {
            let input = self.0;
            let mut result: u32 = 0;
            let total_bits = bits!(@count 0, $($spec)+);
            bits!(@process input, result, total_bits, 0, $($spec)+);

            // sign extend
            ((((result << (u32::BITS as u8 - total_bits)) as i32) >> (u32::BITS as u8 - total_bits)) as i32)
        }
    };

    // Normal extraction
    ($name:ident: [$($spec:tt)+]) => {
        #[inline(never)]
        #[must_use = "this returns the result of the operation, without modifying the original"]
        pub const fn $name(&self) -> u32 {
            let input = self.0;
            let mut result: u32 = 0;
            let total_bits = bits!(@count 0, $($spec)+);
            bits!(@process input, result, total_bits, 0, $($spec)+);
            result
        }
    };

    // Count total bits (for sign extension)
    (@count $acc:expr, $end:literal : $start:literal) => {
        $acc + $end - $start + 1
    };
    (@count $acc:expr, <0 repeat $zeros:literal>) => {
        $acc + $zeros
    };
    (@count $acc:expr, $end:literal : $start:literal | $($rest:tt)+) => {
        bits!(@count $acc + $end - $start + 1, $($rest)+)
    };

    // Zero bits
    (@process $input:ident, $result:ident, $total_bits: ident, $idx:expr, <0 repeat $zeros:literal>) => {};

    // Range Terminal
    (@process $input:ident, $result:ident, $total_bits: ident, $idx:expr, $end:literal : $start:literal) => {
        $result |= crate::bits32($input, $start..($end + 1), $total_bits - $idx - ($end - $start + 1));
    };


    // Range | ...
    (@process $input:ident, $result:ident, $total_bits: ident, $idx:expr, $end:literal : $start:literal | $($rest:tt)+) => {
        $result |= crate::bits32($input, $start..($end + 1), ($total_bits - $idx) - ($end - $start + 1));
        bits!(@process $input, $result, $total_bits, $idx + ($end - $start + 1), $($rest)+)
    };
}

macro_rules! mask {
    ($start: literal, $end: literal) => {
        (((1 << ($end - $start + 1)) - 1) << $start)
    };
    ($start:literal, $end: literal, $($rest:tt)+) => {
        mask!($start, $end) | mask!($($rest)+)
    };
}

macro_rules! desired {
    ($value:literal, $start: literal) => {
        ($value << $start)
    };
    ($value:literal, $start: literal, $($rest:tt)+) => {
        ($value << $start) | desired!($($rest)+)
    };
}

macro_rules! instruction {
    (
        $mod_name:ident::$type:ident {
            $( $name:ident : $bits:tt, )*
        },
        mask: $([$mask_end:literal:$mask_start:literal] )|+,
        opcodes {
            // Match e.g. [6:0] == 0x03 => Opcodes::ALU
            // or more than one constraint [6:0] == 0b0110011 && [14:12] == 0x7 => Opcodes::AND
            $( $([$end:literal : $start:literal] == $opcode:literal)&&+ $(if [$if_end:literal : $if_start:literal] == $if_code:literal)? => $opname:ident, )*
        }
    ) => {
        #[allow(non_snake_case)]
        mod $mod_name {
            #[derive(PartialEq, Eq)]
            pub struct $type(pub u32);

            impl $type {
                $(
                    bits!($name: $bits);
                )*
            }

            #[derive(Debug, PartialEq, Eq)]
            pub enum Opcode {
                $(
                    $opname,
                )*
            }

            const MASK: u32 = mask!( $($mask_start, $mask_end),+ );
            mod desired {
                $(
                    pub const $opname: u32 = $(desired!($opcode, $start)) | +;
                )*
            }

            impl Opcode {
                pub const fn decode(inst: u32) -> Option<Self> {
                    match inst & MASK {
                        $(
                            desired::$opname $(if ((inst & mask!($if_start, $if_end)) == desired!($if_code, $if_start)))? => Some(Opcode::$opname),
                        )*
                        _ => None,
                    }
                }
            }

            impl std::fmt::Debug for $type {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!($type))
                        $(
                            .field(stringify!($name), &self.$name())
                        )*
                        .finish()
                }
            }
        }
    };
}
