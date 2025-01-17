// RISC-V C-to-G table generator
// Auto-generated Rust port of original by Artem Litvinovich (https://github.com/artlav/riscv-c2g-table)

#![allow(dead_code)]

// Instruction opcodes
const CRV_IOP_JALR: u32 = 0x67;
const CRV_IOP_JAL: u32 = 0x6F;
const CRV_IOP_IMM: u32 = 0x13;
const CRV_IOP_LUI: u32 = 0x37;
const CRV_IOP_STORE: u32 = 0x23;
const CRV_IOP_LOAD: u32 = 0x03;
const CRV_IOP_BRANCH: u32 = 0x63;
const CRV_IOP_OP: u32 = 0x33;
const CRV_IOP_OP32: u32 = 0x3B;
const CRV_IOP_IMM32: u32 = 0x1B;

// Function 3 codes
// CRV_IOP_IMM
const CRV_F3_ADDI: u32 = 0;
const CRV_F3_SLLI: u32 = 1;
const CRV_F3_SRLSRAI: u32 = 5;
const CRV_F3_ANDI: u32 = 7;
// CRV_IOP_IMM32
const CRV_F3_ADDIW: u32 = 0;
// CRV_IOP_STORE
const CRV_F3_SW: u32 = 2;
const CRV_F3_SD: u32 = 3;
// CRV_IOP_BRANCH
const CRV_F3_BEQ: u32 = 0;
const CRV_F3_BNE: u32 = 1;
// CRV_IOP_LOAD
const CRV_F3_LW: u32 = 2;
const CRV_F3_LD: u32 = 3;
// CRV_IOP_OP
const CRV_F3_ADDSUB: u32 = 0;
const CRV_F3_XOR: u32 = 4;
const CRV_F3_OR: u32 = 6;
const CRV_F3_AND: u32 = 7;
// CRV_IOP_OP32
const CRV_F3_ADDSUBW: u32 = 0;

// Compressed instruction opcodes
const OP_C0: u16 = 0;
const OP_C1: u16 = 1;
const OP_C2: u16 = 2;

// Compressed instruction major opcodes
const OPC_M0: u16 = 0;
const OPC_M1: u16 = 1;
const OPC_M2: u16 = 2;
const OPC_M3: u16 = 3;
const OPC_M4: u16 = 4;
const OPC_M5: u16 = 5;
const OPC_M6: u16 = 6;
const OPC_M7: u16 = 7;

// Sign extension functions
const fn sign_extend_6(w: u32) -> u32 {
    if w & 0x0020 != 0 {
        0xFFFFFFC0 | w
    } else {
        w
    }
}

const fn sign_extend_9(w: u32) -> u32 {
    if w & 0x0100 != 0 {
        0xFFFFFE00 | w
    } else {
        w
    }
}

const fn sign_extend_10(w: u32) -> u32 {
    if w & 0x0200 != 0 {
        0xFFFFFC00 | w
    } else {
        w
    }
}

const fn sign_extend_12(w: u32) -> u32 {
    if w & 0x0800 != 0 {
        0xFFFFF000 | w
    } else {
        w
    }
}

// Instruction composition functions
const fn crv_compose_j(op: u32, rd: u32, imm: u32) -> u32 {
    (op & 0x7F)
        | ((rd << 7) & 0x00000F80)
        | ((imm << 11) & 0x80000000)
        | (imm & 0x000FF000)
        | ((imm << 9) & 0x00100000)
        | ((imm << 20) & 0x7FE00000)
}

const fn crv_compose_i(op: u32, rs1: u32, rd: u32, funct3: u32, imm: u32) -> u32 {
    (op & 0x7F)
        | ((rd << 7) & 0x00000F80)
        | ((funct3 << 12) & 0x00007000)
        | ((rs1 << 15) & 0x000F8000)
        | ((imm << 20) & 0xFFF00000)
}

const fn crv_compose_r(op: u32, rs1: u32, rs2: u32, rd: u32, funct3: u32, funct7: u32) -> u32 {
    (op & 0x7F)
        | ((rd << 7) & 0x00000F80)
        | ((funct3 << 12) & 0x00007000)
        | ((rs1 << 15) & 0x000F8000)
        | ((rs2 << 20) & 0x01F00000)
        | ((funct7 << 25) & 0xFE000000)
}

