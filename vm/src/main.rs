mod cpu;
mod elf;
mod memory;
mod syscall;

use cpu::Hart32;
use elf::load_elf;
use riscv_inst::Reg;

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
    // TODO: what to set stack pointer to initially?
    let sp = 0xc0000000u32 - 0x1000;
    cpu.set_reg(Reg::Sp, sp);

    println!("Starting at 0x{:08x}", cpu.pc);
    cpu.run(elf);
}
