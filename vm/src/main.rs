use derisc::{cpu::Hart32, elf::load_elf};

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

    println!("Starting at 0x{:08x}", cpu.pc);
    cpu.run(elf).expect("Failed to run program");
}