const fn crv_compose_s(op: u32, rs1: u32, rs2: u32, funct3: u32, imm: u32) -> u32 {
    (op & 0x7F)
        | ((imm << 7) & 0x00000F80)
        | ((funct3 << 12) & 0x00007000)
        | ((rs1 << 15) & 0x000F8000)
        | ((rs2 << 20) & 0x01F00000)
        | ((imm << 20) & 0xFE000000)
}

const fn crv_compose_u(op: u32, rd: u32, imm: u32) -> u32 {
    (op & 0x7F) | ((rd << 7) & 0x00000F80) | (imm & 0xFFFFF000)
}

const fn crv_compose_b(op: u32, rs1: u32, rs2: u32, funct3: u32, imm: u32) -> u32 {
    (op & 0x7F)
        | ((funct3 << 12) & 0x00007000)
        | ((rs1 << 15) & 0x000F8000)
        | ((rs2 << 20) & 0x01F00000)
        | ((imm << 19) & 0x80000000)
        | ((imm << 20) & 0x7E000000)
        | ((imm << 7) & 0x00000F00)
        | ((imm >> 4) & 0x00000080)
}

// Immediate extraction functions
const fn imm_c0m0(cmd: u16) -> u32 {
    (((cmd & 0x1000) >> 7)
        | ((cmd & 0x0800) >> 7)
        | ((cmd & 0x0400) >> 1)
        | ((cmd & 0x0200) >> 1)
        | ((cmd & 0x0100) >> 1)
        | ((cmd & 0x0080) >> 1)
        | ((cmd & 0x0040) >> 4)
        | ((cmd & 0x0020) >> 2)) as u32
}

const fn imm_c0m2(cmd: u16) -> u32 {
    (((cmd & 0x1000) >> 7) | //5
     ((cmd & 0x0800) >> 7) | //4
     ((cmd & 0x0400) >> 7) | //3
     ((cmd & 0x0040) >> 4) | //2
     ((cmd & 0x0020) << 1)) as u32 //6
}

const fn imm_c0m3(cmd: u16) -> u32 {
    (((cmd & 0x1000) >> 7)
        | ((cmd & 0x0800) >> 7)
        | ((cmd & 0x0400) >> 7)
        | ((cmd & 0x0040) << 1)
        | ((cmd & 0x0020) << 1)) as u32
}

const fn imm_c1m0(cmd: u16) -> u32 {
    sign_extend_6(
        (((cmd & 0x1000) >> 7)
            | ((cmd & 0x0040) >> 2)
            | ((cmd & 0x0020) >> 2)
            | ((cmd & 0x0010) >> 2)
            | ((cmd & 0x0008) >> 2)
            | ((cmd & 0x0004) >> 2)) as u32,
    )
}

const fn imm_c1m3(cmd: u16) -> u32 {
    sign_extend_10(
        (((cmd & 0x1000) >> 3)
            | ((cmd & 0x0040) >> 2)
            | ((cmd & 0x0020) << 1)
            | ((cmd & 0x0010) << 4)
            | ((cmd & 0x0008) << 4)
            | ((cmd & 0x0004) << 3)) as u32,
    )
}

const fn imm_c1m5(cmd: u16) -> u32 {
    sign_extend_12(
        (((cmd & 0x1000) >> 1) | //12 11
         ((cmd & 0x0800) >> 7) | //11  4
         ((cmd & 0x0400) >> 1) | //10  9
         ((cmd & 0x0200) >> 1) | // 9  8
         ((cmd & 0x0100) << 2) | // 8 10
         ((cmd & 0x0080) >> 1) | // 7  6
         ((cmd & 0x0040) << 1) | // 6  7
         ((cmd & 0x0020) >> 2) | // 5  3
         ((cmd & 0x0010) >> 2) | // 4  2
         ((cmd & 0x0008) >> 2) | // 3  1
         ((cmd & 0x0004) << 3)) as u32, // 2  5
    )
}

const fn imm_c1m6(cmd: u16) -> u32 {
    sign_extend_9(
        (((cmd & 0x1000) >> 4) | //8
         ((cmd & 0x0800) >> 7) | //4
         ((cmd & 0x0400) >> 7) | //3
         ((cmd & 0x0040) << 1) | //7
         ((cmd & 0x0020) << 1) | //6
         ((cmd & 0x0010) >> 2) | //2
         ((cmd & 0x0008) >> 2) | //1
         ((cmd & 0x0004) << 3)) as u32, //5
    )
}

const fn imm_c2m2(cmd: u16) -> u32 {
    (((cmd & 0x1000) >> 7) | //5
     ((cmd & 0x0040) >> 2) | //4
     ((cmd & 0x0020) >> 2) | //3
     ((cmd & 0x0010) >> 2) | //2
     ((cmd & 0x0008) << 4) | //7
     ((cmd & 0x0004) << 4)) as u32 //6
}

