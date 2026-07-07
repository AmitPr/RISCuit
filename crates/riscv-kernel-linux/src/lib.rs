mod impls;

use std::ffi::CString;
use std::marker::PhantomData;

use goblin::elf::{program_header::PT_LOAD, Elf};

use riscv_vm::{
    error::MachineError,
    hart::{Execute, Hart, X32, X64, Xlen},
    machine::{Kernel, StepResult},
    memory::{Memory, Memory32, Memory64},
    riscv_inst::Reg,
};
use thiserror::Error;

const PAGE_SIZE: u64 = 4096;
/// Nominal stack reservation; brk may not grow into it.
const STACK_RESERVE: u64 = 8 << 20;

/// Per-width process layout and memory pairing for [`MockLinux`].
pub trait KernelXlen: Execute {
    type Memory: Memory<Addr = <Self as Xlen>::U> + Default;

    /// Initial stack top for a fresh process.
    const STACK_TOP: u64;
    /// Where anonymous mmap allocation starts...
    const MMAP_BASE: u64;
    /// ...and which way it moves. rv32 keeps the historical top-down layout
    /// inside its fixed 4 GiB; rv64 grows upward so the elastic arena only
    /// maps what is used.
    const MMAP_GROWS_DOWN: bool;
}

impl KernelXlen for X32 {
    type Memory = Memory32;
    const STACK_TOP: u64 = 0xCFFF_F000;
    const MMAP_BASE: u64 = 0xC000_0000;
    const MMAP_GROWS_DOWN: bool = true;
}

impl KernelXlen for X64 {
    type Memory = Memory64;
    // Stack lives just below 512 MiB; ELF + brk below it, mmap above,
    // so the arena grows with actual use.
    const STACK_TOP: u64 = 0x1FFF_F000;
    const MMAP_BASE: u64 = 0x2000_0000;
    const MMAP_GROWS_DOWN: bool = false;
}

#[derive(Error, Debug)]
pub enum LinuxError {}

#[derive(Debug)]
pub struct MockLinux<X: KernelXlen> {
    exit_code: Option<u64>,
    passthrough_stdio: bool,
    /// Current program break.
    pub(crate) brk: u64,
    /// brk may not reach this address (bottom of mmap region or stack).
    pub(crate) brk_limit: u64,
    /// Next anonymous mmap allocation position.
    pub(crate) mmap_cursor: u64,
    _xlen: PhantomData<X>,
}

pub type MockLinux32 = MockLinux<X32>;
pub type MockLinux64 = MockLinux<X64>;

impl<X: KernelXlen> Default for MockLinux<X> {
    fn default() -> Self {
        Self::new(false)
    }
}

