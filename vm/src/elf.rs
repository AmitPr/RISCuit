use elf::endian::AnyEndian;
use elf::{abi::PT_LOAD, ElfBytes};

use crate::cpu::Cpu;

/// Copies PT_LOAD segments into memory, returns the ELF entry point.
pub fn load_elf(cpu: &mut Cpu, path: &str) -> u32 {
    let file_data = std::fs::read(path).expect("Failed to read ELF file");
    let file =
        ElfBytes::<AnyEndian>::minimal_parse(&file_data).expect("Failed to parse ELF header");

    let phdrs = file.segments().expect("Cannot read program headers");
    for phdr in phdrs {
        if phdr.p_type == PT_LOAD {
            let file_start = phdr.p_offset as usize;
            let file_end = file_start + phdr.p_filesz as usize;
            let vaddr = phdr.p_vaddr as u32;

            println!(
                "Loading segment: vaddr=0x{:08x}, file_start=0x{:08x}, file_end=0x{:08x}",
                vaddr, file_start, file_end
            );
            for i in 0..(phdr.p_filesz as usize) {
                let byte = file_data[file_start + i];
                cpu.mem.store_byte(vaddr + i as u32, byte);
            }
            // If p_memsz > p_filesz, zero out the rest
            for i in (phdr.p_filesz as usize)..(phdr.p_memsz as usize) {
                cpu.mem.store_byte(vaddr + i as u32, 0);
            }
        }
    }

    file.ehdr.e_entry as u32
}