const fn imm_c2m3(cmd: u16) -> u32 {
    (((cmd & 0x1000) >> 7) | //5
     ((cmd & 0x0040) >> 2) | //4
     ((cmd & 0x0020) >> 2) | //3
     ((cmd & 0x0010) << 4) | //8
     ((cmd & 0x0008) << 4) | //7
     ((cmd & 0x0004) << 4)) as u32 //6
}

const fn imm_c2m6(cmd: u16) -> u32 {
    (((cmd & 0x1000) >> 7) | //5
     ((cmd & 0x0800) >> 7) | //4
     ((cmd & 0x0400) >> 7) | //3
     ((cmd & 0x0200) >> 7) | //2
     ((cmd & 0x0100) >> 1) | //7
     ((cmd & 0x0080) >> 1)) as u32 //6
}

const fn imm_c2m7(cmd: u16) -> u32 {
    (((cmd & 0x1000) >> 7) | //5
     ((cmd & 0x0800) >> 7) | //4
     ((cmd & 0x0400) >> 7) | //3
     ((cmd & 0x0200) >> 1) | //8
     ((cmd & 0x0100) >> 1) | //7
     ((cmd & 0x0080) >> 1)) as u32 //6
}

// Table generation
pub const fn generate_tables() -> [[u32; 16384]; 3] {
    let mut tables = [[0u32; 16384]; 3];

    let mut i = 0;
    while i < 16384 {
        let w = (i << 2) as u16;
        tables[0][i] = crv_decompress_real(w);

        let w = ((i << 2) | 1) as u16;
        tables[1][i] = crv_decompress_real(w);

        let w = ((i << 2) | 2) as u16;
        tables[2][i] = crv_decompress_real(w);

        i += 1;
    }

    tables
}

pub fn write_tables<W: std::io::Write>(
    writer: &mut W,
    tables: &[[u32; 16384]; 3],
) -> std::io::Result<()> {
    writeln!(
        writer,
        "// This file is generated by riscv-c-tables. Do not edit. \n// This file contains lookup tables for decompressing RISC-V C instructions."
    )?;
    for (q, table) in tables.iter().enumerate() {
        writeln!(
            writer,
            "//############################################################################//"
        )?;
        writeln!(writer, "pub const RISCV_C2G_C{}: [u32; 16384] = [", q)?;

        for (i, &value) in table.iter().enumerate() {
            if i % 8 == 0 {
                if i > 0 {
                    write!(writer, "   //{:04X}\n    ", i - 8)?;
                } else {
                    write!(writer, "    ")?;
                }
            }
            write!(writer, "0x{:08X},", value)?;
        }
        writeln!(writer, "   //{:04X}", 16384 - 8)?;
        writeln!(writer, "];\n")?;
    }
    writeln!(
        writer,
        "//############################################################################//"
    )?;
    Ok(())
}

