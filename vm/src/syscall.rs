use rand::RngCore;

use crate::cpu::Hart32;

pub fn getrandom(_: &mut Hart32, buf: *mut u8, len: u32, _flags: u32) -> u32 {
    let buf_as_slice = unsafe { std::slice::from_raw_parts_mut(buf, len as usize) };
    let mut rng = rand::thread_rng();
    rng.fill_bytes(buf_as_slice);

    // "getrandom returns the number of bytes read on success."
    len
}

pub fn riscv_hwprobe(
    _hart: &mut Hart32,
    _pairs: *mut u8,
    _pair_count: u32,
    _cpusetsize: u32,
    _cpus: *mut u8,
    _flags: u32,
) -> i32 {
    // Return error code -- we don't implement this.
    -1
}

pub fn brk(hart: &mut Hart32, addr: *mut u8) -> u32 {
    let guest = hart.mem.host_to_guest_ptr(addr);
    if guest > hart.mem.brk {
        hart.mem.brk = guest;
    }

    hart.mem.brk
}

pub fn set_tid_address(_hart: &mut Hart32, _tidptr: *mut u32) -> u32 {
    // We don't implement this syscall.
    0x123
}

pub fn set_robust_list(_hart: &mut Hart32, _head: *mut u8, _len: u32) -> u32 {
    // We don't implement this syscall.
    0
}

pub fn getrlimit(_hart: &mut Hart32, resource: u32, rlim: *mut u8) -> u32 {
    #[repr(C)]
    struct RLimit {
        rlim_cur: u64,
        rlim_max: u64,
    }
    let rlim = rlim as *mut RLimit;
    // We don't implement this syscall.
    match resource {
        // RLIMIT_STACK = 3
        3 => {
            unsafe {
                rlim.write_unaligned(RLimit {
                    rlim_cur: 0x800000, // 8MB
                    rlim_max: 0x800000, // 8MB
                });
            };
        }
        _ => {
            // For other resources, return "unlimited"
            unsafe {
                rlim.write_unaligned(RLimit {
                    rlim_cur: u64::MAX,
                    rlim_max: u64::MAX,
                });
            };
        }
    }

    0
}

pub fn readlinkat(
    _hart: &mut Hart32,
    _dirfd: i32,
    path: *const u8,
    buf: *mut u8,
    bufsiz: u32,
) -> i32 {
    // Read the pathname to see what's being requested
    let pathname = unsafe { std::ffi::CStr::from_ptr(path as *const i8) };
    if pathname == c"/proc/self/exe" {
        // TODO: Handling /proc/self/exe specifically
        let fake_path = c"/tmp/main";
        let len = fake_path.count_bytes().min(bufsiz as usize);

        unsafe {
            std::ptr::copy_nonoverlapping(fake_path.as_ptr() as *const u8, buf, len);
        }

        len as i32
    } else {
        // Return -1 for other paths
        -1
    }
}

pub fn write(_hart: &mut Hart32, fd: i32, buf: *const u8, count: u32) -> i32 {
    match fd {
        1 => {
            // Write to stdout
            let buf = unsafe { std::slice::from_raw_parts(buf, count as usize) };
            let s = std::str::from_utf8(buf).unwrap();
            print!("{}", s);
            count as i32
        }
        2 => {
            // Write to stderr
            let buf = unsafe { std::slice::from_raw_parts(buf, count as usize) };
            let s = std::str::from_utf8(buf).unwrap();
            eprint!("{}", s);
            count as i32
        }
        _ => -1,
    }
}

pub fn writev(_hart: &mut Hart32, fd: i32, iov: *const u8, iovcnt: i32) -> i32 {
    #[repr(C)]
    struct Iovec {
        iov_base: *const u8,
        iov_len: u32,
    }

    let iov = unsafe { std::slice::from_raw_parts(iov as *const Iovec, iovcnt as usize) };
    let mut total = 0;
    for iovec in iov {
        let count = write(_hart, fd, iovec.iov_base, iovec.iov_len);
        if count < 0 {
            return count;
        }
        total += count as u32;
    }

    total as i32
}

pub fn exit(hart: &mut Hart32, status: i32) -> u32 {
    println!("Program exited with status {}", status);
    hart.running = false;

    status as u32
}

pub fn exit_group(hart: &mut Hart32, status: i32) -> u32 {
    exit(hart, status)
}

pub fn mmap(
    hart: &mut Hart32,
    _addr: u32,
    len: u32,
    prot: i32,
    flags: i32,
    _fd: i32,
    _offset: u32,
) -> i32 {
    // ensure only MAP_ANONYMOUS calls are made
    if flags & libc::MAP_ANONYMOUS == 0 {
        return -libc::EINVAL;
    }

    // ensure only PROT_READ and PROT_WRITE are set
    if prot & !(libc::PROT_READ | libc::PROT_WRITE) != 0 {
        return -libc::EINVAL;
    }

    // just give it some memory from the Memory
    let ptr = hart.mem.brk;
    hart.mem.brk += len;

    ptr as i32
}

#[allow(unused)]
pub fn munmap(hart: &mut Hart32, addr: u32, len: u32) -> u32 {
    todo!()
}

pub fn futex(
    _hart: &mut Hart32,
    uaddr: *mut i32,
    op: i32,
    val: i32,
    _utime: *const i32,
    _uaddr2: *mut i32,
    _val3: i32,
) -> i32 {
    match op & 0x7f {
        0 => {
            let current = unsafe { *uaddr };
            if current == val {
                0
            } else {
                -libc::EAGAIN
            }
        }
        1 => 1,
        _ => -libc::ENOSYS,
    }
}

pub fn mprotect(_hart: &mut Hart32, _addr: *mut u8, _len: u32, _prot: u32) -> u32 {
    // TODO
    0
}

pub fn statx(
    _hart: &mut Hart32,
    _dirfd: i32,
    _pathname: *const u8,
    _flags: u32,
    _mask: u32,
    _statxbuf: *mut u8,
) -> i32 {
    // stubbed out
    0
}