/// The syscall dispatch body, instantiated once per width so each gets its
/// native `Sysno` table. `$extra` holds width-specific arms (e.g. rv32's
/// time64 variants).
macro_rules! syscall_dispatch {
    ($self:ident, $hart:ident, $mem:ident, $X:ty,
     [$a0:ident, $a1:ident, $a2:ident, $a3:ident, $a4:ident, $a5:ident, $a7:ident],
     { $($extra:tt)* }) => {{
        let [$a0, $a1, $a2, $a3, $a4, $a5, $a7] =
            [Reg::A0, Reg::A1, Reg::A2, Reg::A3, Reg::A4, Reg::A5, Reg::A7]
                .map(|r| <$X as Xlen>::to_u64($hart.get_reg(r)));

        let Some(call) = Sysno::new($a7 as usize) else {
            tracing::error!("SYSCALL({}) unknown", $a7);
            $hart.set_reg(Reg::A0, <$X as Xlen>::from_i64(-libc_riscv32::ENOSYS as i64));
            return Ok(StepResult::Ok);
        };
        tracing::debug!("SYSCALL({}) -> {call:?}", $a7);

        let ret: Result<u64, i32> = match call {
            Sysno::ioctl => $self.ioctl($a0 as i32, $a1),
            Sysno::write => $self.write($mem, $a0 as i32, $a1, $a2),
            Sysno::writev => $self.writev($mem, $a0 as i32, $a1, $a2 as i32),
            Sysno::readlinkat => $self.readlinkat($mem, $a0 as i32, $a1, $a2, $a3),
            Sysno::exit | Sysno::exit_group => {
                $self.exit_code = Some($a0);
                return Ok(StepResult::Halt);
            }
            Sysno::set_tid_address => $self.set_tid_address($mem, $a0),
            Sysno::futex => $self.futex($mem, $a0, $a1 as u32, $a2 as u32, $a3, $a4, $a5 as u32),
            Sysno::set_robust_list => $self.set_robust_list($mem, $a0, $a1),
            Sysno::tgkill => $self.tgkill($a0 as i32, $a1 as i32, $a2 as i32),
            Sysno::rt_sigaction => $self.rt_sigaction($mem, $a0, $a1, $a2, $a3),
            Sysno::rt_sigprocmask => $self.rt_sigprocmask($mem, $a0 as u32, $a1, $a2, $a3),
            Sysno::getpid => $self.getpid(),
            Sysno::gettid => $self.gettid(),
            Sysno::brk => $self.brk($mem, $a0),
            Sysno::mmap => $self.mmap($mem, $a0, $a1, $a2, $a3, $a4 as i32, $a5),
            Sysno::mprotect => $self.mprotect($mem, $a0, $a1, $a2),
            Sysno::riscv_hwprobe => $self.riscv_hwprobe($mem, $a0, $a1, $a2, $a3, $a4),
            Sysno::getrlimit => $self.getrlimit($mem, $a0 as u32, $a1),
            Sysno::getrandom => $self.getrandom($mem, $a0, $a1, $a2),
            Sysno::statx => $self.statx($mem, $a0 as i32, $a1, $a2, $a3, $a4),
            $($extra)*
            _ => {
                tracing::error!("SYSCALL({call}) unimplemented");
                Err(libc_riscv32::ENOSYS)
            }
        };

        let ret = ret.unwrap_or_else(|e| (-(e as i64)) as u64);
        $hart.set_reg(Reg::A0, <$X as Xlen>::from_u64(ret));

        Ok(StepResult::Ok)
    }};
}

impl Kernel for MockLinux<X32> {
    type Xlen = X32;
    type Error = LinuxError;
    type Memory = Memory32;

    fn syscall(
        &mut self,
        hart: &mut Hart<X32>,
        mem: &mut Memory32,
    ) -> Result<StepResult, MachineError<Self::Error>> {
        use syscalls::riscv32::Sysno;
        syscall_dispatch!(self, hart, mem, X32, [a0, a1, a2, a3, a4, a5, a7], {
            Sysno::ppoll_time64 => self.ppoll(mem, a0, a1, a2, a3, a4),
            Sysno::futex_time64 => self.futex(mem, a0, a1 as u32, a2 as u32, a3, a4, a5 as u32),
        })
    }
}

impl Kernel for MockLinux<X64> {
    type Xlen = X64;
    type Error = LinuxError;
    type Memory = Memory64;

    fn syscall(
        &mut self,
        hart: &mut Hart<X64>,
        mem: &mut Memory64,
    ) -> Result<StepResult, MachineError<Self::Error>> {
        use syscalls::riscv64::Sysno;
        syscall_dispatch!(self, hart, mem, X64, [a0, a1, a2, a3, a4, a5, a7], {
            Sysno::ppoll => self.ppoll(mem, a0, a1, a2, a3, a4),
        })
    }
}

impl<X: KernelXlen> MockLinux<X> {
    pub fn new(passthrough_stdio: bool) -> Self {
        Self {
            exit_code: None,
            passthrough_stdio,
            brk: 0,
            brk_limit: if X::MMAP_GROWS_DOWN {
                X::MMAP_BASE
            } else {
                X::STACK_TOP - STACK_RESERVE
            },
            mmap_cursor: X::MMAP_BASE,
            _xlen: PhantomData,
        }
    }

