use std::ffi::CStr;

use goblin::pe::import::Bitfield;
use rand::RngCore;

use crate::{cpu::Hart32, memory::GuestPtr};

pub fn ioctl(_hart: &mut Hart32, _fd: i32, _request: u32) -> i32 {
    // just return 0 for now
    0
}

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

pub fn brk(hart: &mut Hart32, addr: GuestPtr<u8>) -> u32 {
    let old_brk = hart.mem.brk;
    let new_brk = addr.addr();
    tracing::trace!("brk: new_brk={:#x} cur_brk={old_brk:#x}", addr.addr());

    // brk(0) is used to query the current break
    // brk returns the old program break on failure
    if new_brk == 0 {
        return old_brk;
    }

    if new_brk >= hart.mem.mmap_top {
        return old_brk;
    }

    hart.mem.brk = new_brk;
    // Zero memory
    let base = new_brk.min(old_brk);
    let region = hart.mem.pointer::<u8>(base).as_host_ptr_mut();
    let size = new_brk.abs_diff(old_brk) as usize;
    unsafe { std::ptr::write_bytes(region, 0, size) };

    // brk returns the new program break on success
    // new_brk
    0
}

pub fn gettid(_hart: &mut Hart32) -> u32 {
    // There's only one thread, so just return 1
    0x01
}

