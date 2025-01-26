use std::collections::HashMap;

use goblin::elf::{
    dynamic::DT_NEEDED,
    program_header::PT_LOAD,
    reloc::{R_RISCV_32, R_RISCV_JUMP_SLOT, R_RISCV_RELATIVE},
};
use riscv_inst::Reg;

use crate::cpu::Hart32;

pub fn load_elf<'a>(cpu: &mut Hart32, bytes: &'a [u8]) -> goblin::elf::Elf<'a> {
    let elf = goblin::elf::Elf::parse(bytes).expect("Failed to parse ELF");
    let mut symbols = HashMap::new();
    let mut loaded_libs = HashMap::new();

    // Helper to load a single library and its dependencies
    fn load_library(
        cpu: &mut Hart32,
        lib_name: &str,
        base_addr: u32,
        symbols: &mut HashMap<String, u32>,
        loaded_libs: &mut HashMap<String, u32>,
    ) -> u32 {
        if let Some(&addr) = loaded_libs.get(lib_name) {
            return addr;
        }

        let lib_data = std::fs::read(lib_name).expect("Failed to read library");
        let lib_elf = goblin::elf::Elf::parse(&lib_data).expect("Failed to parse library");

        // Record this library's base address
        loaded_libs.insert(lib_name.to_string(), base_addr);

        // Load its dependencies first
        let mut next_base = base_addr + 0x1000000;
        if let Some(dynamic) = &lib_elf.dynamic {
            for dyn_entry in &dynamic.dyns {
                if dyn_entry.d_tag == DT_NEEDED {
                    let dep_name = lib_elf.dynstrtab[dyn_entry.d_val as usize].to_string();
                    next_base = load_library(cpu, &dep_name, next_base, symbols, loaded_libs);
                }
            }
        }

        // Load segments
        println!("Loading library: {} at {:08x}", lib_name, base_addr);
        for ph in &lib_elf.program_headers {
            if ph.p_type == PT_LOAD {
                let vaddr = base_addr + ph.p_vaddr as u32;
                for i in 0..ph.p_filesz as usize {
                    cpu.mem
                        .store::<u8>(vaddr + i as u32, lib_data[ph.p_offset as usize + i]);
                }
                // Zero BSS
                for i in ph.p_filesz as usize..ph.p_memsz as usize {
                    cpu.mem.store::<u8>(vaddr + i as u32, 0);
                }
            }
        }

        // Add symbols
        for sym in lib_elf.dynsyms.iter() {
            if let Some(name) = lib_elf.dynstrtab.get_at(sym.st_name) {
                symbols.insert(name.to_string(), base_addr + sym.st_value as u32);
            }
        }

        // Process relocations
        for rela in &lib_elf.dynrelas {
            let sym = &lib_elf
                .dynsyms
                .get(rela.r_sym)
                .expect("Invalid symbol index");
            if let Some(name) = lib_elf.dynstrtab.get_at(sym.st_name) {
                let target = symbols.get(name).copied().unwrap_or(0);
                let addr = base_addr + rela.r_offset as u32;

                match rela.r_type {
                    R_RISCV_RELATIVE | R_RISCV_32 | R_RISCV_JUMP_SLOT => {
                        cpu.mem.store::<u32>(addr, target);
                    }
                    _ => println!("Unhandled relocation: {} at {:#x}", rela.r_type, addr),
                }
            }
        }

        next_base
    }

    // Load main program segments
    let mut brk = 0;
    for ph in &elf.program_headers {
        if ph.p_type == PT_LOAD {
            let vaddr = ph.p_vaddr as u32;
            for i in 0..ph.p_filesz as usize {
                cpu.mem
                    .store::<u8>(vaddr + i as u32, bytes[ph.p_offset as usize + i]);
            }
            for i in ph.p_filesz as usize..ph.p_memsz as usize {
                cpu.mem.store::<u8>(vaddr + i as u32, 0);
            }
            if vaddr + ph.p_memsz as u32 > brk {
                brk = vaddr + ph.p_memsz as u32;
            }
        }
    }

    // Load all libraries starting at 0x40000000
    if let Some(dynamic) = &elf.dynamic {
        let mut next_base = 0x40000000;
        for dyn_entry in &dynamic.dyns {
            if dyn_entry.d_tag == DT_NEEDED {
                let lib_name = elf.dynstrtab[dyn_entry.d_val as usize].to_string();
                next_base = load_library(cpu, &lib_name, next_base, &mut symbols, &mut loaded_libs);
            }
        }
    }

    // Process relocations
    for rela in &elf.dynrelas {
        let sym = &elf.dynsyms.get(rela.r_sym).expect("Invalid symbol index");
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            let target = symbols.get(name).copied().unwrap_or(0);
            let addr = rela.r_offset as u32;

            match rela.r_type {
                R_RISCV_RELATIVE | R_RISCV_32 | R_RISCV_JUMP_SLOT => {
                    cpu.mem.store::<u32>(addr, target);
                }
                _ => println!("Unhandled relocation: {} at {:#x}", rela.r_type, addr),
            }
        }
    }
    // Align and set brk
    brk = (brk + 0xfff) & !0xfff;
    cpu.mem.brk = brk;
    println!("Loaded ELF at {:08x}, brk={:08x}", elf.entry, brk);
    // Stack
    // TODO: what to set stack pointer to initially?
    let sp = 0xCFFF_F000u32;
    cpu.set_reg(Reg::Sp, sp);
    // Global pointer is at __DATA_BEGIN__
    let data_begin = elf
        .syms
        .iter()
        .find(|sym| elf.strtab.get_at(sym.st_name) == Some("__DATA_BEGIN__"))
        .map(|sym| sym.st_value as u32)
        .unwrap_or(0);
    cpu.set_reg(Reg::Gp, data_begin);
    println!("Stack at {:#x}, GP at {:#x}", sp, data_begin);

    // PC
    cpu.pc = elf.entry as u32;

    elf
}