    pub fn exit_code(&self) -> Option<u64> {
        self.exit_code
    }

    pub fn load_static_elf<'a>(
        &mut self,
        hart: &mut Hart<X>,
        mem: &mut X::Memory,
        bytes: &'a [u8],
        args: &[&str],
        env: &[&str],
    ) -> Elf<'a> {
        let elf = Elf::parse(bytes).expect("Failed to parse ELF");

        // Load main program segments
        let mut brk = 0u64;
        for ph in &elf.program_headers {
            if ph.p_type == PT_LOAD {
                let vaddr = ph.p_vaddr;
                let end = vaddr + ph.p_memsz;
                mem.grow_to(end).expect("guest memory cap too small for ELF");
                let file = &bytes[ph.p_offset as usize..(ph.p_offset + ph.p_filesz) as usize];
                mem.copy_to(vaddr, file).expect("Failed to copy segment");
                // BSS is already zero: fresh arena pages are demand-zero.
                brk = brk.max(end);
            }
        }

        // PC
        hart.pc = X::from_u64(elf.entry);

        // Align and set brk
        self.brk = (brk + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        // Global pointer is at __DATA_BEGIN__
        // TODO: Do we actually need to set this? Or does libc initialize it on its own?
        let data_begin = elf
            .syms
            .iter()
            .find(|sym| elf.strtab.get_at(sym.st_name) == Some("__DATA_BEGIN__"))
            .map(|sym| sym.st_value)
            .unwrap_or(0);
        hart.set_reg(Reg::Gp, X::from_u64(data_begin));

        // Setup Stack.
        let align = std::mem::size_of::<X::U>() as u64;
        let mut sp = X::STACK_TOP.min(mem.max_addr() - PAGE_SIZE);
        mem.grow_to(sp + PAGE_SIZE).expect("guest memory cap too small for stack");
        let mut stack_init: Vec<X::U> = vec![];

        // Arguments
        stack_init.push(X::from_u64(args.len() as u64)); // argc
        for &arg in args.iter().rev() {
            let bytes = CString::new(arg).expect("argument contains null byte");
            let bytes = bytes.to_bytes_with_nul();

            sp = (sp - bytes.len() as u64) & !(align - 1);
            mem.copy_to(sp, bytes)
                .expect("Failed to copy argument to stack");

            stack_init.push(X::from_u64(sp)); // pointer to arg
        }
        stack_init.push(X::from_u64(0)); // argv NULL terminator

        // Environment
        for &e in env.iter().rev() {
            let bytes = CString::new(e).expect("environment variable contains null byte");
            let bytes = bytes.to_bytes_with_nul();

            sp = (sp - bytes.len() as u64) & !(align - 1);
            mem.copy_to(sp, bytes)
                .expect("Failed to copy environment variable to stack");

            stack_init.push(X::from_u64(sp));
        }
        stack_init.push(X::from_u64(0)); // envp NULL terminator

        // ELF Auxillary Vector
        macro_rules! set_AT {
            ($at:expr, $val:expr) => {
                stack_init.push(X::from_u64($at as u64));
                stack_init.push(X::from_u64($val));
            };
        }
        // TODO: Fill in the rest of the aux vector
        set_AT!(libc_riscv32::AT_PAGESZ, PAGE_SIZE);
        set_AT!(libc_riscv32::AT_CLKTCK, 100);
        set_AT!(libc_riscv32::AT_BASE, 0); // TODO
        set_AT!(libc_riscv32::AT_ENTRY, elf.entry);
        set_AT!(libc_riscv32::AT_NULL, 0);

        // Set up stack
        sp -= stack_init.len() as u64 * align;

        mem.copy_to(sp, &stack_init)
            .expect("Failed to copy stack data vector");

        hart.set_reg(Reg::Sp, X::from_u64(sp));

        tracing::debug!("Stack at {:#x}, GP at {:#x}", sp, data_begin);
        tracing::debug!("Loaded ELF. Start at {:08x}, brk={:08x}", elf.entry, self.brk);

        elf
    }
}
