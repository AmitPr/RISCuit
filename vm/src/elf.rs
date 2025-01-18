use std::collections::HashMap;

use goblin::elf::{
    dynamic::DT_NEEDED,
    program_header::{PT_DYNAMIC, PT_LOAD},
};

use crate::cpu::Hart32;

pub fn load_elf<'a>(cpu: &mut Hart32, bytes: &'a [u8]) -> goblin::elf::Elf<'a> {
    let file = goblin::elf::Elf::parse(bytes).expect("Failed to parse ELF");

    // Track base addresses of loaded libraries
    let mut loaded_libs: HashMap<String, u32> = HashMap::new();
    // Track resolved symbols
    let mut symbol_map: HashMap<String, u32> = HashMap::new();

    // Helper function to load an ELF at a specific base address
    fn load_segments(cpu: &mut Hart32, elf: &goblin::elf::Elf, data: &[u8], base_addr: u32) {
        for ph in &elf.program_headers {
            if ph.p_type == PT_LOAD {
                let file_start = ph.p_offset as usize;
                let vaddr = base_addr + ph.p_vaddr as u32;

                unsafe {
                    // Load segment data
                    std::ptr::copy(
                        data.as_ptr().add(file_start),
                        cpu.mem.pointer(vaddr),
                        ph.p_filesz as usize,
                    );
                    // Zero remainder
                    std::ptr::write_bytes(
                        cpu.mem.pointer(vaddr).add(ph.p_filesz as usize),
                        0,
                        ph.p_memsz as usize - ph.p_filesz as usize,
                    );
                }
            }
        }
    }

    // Load main program at its preferred address
    load_segments(cpu, &file, bytes, 0);

    // Process dynamic section to load libraries
    if let Some(dynamic) = &file.dynamic {
        for dyn_entry in &dynamic.dyns {
            if dyn_entry.d_tag == DT_NEEDED {
                let lib_name = &file.dynstrtab[dyn_entry.d_val as usize];
                println!("Loading library: {}", lib_name);

                // Load library file (you'll want better path resolution)
                let lib_data = std::fs::read(lib_name).expect("Failed to read library");
                let lib_elf = goblin::elf::Elf::parse(&lib_data).expect("Failed to parse library");

                // Choose a base address for this library (simplified)
                let lib_base = 0x400000 + (loaded_libs.len() as u32 * 0x1000000);
                loaded_libs.insert(lib_name.to_string(), lib_base);

                // Load library segments
                load_segments(cpu, &lib_elf, &lib_data, lib_base);

                // Add library symbols to symbol map
                for sym in lib_elf.dynsyms.iter() {
                    if sym.st_value != 0 {
                        if let Some(name) = lib_elf.dynstrtab.get_at(sym.st_name) {
                            let sym_addr = lib_base + sym.st_value as u32;
                            symbol_map.insert(name.to_string(), sym_addr);
                        }
                    }
                }
            }
        }
    }

    // Process relocations
    for rela in &file.dynrelas {
        // Get symbol this relocation refers to
        let sym = &file.dynsyms.get(rela.r_sym).expect("Failed to get symbol");
        if let Some(sym_name) = file.dynstrtab.get_at(sym.st_name) {
            if let Some(&target_addr) = symbol_map.get(sym_name) {
                let reloc_addr = rela.r_offset as u32;

                match rela.r_type {
                    goblin::elf::reloc::R_RISCV_RELATIVE => {
                        // Handle relative relocation
                        let value = cpu.mem.load::<u32>(reloc_addr);
                        cpu.mem.store::<u32>(reloc_addr, value + target_addr);
                    }
                    goblin::elf::reloc::R_RISCV_JUMP_SLOT => {
                        // Handle PLT/GOT entry
                        cpu.mem.store::<u32>(reloc_addr, target_addr);
                    }
                    goblin::elf::reloc::R_RISCV_32 => {
                        // Handle 32-bit absolute relocation
                        cpu.mem.store::<u32>(reloc_addr, target_addr);
                    }
                    _ => println!("Unhandled relocation type: {}", rela.r_type),
                }
            }
        }
    }

    file
}
