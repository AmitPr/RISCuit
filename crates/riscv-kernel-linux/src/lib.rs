mod impls;

use std::ffi::CString;

use goblin::elf::{program_header::PT_LOAD, Elf};

use riscv_vm::{
    error::MachineError,
    hart::Hart32,
    machine::{Kernel, StepResult},
    memory::{Memory, Memory32},
    riscv_inst::Reg,
};
use syscalls::riscv32::Sysno;
use thiserror::Error;

const PAGE_SIZE: u32 = 4096;

#[derive(Error, Debug)]
pub enum LinuxError {}

#[derive(Default, Debug)]
pub struct MockLinux {
    exit_code: Option<u32>,
    passthrough_stdio: bool,
}

impl Kernel for MockLinux {
    type Error = LinuxError;
    type Memory = Memory32;

    fn syscall(
        &mut self,
        hart: &mut Hart32,
        mem: &mut Memory32,
    ) -> Result<StepResult, MachineError<Self::Error>> {
        macro_rules! reg {
            ($reg: ident) => {
                hart.get_reg(Reg::$reg) as _
            };
        }

        let call: usize = reg!(A7);
        let parsed = Sysno::new(call);
        if parsed.is_none() {
            tracing::error!("SYSCALL({call}) unknown");
            hart.set_reg(Reg::A0, -libc_riscv32::ENOSYS as u32);
            return Ok(StepResult::Ok);
        }
        tracing::debug!("SYSCALL({call}) -> {parsed:?}");

        let call = parsed.unwrap();
        let ret = match call {
            Sysno::ioctl => self.ioctl(reg!(A0), reg!(A1)),
            Sysno::write => self.write(mem, reg!(A0), reg!(A1), reg!(A2)),
            Sysno::writev => self.writev(mem, reg!(A0), reg!(A1), reg!(A2)),
            Sysno::readlinkat => self.readlinkat(mem, reg!(A0), reg!(A1), reg!(A2), reg!(A3)),
            Sysno::exit | Sysno::exit_group => {
                self.exit_code = Some(reg!(A0));
                return Ok(StepResult::Halt);
            }
            Sysno::set_tid_address => self.set_tid_address(mem, reg!(A0)),
            Sysno::futex => self.futex(
                mem,
                reg!(A0),
                reg!(A1),
                reg!(A2),
                reg!(A3),
                reg!(A4),
                reg!(A5),
            ),
            Sysno::set_robust_list => self.set_robust_list(mem, reg!(A0), reg!(A1)),
            Sysno::tgkill => self.tgkill(reg!(A0), reg!(A1), reg!(A2)),
            Sysno::rt_sigaction => self.rt_sigaction(mem, reg!(A0), reg!(A1), reg!(A2), reg!(A3)),
            Sysno::rt_sigprocmask => {
                self.rt_sigprocmask(mem, reg!(A0), reg!(A1), reg!(A2), reg!(A3))
            }
            Sysno::getpid => self.getpid(),
            Sysno::gettid => self.gettid(),
            Sysno::brk => self.brk(mem, reg!(A0)),
            Sysno::mmap => self.mmap(
                mem,
                reg!(A0),
                reg!(A1),
                reg!(A2),
                reg!(A3),
                reg!(A4),
                reg!(A5),
            ),
            Sysno::mprotect => self.mprotect(mem, reg!(A0), reg!(A1), reg!(A2)),
            Sysno::riscv_hwprobe => {
                self.riscv_hwprobe(mem, reg!(A0), reg!(A1), reg!(A2), reg!(A3), reg!(A4))
            }
            Sysno::getrlimit => self.getrlimit(mem, reg!(A0), reg!(A1)),
            Sysno::getrandom => self.getrandom(mem, reg!(A0), reg!(A1), reg!(A2)),
            Sysno::statx => self.statx(mem, reg!(A0), reg!(A1), reg!(A2), reg!(A3), reg!(A4)),
            Sysno::ppoll_time64 => {
                self.ppoll_time64(mem, reg!(A0), reg!(A1), reg!(A2), reg!(A3), reg!(A4))
            }
            Sysno::futex_time64 => self.futex(
                mem,
                reg!(A0),
                reg!(A1),
                reg!(A2),
                reg!(A3),
                reg!(A4),
                reg!(A5),
            ),
            _ => {
                tracing::error!("SYSCALL({call}) unimplemented");
                Err(libc_riscv32::ENOSYS)
            }
        }
        .unwrap_or_else(|e| -e as u32);

        hart.set_reg(Reg::A0, ret as u32);

        Ok(StepResult::Ok)
    }
}