// Main decompression function
const fn crv_decompress_real(cmd: u16) -> u32 {
    if cmd == 0 {
        return 0;
    }

    let op = cmd & 0x7F;
    let imm3 = (cmd >> 13) & 0x07;
    let b12 = (cmd >> 12) & 0x01;
    let a = (cmd >> 7) & 0x1F;
    let b = (cmd >> 2) & 0x1F;
    let a3 = 8 + ((cmd >> 7) & 0x07);
    let b3 = 8 + ((cmd >> 2) & 0x07);

    match op & 0x03 {
        OP_C0 => match imm3 {
            OPC_M0 => crv_compose_i(
                CRV_IOP_IMM,
                2,
                ((b & 0x07) + 8) as u32,
                CRV_F3_ADDI,
                imm_c0m0(cmd),
            ), // c.addi4spn
            OPC_M2 => crv_compose_i(CRV_IOP_LOAD, a3 as u32, b3 as u32, CRV_F3_LW, imm_c0m2(cmd)), // c.lw
            OPC_M3 => crv_compose_i(CRV_IOP_LOAD, a3 as u32, b3 as u32, CRV_F3_LD, imm_c0m3(cmd)), // c.ld
            OPC_M6 => crv_compose_s(
                CRV_IOP_STORE,
                a3 as u32,
                b3 as u32,
                CRV_F3_SW,
                imm_c0m2(cmd),
            ), // c.sw
            OPC_M7 => crv_compose_s(
                CRV_IOP_STORE,
                a3 as u32,
                b3 as u32,
                CRV_F3_SD,
                imm_c0m3(cmd),
            ), // c.sd
            _ => 0, // Invalid
        },
        OP_C1 => match imm3 {
            OPC_M0 => {
                if cmd & 0xFFFC == 0 {
                    crv_compose_i(CRV_IOP_IMM, 0, 0, CRV_F3_ADDI, 0) // c.nop
                } else {
                    crv_compose_i(CRV_IOP_IMM, a as u32, a as u32, CRV_F3_ADDI, imm_c1m0(cmd))
                    // c.addi
                }
            }
            OPC_M1 => crv_compose_i(
                CRV_IOP_IMM32,
                a as u32,
                a as u32,
                CRV_F3_ADDIW,
                imm_c1m0(cmd),
            ), // c.addiw
            OPC_M2 => crv_compose_i(CRV_IOP_IMM, 0, a as u32, CRV_F3_ADDI, imm_c1m0(cmd)), // c.li
            OPC_M3 => match a {
                2 => crv_compose_i(CRV_IOP_IMM, 2, 2, CRV_F3_ADDI, imm_c1m3(cmd)), // c.addi16sp
                0 => 0,                                                            // Invalid
                _ => crv_compose_u(CRV_IOP_LUI, a as u32, imm_c1m0(cmd) << 12),    // c.lui
            },
            OPC_M4 => match a >> 3 {
                0 => crv_compose_i(
                    CRV_IOP_IMM,
                    a3 as u32,
                    a3 as u32,
                    CRV_F3_SRLSRAI,
                    ((b12 << 5) | b) as u32,
                ), // c.srli
                1 => crv_compose_i(
                    CRV_IOP_IMM,
                    a3 as u32,
                    a3 as u32,
                    CRV_F3_SRLSRAI,
                    (0x400 | (b12 << 5) | b) as u32,
                ), // c.srai
                2 => crv_compose_i(
                    CRV_IOP_IMM,
                    a3 as u32,
                    a3 as u32,
                    CRV_F3_ANDI,
                    sign_extend_6(((b12 << 5) | b) as u32),
                ), // c.andi
                3 => match b12 {
                    0 => match b >> 3 {
                        0 => crv_compose_r(
                            CRV_IOP_OP,
                            a3 as u32,
                            b3 as u32,
                            a3 as u32,
                            CRV_F3_ADDSUB,
                            32,
                        ), // c.sub
                        1 => crv_compose_r(
                            CRV_IOP_OP, a3 as u32, b3 as u32, a3 as u32, CRV_F3_XOR, 0,
                        ), // c.xor
                        2 => {
                            crv_compose_r(CRV_IOP_OP, a3 as u32, b3 as u32, a3 as u32, CRV_F3_OR, 0)
                        } // c.or
                        3 => crv_compose_r(
                            CRV_IOP_OP, a3 as u32, b3 as u32, a3 as u32, CRV_F3_AND, 0,
                        ), // c.and
                        _ => 0,
                    },
                    1 => match b >> 3 {
                        0 => crv_compose_r(
                            CRV_IOP_OP32,
                            a3 as u32,
                            b3 as u32,
                            a3 as u32,
                            CRV_F3_ADDSUBW,
                            32,
                        ), // c.subw
                        1 => crv_compose_r(
                            CRV_IOP_OP32,
                            a3 as u32,
                            b3 as u32,
                            a3 as u32,
                            CRV_F3_ADDSUBW,
                            0,
                        ), // c.addw
                        _ => 0, // Invalid
                    },
                    _ => 0,
                },
                _ => 0,
            },
            OPC_M5 => crv_compose_j(CRV_IOP_JAL, 0, imm_c1m5(cmd)), // c.j
            OPC_M6 => crv_compose_b(CRV_IOP_BRANCH, a3 as u32, 0, CRV_F3_BEQ, imm_c1m6(cmd)), // c.beqz
            OPC_M7 => crv_compose_b(CRV_IOP_BRANCH, a3 as u32, 0, CRV_F3_BNE, imm_c1m6(cmd)), // c.bnez
            _ => 0, // Invalid
        },
        OP_C2 => match imm3 {
            OPC_M0 => crv_compose_i(
                CRV_IOP_IMM,
                a as u32,
                a as u32,
                CRV_F3_SLLI,
                ((b12 << 5) | b) as u32,
            ), // c.slli
            OPC_M2 => crv_compose_i(CRV_IOP_LOAD, 2, a as u32, CRV_F3_LW, imm_c2m2(cmd)), // c.lwsp
            OPC_M3 => crv_compose_i(CRV_IOP_LOAD, 2, a as u32, CRV_F3_LD, imm_c2m3(cmd)), // c.ldsp
            OPC_M4 => match b12 {
                0 => match b {
                    0 => crv_compose_i(CRV_IOP_JALR, a as u32, 0, 0, 0), // c.jr
                    _ => crv_compose_r(CRV_IOP_OP, 0, b as u32, a as u32, CRV_F3_ADDSUB, 0), // c.mv
                },
                1 => match b {
                    0 => match a {
                        0 => 0,                                              // c.ebreak
                        _ => crv_compose_i(CRV_IOP_JALR, a as u32, 1, 0, 0), // c.jalr
                    },
                    _ => crv_compose_r(CRV_IOP_OP, a as u32, b as u32, a as u32, CRV_F3_ADDSUB, 0), // c.add
                },
                _ => 0,
            },
            OPC_M6 => crv_compose_s(CRV_IOP_STORE, 2, b as u32, CRV_F3_SW, imm_c2m6(cmd)), // c.swsp
            OPC_M7 => crv_compose_s(CRV_IOP_STORE, 2, b as u32, CRV_F3_SD, imm_c2m7(cmd)), // c.sdsp
            _ => 0, // Invalid
        },
        _ => 0, // Invalid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_extension() {
        assert_eq!(sign_extend_6(0x20), 0xFFFFFFC0);
        assert_eq!(sign_extend_6(0x1F), 0x1F);
        assert_eq!(sign_extend_9(0x100), 0xFFFFFE00);
        assert_eq!(sign_extend_9(0xFF), 0xFF);
        assert_eq!(sign_extend_10(0x200), 0xFFFFFC00);
        assert_eq!(sign_extend_10(0x1FF), 0x1FF);
        assert_eq!(sign_extend_12(0x800), 0xFFFFF000);
        assert_eq!(sign_extend_12(0x7FF), 0x7FF);
    }

    #[test]
    fn test_immediate_extraction() {
        // Test c.addi4spn immediate
        assert_eq!(imm_c0m0(0x1000), 0x80);
        assert_eq!(imm_c0m0(0x0020), 0x08);

        // Test c.lw immediate
        assert_eq!(imm_c0m2(0x1000), 0x20);
        assert_eq!(imm_c0m2(0x0020), 0x40);

        // Test c.ld immediate
        assert_eq!(imm_c0m3(0x1000), 0x20);
        assert_eq!(imm_c0m3(0x0020), 0x40);
    }

    #[test]
    fn test_basic_decompression() {
        // Test c.nop (compressed)
        assert_eq!(
            crv_decompress_real(0x0001),
            crv_compose_i(CRV_IOP_IMM, 0, 0, CRV_F3_ADDI, 0)
        );

        // Test c.addi4spn
        let cmd = (1 << 13) | (0x1F << 2); // Example compressed instruction
        let expected = crv_compose_i(CRV_IOP_IMM, 2, 15, CRV_F3_ADDI, imm_c0m0(cmd as u16));
        assert_eq!(crv_decompress_real(cmd as u16), expected);
    }

    #[test]
    fn test_instruction_composition() {
        // Test J-type instruction composition
        assert_eq!(crv_compose_j(CRV_IOP_JAL, 1, 0x123), CRV_IOP_JAL | (1 << 7));

        // Test I-type instruction composition
        assert_eq!(
            crv_compose_i(CRV_IOP_IMM, 1, 2, CRV_F3_ADDI, 0x123),
            (CRV_IOP_IMM | (2 << 7) | (CRV_F3_ADDI << 12) | (1 << 15) | (0x123 << 20))
        );
    }

    #[test]
    fn test_table_generation() {
        let tables = generate_tables();
        assert_eq!(tables.len(), 3);
        assert_eq!(tables[0].len(), 16384);
        assert_eq!(tables[1].len(), 16384);
        assert_eq!(tables[2].len(), 16384);

        // Test that table 0 starts correctly (should contain decompressed instructions)
        assert_ne!(tables[0][1], 0); // Most entries shouldn't be zero

        // Test that invalid instructions decompress to 0
        assert_eq!(tables[0][0], 0);
    }

    #[test]
    fn test_cli() {
        let tables = generate_tables();
        let inst: u16 = 0x200d; // c.li
        let idx = inst >> 2;
        let decompressed = tables[1][idx as usize];
        println!("Decompressed: {:08X}", decompressed);
    }
}
