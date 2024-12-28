mod cpu;
mod elf;
mod memory;
mod memory64;

use cpu::Cpu;
use elf::load_elf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <program.elf>", args[0]);
        return;
    }
    let elf_path = &args[1];

    let mut cpu = Cpu::new();
    let entry = load_elf(&mut cpu, elf_path);
    cpu.pc = entry as u64;

    println!("Starting at 0x{:08x}", entry);
    cpu.run();
}