pub fn set_tid_address(hart: &mut Hart32, tidptr: GuestPtr<u32>) -> u32 {
    let tid = gettid(hart);
    tidptr.store(tid);

    tid
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
pub struct RLimit {
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
    let buf = buf.read(count as usize);
    // TODO: With tracing, should we still print to stdout/stderr?
    // Tracing is more for debugging/telemetry, not for user output.
    match fd {
        1 => {
            // Write to stdout
            print!("{}", std::str::from_utf8(buf).unwrap());
            count as i32
        }
        2 => {
            // Write to stderr
            eprint!("{}", std::str::from_utf8(buf).unwrap());
            count as i32
        }
        _ => -1,
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct iovec {
    iov_base: u32,
    iov_len: u32,
}
pub fn writev(hart: &mut Hart32, fd: i32, iov: GuestPtr<u8>, iovcnt: i32) -> i32 {
    if iovcnt == 0 {
        hart.running = false;
    }
    let mut total = 0;
    for i in 0..iovcnt {
        let iov_ptr = iov
            .offset(i * std::mem::size_of::<iovec>() as i32)
            .cast::<iovec>();
        let iov = iov_ptr.read();

        let data_ptr = hart.mem.pointer(iov.iov_base);
        let count = write(hart, fd, data_ptr, iov.iov_len as u32);
        if count < 0 {
            return count;
        }
        total += count as u32;
    }

    total as i32
}

pub fn exit(hart: &mut Hart32, status: i32) -> u32 {
    tracing::info!("Program exited with status {}", status);
    hart.running = false;
    hart.exit_code = status;

    status as u32
}

pub fn exit_group(hart: &mut Hart32, status: i32) -> u32 {
    exit(hart, status)
}

pub fn mmap(
    hart: &mut Hart32,
    addr: u32,
    len: u32,
    _prot: u32,
    _flags: u32,
    fd: i32,
    _offset: u32,
) -> u32 {
    tracing::trace!(
        "mmap: addr={addr:#x} len={len:#x} prot={_prot:#x} flags={_flags:#x} fd={fd} offset={_offset:#x}"
    );

    let size = (len + 0xFFF) & !0xFFF;
    if fd != -1 || size.is_zero() {
        tracing::warn!("mmap: invalid fd or size");
        return libc_riscv32::MAP_FAILED;
    }

    // Align up to page size
    let map_addr = if addr != 0 {
        // Align hint downwards
        addr & !0xFFF
    } else {
        // Use mmap_top, aligning downwards
        hart.mem.mmap_top.saturating_sub(size) & !0xFFF
    };

    if map_addr.checked_add(size).is_none() {
        tracing::warn!("mmap: address overflow");
        return libc_riscv32::MAP_FAILED;
    }

    if addr == 0 {
        hart.mem.mmap_top = map_addr;
    }

    // Zero out the region
    unsafe {
        std::ptr::write_bytes(
            hart.mem.pointer::<u8>(map_addr).as_host_ptr_mut(),
            0,
            size as usize,
        )
    }

    tracing::debug!("mmap: returning region at {map_addr:#x} of size {size:#x}");

    map_addr
}

#[allow(unused)]
pub fn munmap(hart: &mut Hart32, addr: u32, len: u32) -> u32 {
    todo!()
}

pub fn futex(
    hart: &mut Hart32,
    uaddr: GuestPtr<u32>,
    op: u32,
    val: u32,
    _utime: GuestPtr<i32>,
    _uaddr2: GuestPtr<i32>,
    mut val3: u32,
) -> i32 {
    let futex_word = uaddr.read();
    tracing::trace!(
        "futex: uaddr={:x}({futex_word:x}) op={op} val={val} utime={:x} uaddr2={:x} val3={val3:x}",
        uaddr.addr(),
        _utime.addr(),
        _uaddr2.addr(),
    );
    let cmd = op & libc_riscv32::FUTEX_CMD_MASK;
    if op & libc_riscv32::FUTEX_CLOCK_REALTIME != 0 {
        match cmd {
            libc_riscv32::FUTEX_WAIT_BITSET
            | libc_riscv32::FUTEX_WAIT_REQUEUE_PI
            | libc_riscv32::FUTEX_LOCK_PI2 => {}
            _ => {
                tracing::warn!("futex: invalid cmd");
                return -libc_riscv32::ENOSYS;
            }
        }
    }
    // First pass: set arguments
    match cmd {
        libc_riscv32::FUTEX_WAIT | libc_riscv32::FUTEX_WAKE => {
            val3 = libc_riscv32::FUTEX_BITSET_MATCH_ANY
        }
        _ => {}
    }

    fn futex_wait(hart: &mut Hart32, uaddr: GuestPtr<u32>, val: u32, val3: u32) -> i32 {
        let futex_word = uaddr.read();

        todo!()
    }
    fn futex_wake(hart: &mut Hart32, uaddr: GuestPtr<u32>, val: u32, val3: u32) -> i32 {
        todo!()
    }
    match cmd {
        libc_riscv32::FUTEX_WAIT => {
            // futex_wait(hart, uaddr, val, libc_riscv32::FUTEX_BITSET_MATCH_ANY)
            // immediately wake thread
            hart.mem.store::<u32>(uaddr.addr(), 0);
            0
        }
        libc_riscv32::FUTEX_WAIT_BITSET => {
            //futex_wait(hart, uaddr, val, val3),
            hart.mem.store::<u32>(uaddr.addr(), 0);
            0
        }
        libc_riscv32::FUTEX_WAKE => {
            // futex_wake(hart, uaddr, val, libc_riscv32::FUTEX_BITSET_MATCH_ANY)
            0
        }
        libc_riscv32::FUTEX_WAKE_BITSET => 0, //futex_wake(hart, uaddr, val, val3),
        _ => {
            tracing::warn!("futex: unhandled cmd");
            -libc_riscv32::ENOSYS
        }
    }
}

pub fn mprotect(_hart: &mut Hart32, _addr: GuestPtr<u8>, _len: u32, _prot: u32) -> i32 {
    // unimplemented
    -libc_riscv32::ENOSYS
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
    // stubbed out -- this is called w/ SIGPIPE during rust runtime startup
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

#[repr(C)]
#[derive(Debug)]
pub struct PollFd {
    fd: i32,
    events: i16,
    revents: i16,
}
pub fn ppoll(
    _hart: &mut Hart32,
    mut fds: GuestPtr<[PollFd]>,
    nfds: u32,
    _tsp: GuestPtr<u8>,
    _sigmask: GuestPtr<u8>,
    _sigsetsize: u32,
) -> i32 {
    // This method is called during CRT init for stdin/stdout/stderr
    // We can just return 0, as these file descriptors are always ready
    let fds = fds.read_mut(nfds as usize);
    for fd in fds {
        if fd.fd > 2 {
            // We don't support any other file descriptors
            return -libc_riscv32::ENOSYS;
        }
    }

    0
}
