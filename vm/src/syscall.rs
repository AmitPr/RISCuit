use std::ffi::CStr;

use rand::RngCore;

use crate::{cpu::Hart32, memory::GuestPtr};

pub fn getrandom(_: &mut Hart32, mut buf: GuestPtr<[u8]>, len: u32, _flags: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.fill_bytes(buf.read_mut(len as usize));

    // "getrandom returns the number of bytes read on success."
    len
}

pub fn riscv_hwprobe(
    _hart: &mut Hart32,
    _pairs: GuestPtr<u8>,
    _pair_count: u32,
    _cpusetsize: u32,
    _cpus: GuestPtr<u8>,
    _flags: u32,
) -> i32 {
    // Return error code -- we don't implement this.
    -1
}

pub fn brk(hart: &mut Hart32, ptr: GuestPtr<u8>) -> u32 {
    if ptr.addr() > hart.mem.brk {
        hart.mem.brk = ptr.addr();
    }

    hart.mem.brk
}

pub fn gettid(_hart: &mut Hart32) -> u32 {
    // We don't implement this syscall.
    0x123
}

pub fn set_tid_address(hart: &mut Hart32, _tidptr: GuestPtr<u32>) -> u32 {
    // We don't implement this syscall.
    gettid(hart)
}

pub fn getpid(_hart: &mut Hart32) -> u32 {
    // We don't implement this syscall.
    1
}

pub fn set_robust_list(_hart: &mut Hart32, _head: GuestPtr<u8>, _len: u32) -> u32 {
    // We don't implement this syscall.
    0
}

#[repr(C)]
pub(crate) struct RLimit {
    rlim_cur: u64,
    rlim_max: u64,
}
pub fn getrlimit(_hart: &mut Hart32, resource: u32, rlim: GuestPtr<RLimit>) -> u32 {
    // We don't implement this syscall.
    match resource {
        // RLIMIT_STACK = 3, 8MB of stack
        3 => rlim.store(RLimit {
            rlim_cur: 0x800000,
            rlim_max: 0x800000,
        }),
        // For other resources, return "unlimited"
        _ => rlim.store(RLimit {
            rlim_cur: u64::MAX,
            rlim_max: u64::MAX,
        }),
    }

    0
}

pub fn readlinkat(
    _hart: &mut Hart32,
    _dirfd: i32,
    path: GuestPtr<CStr>,
    buf: GuestPtr<u8>,
    bufsiz: u32,
) -> i32 {
    // Read the pathname to see what's being requested
    let pathname = path.read();
    if pathname == c"/proc/self/exe" {
        // TODO: Handling /proc/self/exe specifically
        let fake_path = c"/tmp/main";
        let len = fake_path.count_bytes().min(bufsiz as usize);

        unsafe {
            std::ptr::copy_nonoverlapping(
                fake_path.as_ptr() as *const u8,
                buf.as_host_ptr_mut(),
                len,
            );
        }

        len as i32
    } else {
        // Return -1 for other paths
        -1
    }
}

pub fn write(_hart: &mut Hart32, fd: i32, buf: GuestPtr<[u8]>, count: u32) -> i32 {
    match fd {
        1 => {
            // Write to stdout
            let buf = buf.read(count as usize);
            let s = std::str::from_utf8(buf).unwrap();
            print!("{}", s);
            count as i32
        }
        2 => {
            // Write to stderr
            let buf = buf.read(count as usize);
            let s = std::str::from_utf8(buf).unwrap();
            eprint!("{}", s);
            count as i32
        }
        _ => -1,
    }
}

#[repr(C)]
pub(crate) struct Iovec {
    iov_base: *const u8,
    iov_len: u32,
}
pub fn writev(_hart: &mut Hart32, fd: i32, iov: GuestPtr<[Iovec]>, iovcnt: i32) -> i32 {
    let mut total = 0;
    for &Iovec { iov_base, iov_len } in iov.read(iovcnt as usize) {
        let ptr = GuestPtr::new(iov.base(), iov_base as u32);
        let count = write(_hart, fd, ptr, iov_len);
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
    uaddr: GuestPtr<i32>,
    op: i32,
    val: i32,
    _utime: GuestPtr<i32>,
    _uaddr2: GuestPtr<i32>,
    _val3: i32,
) -> i32 {
    match op & 0x7f {
        0 => {
            if uaddr.read() == val {
                0
            } else {
                -libc::EAGAIN
            }
        }
        1 => 1,
        _ => -libc::ENOSYS,
    }
}

pub fn mprotect(_hart: &mut Hart32, _addr: GuestPtr<u8>, _len: u32, _prot: u32) -> u32 {
    // TODO
    0
}

pub fn statx(
    _hart: &mut Hart32,
    _dirfd: i32,
    _pathname: GuestPtr<u8>,
    _flags: u32,
    _mask: u32,
    _statxbuf: GuestPtr<u8>,
) -> i32 {
    // stubbed out
    0
}

pub fn rt_sigaction(
    _hart: &mut Hart32,
    _signum: i32,
    _act: GuestPtr<u8>,
    _oldact: GuestPtr<u8>,
    _sigsetsize: u32,
) -> i32 {
    // stubbed out
    0
}

pub fn rt_sigprocmask(
    _hart: &mut Hart32,
    _how: i32,
    _set: GuestPtr<u8>,
    oldset: GuestPtr<u8>,
    sigsetsize: u32,
) -> i32 {
    if !oldset.is_null() {
        unsafe {
            std::ptr::write_bytes(oldset.as_host_ptr_mut(), 0, sigsetsize as usize);
        }
    }

    0
}

pub fn tgkill(_hart: &mut Hart32, _tgid: i32, _tid: i32, _sig: i32) -> i32 {
    // stubbed out
    0
}

pub fn ppoll(
    _hart: &mut Hart32,
    _fds: GuestPtr<u8>,
    _nfds: u32,
    _tsp: GuestPtr<u8>,
    _sigmask: GuestPtr<u8>,
    _sigsetsize: u32,
) -> i32 {
    // stubbed out
    0
}
