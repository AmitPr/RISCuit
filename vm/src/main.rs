mod cpu;
mod elf;
mod memory;
mod syscall;

use cpu::Hart32;
use elf::load_elf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <program.elf>", args[0]);
        return;
    }
    let elf_path = &args[1];

    let mut cpu = Hart32::new();
    let elf = std::fs::read(elf_path).expect("Failed to read ELF file");
    let elf = load_elf(&mut cpu, &elf);
    cpu.pc = elf.entry as u32;
    // TODO: we are setting stack pointer way too high
    cpu.set_reg(2, u32::MAX);
    // TODO: How do we set the brk correctly?
    cpu.mem.brk = 0x8000000;

    println!("Starting at 0x{:08x}", cpu.pc);
    cpu.run(elf);
}
