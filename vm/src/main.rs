mod cpu;
mod elf;
mod memory;

use cpu::Cpu32;
use elf::load_elf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <program.elf>", args[0]);
        return;
    }
    let elf_path = &args[1];

    let mut cpu = Cpu32::new();
    let entry = load_elf(&mut cpu, elf_path);
    cpu.pc = entry;

    println!("Starting at 0x{:08x}", entry);
    cpu.run();
}