impl MockLinux {
    pub fn new(passthrough_stdio: bool) -> Self {
        Self {
            exit_code: None,
            passthrough_stdio,
        }
    }

    pub fn exit_code(&self) -> Option<u32> {
        self.exit_code
    }

    pub fn load_static_elf<'a>(
        &mut self,
        hart: &mut Hart32,
        mem: &mut Memory32,
        bytes: &'a [u8],
        args: &[&str],
        env: &[&str],
    ) -> Elf<'a> {
        let elf = Elf::parse(bytes).expect("Failed to parse ELF");
        // Load main program segments
        let mut brk = 0;
        for ph in &elf.program_headers {
            if ph.p_type == PT_LOAD {
                let vaddr = ph.p_vaddr as u32;
                for i in 0..ph.p_filesz as usize {
                    mem.store::<u8>(vaddr + i as u32, bytes[ph.p_offset as usize + i]);
                }
                // BSS already zero since fresh mmap
                if vaddr + ph.p_memsz as u32 > brk {
                    brk = vaddr + ph.p_memsz as u32;
                }
            }
        }

        // PC
        hart.pc = elf.entry as u32;

        // Align and set brk
        brk = (brk + 0xfff) & !0xfff;
        mem.brk = brk;
        // Global pointer is at __DATA_BEGIN__
        // TODO: Do we actually need to set this? Or does libc initialize it on its own?
        let data_begin = elf
            .syms
            .iter()
            .find(|sym| elf.strtab.get_at(sym.st_name) == Some("__DATA_BEGIN__"))
            .map(|sym| sym.st_value as u32)
            .unwrap_or(0);
        hart.set_reg(Reg::Gp, data_begin);

        // Setup Stack.
        let mut sp = 0xCFFF_F000u32;
        let mut stack_init: Vec<u32> = vec![];

        // Arguments
        stack_init.push(args.len() as u32); // argc
        for &arg in args.iter().rev() {
            let bytes = CString::new(arg).expect("argument contains null byte");
            let bytes = bytes.to_bytes_with_nul();

            sp = (sp - bytes.len() as u32) & !(4 - 1); // align to 4 bytes
            mem.copy_to(sp, bytes)
                .expect("Failed to copy argument to stack");

            stack_init.push(sp); // pointer to arg
        }
        stack_init.push(0); // argv NULL terminator

        // Environment
        for &e in env.iter().rev() {
            let bytes = CString::new(e).expect("environment variable contains null byte");
            let bytes = bytes.to_bytes_with_nul();

            sp = (sp - bytes.len() as u32) & !(4 - 1); // align to 4 bytes
            mem.copy_to(sp, bytes)
                .expect("Failed to copy environment variable to stack");

            stack_init.push(sp);
        }
        stack_init.push(0); // envp NULL terminator

        // ELF Auxillary Vector
        macro_rules! set_AT {
            ($at:expr, $val:expr) => {
                stack_init.push($at as u32);
                stack_init.push($val);
            };
        }
        // TODO: Fill in the rest of the aux vector
        set_AT!(libc_riscv32::AT_PAGESZ, PAGE_SIZE);
        set_AT!(libc_riscv32::AT_CLKTCK, 100);
        set_AT!(libc_riscv32::AT_BASE, 0); // TODO
        set_AT!(libc_riscv32::AT_ENTRY, hart.pc);
        set_AT!(libc_riscv32::AT_NULL, 0);

        // Set up stack
        sp -= (stack_init.len() * 4) as u32;

        mem.copy_to(sp, &stack_init)
            .expect("Failed to copy stack data vector");

        hart.set_reg(Reg::Sp, sp);

        tracing::debug!("Stack at {:#x}, GP at {:#x}", sp, data_begin);
        tracing::debug!("Loaded ELF. Start at {:08x}, brk={:08x}", elf.entry, brk);

        elf
    }
}
