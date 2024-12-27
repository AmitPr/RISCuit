macro_rules! width_ty {
    (i, 32) => {
        i32
    };
    (i, 16) => {
        i16
    };
    (u, 32) => {
        u32
    };
    (u, 16) => {
        u16
    };
    (bits, 16) => {
        crate::bits16
    };
    (bits, 32) => {
        crate::bits32
    };
}

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
    ($width: tt, $name:ident: [sigext $($spec:tt)+]) => {
        #[inline(never)]
        #[must_use = "this returns the result of the operation, without modifying the original"]
        pub const fn $name(&self) -> crate::width_ty!(i, $width) {
            let input = self.0;
            let mut result: crate::width_ty!(u, $width) = 0;
            let total_bits = crate::bits!(@count 0, $($spec)+);
            crate::bits!(@process $width, input, result, total_bits, 0, $($spec)+);

            // sign extend
            ((((result << (<crate::width_ty!(u, $width)>::BITS as u8 - total_bits)) as crate::width_ty!(i, $width)) >> (<crate::width_ty!(u, $width)>::BITS as u8 - total_bits)) as crate::width_ty!(i, $width))
        }
    };

    // Normal extraction
    ($width:tt, $name:ident: [$($spec:tt)+]) => {
        #[inline(never)]
        #[must_use = "this returns the result of the operation, without modifying the original"]
        pub const fn $name(&self) -> crate::width_ty!(u, $width) {
            let input = self.0;
            let mut result: crate::width_ty!(u, $width) = 0;
            let total_bits = crate::bits!(@count 0, $($spec)+);
            crate::bits!(@process $width, input, result, total_bits, 0, $($spec)+);
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
        crate::bits!(@count $acc + $end - $start + 1, $($rest)+)
    };

    // Zero bits
    (@process $width:tt, $input:ident, $result:ident, $total_bits: ident, $idx:expr, <0 repeat $zeros:literal>) => {};

    // Range Terminal
    (@process $width:tt, $input:ident, $result:ident, $total_bits: ident, $idx:expr, $end:literal : $start:literal) => {
        $result |= (crate::width_ty!(bits, $width))($input, $start..($end + 1), $total_bits - $idx - ($end - $start + 1));
    };


    // Range | ...
    (@process $width:tt, $input:ident, $result:ident, $total_bits: ident, $idx:expr, $end:literal : $start:literal | $($rest:tt)+) => {
        $result |= crate::bits32($input, $start..($end + 1), ($total_bits - $idx) - ($end - $start + 1));
        crate::bits!(@process $width, $input, $result, $total_bits, $idx + ($end - $start + 1), $($rest)+)
    };
}

macro_rules! mask {
    ($start: literal, $end: literal) => {
        (((1 << ($end - $start + 1)) - 1) << $start)
    };
    ($start:literal, $end: literal, $($rest:tt)+) => {
        crate::mask!($start, $end) | crate::mask!($($rest)+)
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
        $width: tt,
        $mod_name:ident::$type:ident {
            $( $name:ident : $bits:tt, )*
        },
        mask: $([$mask_end:literal:$mask_start:literal] )|+,
        opcodes {
            // Match e.g. [6:0] == 0x03 => Opcodes::ALU
            // or more than one constraint [6:0] == 0b0110011 && [14:12] == 0x7 => Opcodes::AND
            $(
                // Optional cfg feature flag
                $(#[cfg($cfg:meta)])?
                // check fields within the mask are as expected
                $([$end:literal : $start:literal] == $opcode:literal)&&+
                // Optional constraint that isn't accounted for by the mask
                $(if $([$if_end:literal : $if_start:literal] $check: tt $if_code:literal)&&+)? => $opname:ident,
            )*
        }
    ) => {
        #[allow(non_snake_case)]
        mod $mod_name {
            #[derive(PartialEq, Eq)]
            pub struct $type(pub crate::width_ty!(u, $width));

            impl $type {
                $(
                    crate::bits!($width, $name: $bits);
                )*
            }

            #[derive(Debug, PartialEq, Eq)]
            pub enum Opcode {
                $(
                    $(#[cfg($cfg)])?
                    $opname,
                )*
            }

            const MASK: crate::width_ty!(u, $width) = crate::mask!( $($mask_start, $mask_end),+ );
            mod _mask_match {
                $(
                    $(#[cfg($cfg)])?
                    pub const $opname: crate::width_ty!(u, $width) = $(crate::desired!($opcode, $start)) | +;
                )*
            }

            impl Opcode {
                pub const fn decode(inst: crate::width_ty!(u, $width)) -> Option<Self> {
                    match inst & MASK {
                        $(
                            $(#[cfg($cfg)])?
                            _mask_match::$opname $(
                                if $(((inst & crate::mask!($if_start, $if_end)) $check crate::desired!($if_code, $if_start)))&&+
                            )? => Some(Opcode::$opname),
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

pub(crate) use {bits, desired, instruction, mask, width_ty};
