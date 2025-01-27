use std::ffi::CStr;

use riscv_inst::Reg;

use crate::cpu::Hart32;

const PAGE_SIZE: u32 = 4096;

pub fn setup_stack(cpu: &mut Hart32, args: &[&CStr], env: &[&CStr]) {
    let mut sp = cpu.get_reg(Reg::Sp);
    let mut stack_init: Vec<u32> = vec![];

    // Arguments
    stack_init.push(args.len() as u32); // argc
    for &arg in args.iter().rev() {
        let bytes = arg.to_bytes_with_nul();
        let len = bytes.len();
        sp = (sp - len as u32) & !(4 - 1); // align to 4 bytes
        let dst = cpu.mem.pointer::<u8>(sp);
        unsafe {
            dst.as_host_ptr_mut()
                .copy_from_nonoverlapping(bytes.as_ptr(), len);
        }
        stack_init.push(dst.addr()); // pointer to arg
    }
    stack_init.push(0); // argv NULL terminator

    // Environment
    for &e in env.iter().rev() {
        let bytes = e.to_bytes_with_nul();
        let len = bytes.len();
        sp = (sp - len as u32) & !(4 - 1); // align to 4 bytes
        let dst = cpu.mem.pointer::<u8>(sp);
        unsafe {
            dst.as_host_ptr_mut()
                .copy_from_nonoverlapping(bytes.as_ptr(), len);
        }
        stack_init.push(dst.addr());
    }
    stack_init.push(0); // envp NULL terminator

    // ELF Auxillary Vector
    macro_rules! set_AT {
        ($at:expr, $val:expr) => {
            stack_init.push($at as u32);
            stack_init.push($val);
        };
    }
    set_AT!(libc_riscv32::AT_PAGESZ, PAGE_SIZE);
    set_AT!(libc_riscv32::AT_CLKTCK, 100);

    // TODO: Fill in the rest of the aux vector
    set_AT!(libc_riscv32::AT_PHDR, 0);
    set_AT!(libc_riscv32::AT_PHENT, 0);
    set_AT!(libc_riscv32::AT_PHNUM, 0);

    set_AT!(libc_riscv32::AT_BASE, 0); // TODO
    set_AT!(libc_riscv32::AT_ENTRY, cpu.pc);

    set_AT!(libc_riscv32::AT_HWCAP, 0);
    set_AT!(libc_riscv32::AT_HWCAP2, 0);
    set_AT!(libc_riscv32::AT_HWCAP3, 0);
    set_AT!(libc_riscv32::AT_HWCAP4, 0);
    set_AT!(libc_riscv32::AT_UID, 0);
    set_AT!(libc_riscv32::AT_EUID, 0);
    set_AT!(libc_riscv32::AT_GID, 0);
    set_AT!(libc_riscv32::AT_EGID, 0);
    set_AT!(libc_riscv32::AT_SECURE, 0);

    set_AT!(libc_riscv32::AT_NULL, 0);

    // Set up stack init vector
    let stack_init_ptr = sp - (stack_init.len() as u32 * 4);
    for (i, &val) in stack_init.iter().enumerate() {
        cpu.mem.store::<u32>(stack_init_ptr + (i as u32 * 4), val);
    }

    // Set up stack pointer
    sp -= stack_init.len() as u32 * 4;
    cpu.set_reg(Reg::Sp, sp);
    tracing::debug!("Stack at {:#x}", sp);
}
