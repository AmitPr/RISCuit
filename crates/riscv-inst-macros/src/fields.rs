use proc_macro2::TokenStream;
use quote::quote;

use crate::Field;

pub fn get_operand_type(name: &str) -> TokenStream {
    match name {
        // Immediates that are signed
        "imm20" | "oimm20" | "jimm20" | "imm12" | "oimm12" | "csr12" | "simm12" | "sbimm12"
        | "cimmi" | "cnzimmi" | "cimmui" | "cimm16sp" | "cimmj" | "cimmb" => {
            quote! { i32 }
        }

        // Flags
        "aq" | "rl" => quote! { bool },

        // Compressed
        "crd0" => quote! { bool },

        // Everything else defaults to u32
        _ => quote! { u32 },
    }
}

pub fn operand_accessor(name: &str) -> TokenStream {
    match name {
        // Standard registers
        "rd" | "frd" => quote! { ::riscv_inst_macros::bits!(inst[11:7]) },
        "rs1" | "frs1" => quote! { ::riscv_inst_macros::bits!(inst[19:15]) },
        "rs2" | "frs2" => quote! { ::riscv_inst_macros::bits!(inst[24:20]) },
        "rs3" | "frs3" => quote! { ::riscv_inst_macros::bits!(inst[31:27]) },

        // Flags
        "aq" => quote! { ::riscv_inst_macros::bits!(inst[26]) != 0 },
        "rl" => quote! { ::riscv_inst_macros::bits!(inst[25]) != 0 },
        "pred" => quote! { ::riscv_inst_macros::bits!(inst[27:24]) },
        "succ" => quote! { ::riscv_inst_macros::bits!(inst[23:20]) },
        "rm" => quote! { ::riscv_inst_macros::bits!(inst[14:12]) },

        // Immediates
        "imm20" | "oimm20" => quote! { ::riscv_inst_macros::bits!(sign, inst[31:12 | +0*12]) },
        "jimm20" => quote! { ::riscv_inst_macros::bits!(sign, inst[31 | 19:12 | 20 | 30:21 | +0]) },
        "imm12" | "oimm12" | "csr12" => quote! { ::riscv_inst_macros::bits!(sign, inst[31:20]) },
        "simm12" => quote! { ::riscv_inst_macros::bits!(sign, inst[31:25 | 11:7]) },
        "sbimm12" => quote! { ::riscv_inst_macros::bits!(sign, inst[31 | 7 | 30:25 | 11:8 | +0]) },
        "zimm" => quote! { ::riscv_inst_macros::bits!(inst[19:15]) },

        // shamt
        "shamt5" => quote! { ::riscv_inst_macros::bits!(inst[24:20]) },
        "shamt6" => quote! { ::riscv_inst_macros::bits!(inst[25:20]) },
        "shamt7" => quote! { ::riscv_inst_macros::bits!(inst[26:20]) },

        // Compressed
        "crd0" => quote! { ::riscv_inst_macros::bits!(inst[12]) != 0 },
        "crdq" | "crs2q" => quote! { ::riscv_inst_macros::bits!(inst[4:2]) },
        "crs1q" | "crs1rdq" => quote! { ::riscv_inst_macros::bits!(inst[9:7]) },
        "crd" | "crs1" | "crs1rd" => quote! { ::riscv_inst_macros::bits!(inst[11:7]) },
        "crs2" => quote! { ::riscv_inst_macros::bits!(inst[6:2]) },
        "cfrdq" | "cfrs2q" => quote! { ::riscv_inst_macros::bits!(inst[4:2]) },
        "cfrs2" => quote! { ::riscv_inst_macros::bits!(inst[6:2]) },
        "cfrd" => quote! { ::riscv_inst_macros::bits!(inst[11:7]) },

        // Compressed immediates
        "cimmsh5" => quote! { ::riscv_inst_macros::bits!(inst[6:2]) },
        "cimmsh6" => quote! { ::riscv_inst_macros::bits!(inst[12 | 6:2]) },
        "cimmi" | "cnzimmi" => quote! { ::riscv_inst_macros::bits!(sign, inst[12 | 6:2]) },
        "cimmui" => quote! { ::riscv_inst_macros::bits!(sign, inst[12 | 6:2 | +0*12]) },
        "cimmlwsp" => quote! { ::riscv_inst_macros::bits!(inst[3:2 | 12 | 6:4 | +0*2]) },
        "cimmldsp" => quote! { ::riscv_inst_macros::bits!(inst[3:2 | 12 | 6:4 | +0*3]) },
        "cimmlqsp" => quote! { ::riscv_inst_macros::bits!(inst[3:2 | 12 | 6:4 | +0*4]) },
        "cimm16sp" => {
            quote! { ::riscv_inst_macros::bits!(sign, inst[12 | 4:3 | 5 | 2 | 6 | +0*4]) }
        }
        "cimmj" => {
            quote! { ::riscv_inst_macros::bits!(sign, inst[12 | 8 | 10:9 | 6 | 7 | 5:3 | 2 | +0 ]) }
        }
        "cimmb" => {
            quote! { ::riscv_inst_macros::bits!(sign, inst[12 | 6:5 | 2 | 11:10 | 4:3 | +0 ]) }
        }
        "cimmswsp" => quote! { ::riscv_inst_macros::bits!(inst[8:7 | 12:9 | +0*2]) },
        "cimmsdsp" => quote! { ::riscv_inst_macros::bits!(inst[8:7 | 12:9 | +0*3]) },
        "cimmsqsp" => quote! { ::riscv_inst_macros::bits!(inst[8:7 | 12:9 | +0*4]) },
        "cimm4spn" => quote! { ::riscv_inst_macros::bits!(inst[10:7 | 12:11 | 5 | 6 | +0*2]) },
        "cimmw" => quote! { ::riscv_inst_macros::bits!(inst[5 | 12:10 | 6 | +0*2]) },
        "cimmd" => quote! { ::riscv_inst_macros::bits!(inst[6:5 | 12:10 | +0*3]) },
        "cimmq" => quote! { ::riscv_inst_macros::bits!(inst[10 | 6:5 | 12:11 | +0*4]) },

        // Unhandled cases return a compiler error
        _ => quote! {
            compile_error!(concat!("Unknown operand type: ", #name))
        },
    }
}

pub fn operand_accessor_fn(operand: Field) -> TokenStream {
    let operand_name = operand.field.to_string();
    let operand_type = get_operand_type(&operand_name);
    let operand_accessor = operand_accessor(&operand_name);
    let operand_fn_name = operand.alias.unwrap_or(operand.field);
    quote! {
        #[inline]
        pub const fn #operand_fn_name(&self) -> #operand_type {
            let inst = self.0 as u32;
            #operand_accessor
        }
    }
}
