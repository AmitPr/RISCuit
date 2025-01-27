use std::{collections::BTreeMap, ffi::CString};

use clap::Parser;
use riscv_vm::{cpu::Hart32, elf::load_elf, initialize::setup_stack, riscv_inst::Reg};

#[derive(Debug, Parser)]
struct Args {
    /// Path to the ELF file to run
    elf_path: String,
    /// Breakpoint address
    #[clap(short, value_parser = maybe_hex, num_args = 0..)]
    breakpoints: Vec<u32>,
}

fn maybe_hex(s: &str) -> Result<u32, std::num::ParseIntError> {
    if s.starts_with("0x") {
        u32::from_str_radix(s.trim_start_matches("0x"), 16)
    } else {
        s.parse::<u32>()
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .without_time()
        .init();

    let args = Args::parse();

    let mut cpu = Hart32::new();
    let elf = std::fs::read(&args.elf_path).expect("Failed to read ELF file");
    let elf = load_elf(&mut cpu, &elf);

    let filename = CString::new(args.elf_path.split('/').last().unwrap())
        .expect("Failed to convert filename to CString");
    setup_stack(&mut cpu, &[filename.as_c_str()], &[]);

    let mut debugger = Debugger::new(cpu, elf, args.breakpoints);

    debugger.run();
}

enum Mode {
    Running,
    Debugging,
}

struct Debugger {
    cpu: Hart32,
    mode: Mode,
    syms: BTreeMap<u32, String>,
    last_sym: Option<String>,
    breakpoints: Vec<u32>,
}

impl Debugger {
    pub fn new(cpu: Hart32, elf: goblin::elf::Elf, breakpoints: Vec<u32>) -> Self {
        let syms = elf
            .syms
            .iter()
            .filter_map(|sym| {
                if sym.is_function() {
                    elf.strtab
                        .get_at(sym.st_name)
                        .map(|name| (sym.st_value as u32, name.to_string()))
                } else {
                    None
                }
            })
            .collect();

        Self {
            cpu,
            mode: Mode::Running,
            syms,
            last_sym: None,
            breakpoints,
        }
    }

    pub fn run(&mut self) {
        while self.cpu.running {
            if let Some((pc, sym)) = self.syms.range(..=self.cpu.pc).next_back() {
                if self.last_sym.as_deref() != Some(sym) {
                    if self.cpu.pc == *pc {
                        let reg_state = self
                            .cpu
                            .regs_range(Reg::A0, Reg::A7)
                            .map(|(r, v)| format!("{r:?}=0x{v:x}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        tracing::debug!("At {}({})", sym, reg_state);
                    } else {
                        tracing::debug!("In {}", sym);
                    }
                    self.last_sym = Some(sym.clone());
                }
            }
            self.cpu.step().expect("Failed to step");

            if self.breakpoints.contains(&self.cpu.pc) {
                tracing::info!("Breakpoint hit at 0x{:08x}", self.cpu.pc);
                self.mode = Mode::Debugging;
            }

            match self.mode {
                Mode::Running => {}
                Mode::Debugging => loop {
                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    let mut input = input.split_whitespace();
                    let cmd = input.next();

                    match cmd {
                        Some("c") => {
                            self.mode = Mode::Running;
                            break;
                        }
                        Some("s") => {
                            break;
                        }
                        Some("r") => {
                            for (reg, val) in self.cpu.regs() {
                                tracing::info!("{:?}: 0x{:08x}", reg, val);
                            }
                        }
                        Some("q") => {
                            self.cpu.running = false;
                            break;
                        }
                        Some("lw") => {
                            let addr = input.next();
                            if addr.is_none() {
                                tracing::error!("Usage: lw <addr>");
                                continue;
                            }
                            let addr = maybe_hex(addr.unwrap());
                            if addr.is_err() {
                                tracing::error!("Invalid address");
                                continue;
                            }
                            let addr = addr.unwrap();
                            let val = self.cpu.mem.load::<u32>(addr);
                            tracing::info!("0x{:08x}: 0x{:08x}", addr, val);
                        }
                        _ => tracing::info!("Unknown command"),
                    }
                },
            }
        }

        tracing::info!("Instructions executed: {}", self.cpu.inst_count);
    }
}
