#![no_std]
#![no_main]

use core::arch::asm;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_str("Hello from no_std!\n\0");
    // test branching
    if pc() & 1 == 1 {
        print_str("PC is even\n\0");
    } else {
        print_str("PC is odd\n\0");
    }
    exit(0);
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    exit(1);
}

fn pc() -> u32 {
    let pc: u32;
    unsafe {
        asm!("auipc {}, 0", out(reg) pc);
    }
    pc
}

#[no_mangle]
fn ecall(a0: u32, a1: u32) {
    unsafe {
        asm!(
            "ecall",
            in("a0") a0,
            in("a1") a1,
            options(nostack),
        );
    }
}

#[no_mangle]
pub fn print_str(s: &str) {
    ecall(3, s.as_ptr() as u32);
}

#[no_mangle]
pub fn exit(code: i32) -> ! {
    ecall(93, code as u32);
    loop {}
}
