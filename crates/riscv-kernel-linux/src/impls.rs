//! Syscall implementations, generic over register width.
//!
//! Handlers take `u64`-normalized arguments (zero-extended from `X::U` at
//! dispatch) and return `Result<u64, errno>`; the dispatcher sign-truncates
//! errno returns back to width. ABI structs that contain pointers/longs are
//! parameterized by `X::U`.
use std::ffi::CString;

use riscv_vm::memory::{Memory, Pod};

use crate::{KernelXlen, MockLinux, PAGE_SIZE};

#[repr(C)]
#[derive(Clone, Copy)]
struct IoVec<U> {
    base: U,
    len: U,
}
// Safety: width-sized integers only; any bit pattern valid, no padding.
unsafe impl<U: Pod> Pod for IoVec<U> {}

#[repr(C)]
#[derive(Clone, Copy)]
struct RLimit<U> {
    rlim_cur: U,
    rlim_max: U,
}
// Safety: as above.
unsafe impl<U: Pod> Pod for RLimit<U> {}

#[repr(C)]
#[derive(Clone, Copy)]
struct PollFd {
    fd: i32,
    events: i16,
    revents: i16,
}
// Safety: integers only; any bit pattern valid, no padding.
unsafe impl Pod for PollFd {}

impl<X: KernelXlen> MockLinux<X> {
    pub(crate) fn ioctl(&mut self, _fd: i32, _request: u64) -> Result<u64, i32> {
        // just return 0 for now
        Ok(0)
    }

    pub(crate) fn readlinkat(
        &mut self,
        mem: &mut X::Memory,
        _dirfd: i32,
        pathname: u64,
        buf: u64,
        bufsiz: u64,
    ) -> Result<u64, i32> {
        let pathname = mem
            .bytes_null_terminated(pathname, None)
            .map_err(|_| libc_riscv32::EINVAL)?;
        let pathname = std::str::from_utf8(pathname).map_err(|_| libc_riscv32::EINVAL)?;

        match pathname {
            "/proc/self/exe" => {
                let fake_path = "/tmp/main".as_bytes();

                let cstr = CString::new(fake_path).map_err(|_| libc_riscv32::EINVAL)?;
                let as_bytes = cstr.as_bytes_with_nul();
                // readlink doesn't necessarily NULL-terminate.
                let as_bytes: &[u8] = &as_bytes[..as_bytes.len().min(bufsiz as usize)];

                mem.copy_to(buf, as_bytes)
                    .map_err(|_| libc_riscv32::EFAULT)?;

                // Always return the full length
                Ok(fake_path.len() as u64)
            }
            _ => Err(libc_riscv32::ENOENT),
        }
    }

    pub(crate) fn write(
        &mut self,
        mem: &X::Memory,
        fd: i32,
        buf: u64,
        count: u64,
    ) -> Result<u64, i32> {
        let slice = mem.slice::<u8>(buf, count).map_err(|_| {
            tracing::warn!("write: buffer read would overflow guest memory");
            libc_riscv32::EFAULT
        })?;

        match fd {
            1 => {
                if self.passthrough_stdio {
                    print!("{}", String::from_utf8_lossy(slice)); // stdout
                }
                Ok(count)
            }
            2 => {
                if self.passthrough_stdio {
                    eprint!("{}", String::from_utf8_lossy(slice)); // stderr
                }
                Ok(count)
            }
            _ => {
                tracing::warn!("write: fd {fd} not supported");
                Err(libc_riscv32::EBADF)
            }
        }
    }

    pub(crate) fn writev(
        &mut self,
        mem: &X::Memory,
        fd: i32,
        iov: u64,
        iovcnt: i32,
    ) -> Result<u64, i32> {
        if iovcnt < 0 {
            return Err(libc_riscv32::EINVAL);
        }

        let iovs = mem
            .slice::<IoVec<X::U>>(iov, iovcnt as u64)
            .map_err(|_| libc_riscv32::EFAULT)?;

        let total = iovs.iter().try_fold(0u64, |total, iov| {
            let count = self.write(mem, fd, X::to_u64(iov.base), X::to_u64(iov.len))?;
            total.checked_add(count).ok_or(libc_riscv32::EINVAL)
        })?;

        Ok(total)
    }

    pub(crate) fn gettid(&mut self) -> Result<u64, i32> {
        Ok(0x1)
    }

    pub(crate) fn set_tid_address(&mut self, mem: &mut X::Memory, tidptr: u64) -> Result<u64, i32> {
        let tid = self.gettid()?;
        mem.store_at::<u32>(tidptr, tid as u32)
            .map_err(|_| libc_riscv32::EFAULT)?;

        Ok(tid)
    }

    pub(crate) fn getpid(&mut self) -> Result<u64, i32> {
        Ok(0x1)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn futex(
        &mut self,
        mem: &mut X::Memory,
        uaddr: u64,
        op: u32,
        val: u32,
        _utime: u64,
        _uaddr2: u64,
        val3: u32,
    ) -> Result<u64, i32> {
        let futex_word = mem
            .load_at::<u32>(uaddr)
            .map_err(|_| libc_riscv32::EFAULT)?;
        tracing::trace!(
        "futex: uaddr={:x}({futex_word:x}) op={op} val={val} utime={:x} uaddr2={:x} val3={val3:x}",
        uaddr,
        _utime,
        _uaddr2,
    );
        let cmd = op & libc_riscv32::FUTEX_CMD_MASK;
        if op & libc_riscv32::FUTEX_CLOCK_REALTIME != 0 {
            match cmd {
                libc_riscv32::FUTEX_WAIT_BITSET
                | libc_riscv32::FUTEX_WAIT_REQUEUE_PI
                | libc_riscv32::FUTEX_LOCK_PI2 => {}
                _ => {
                    tracing::warn!("futex: invalid cmd");
                    return Err(libc_riscv32::ENOSYS);
                }
            }
        }
        match cmd {
            libc_riscv32::FUTEX_WAIT => {
                // TODO: Seems like there's an issue where futex is erroneously used
                // in the rust exit code, causing a deadlock... Temporary fix:
                mem.store_at::<u32>(uaddr, 0)
                    .map_err(|_| libc_riscv32::EFAULT)?;
                Ok(0)
            }
            libc_riscv32::FUTEX_WAIT_BITSET => {
                // TODO: See above
                mem.store_at::<u32>(uaddr, 0)
                    .map_err(|_| libc_riscv32::EFAULT)?;
                Ok(0)
            }
            // TODO: No-op right now.
            libc_riscv32::FUTEX_WAKE => Ok(0),
            libc_riscv32::FUTEX_WAKE_BITSET => Ok(0),
            _ => {
                tracing::warn!("futex: unhandled cmd");
                Err(libc_riscv32::ENOSYS)
            }
        }
    }

    pub(crate) fn set_robust_list(
        &mut self,
        _mem: &mut X::Memory,
        _head: u64,
        _len: u64,
    ) -> Result<u64, i32> {
        Ok(0)
    }

    pub(crate) fn tgkill(&mut self, _tgid: i32, _pid: i32, _sig: i32) -> Result<u64, i32> {
        Ok(0)
    }

    pub(crate) fn rt_sigaction(
        &mut self,
        _mem: &X::Memory,
        _sig: u64,
        _act: u64,
        _oldact: u64,
        _sigsetsize: u64,
    ) -> Result<u64, i32> {
        // stubbed out -- this is called w/ SIGPIPE during rust runtime startup
        Ok(0)
    }

    pub(crate) fn rt_sigprocmask(
        &mut self,
        mem: &mut X::Memory,
        _how: u32,
        _set: u64,
        oldset: u64,
        sigsetsize: u64,
    ) -> Result<u64, i32> {
        if oldset != 0 {
            mem.memset(oldset, 0, sigsetsize)
                .map_err(|_| libc_riscv32::EFAULT)?;
        }

        Ok(0)
    }

    pub(crate) fn brk(&mut self, mem: &mut X::Memory, addr: u64) -> Result<u64, i32> {
        // TODO: OOM detection/handling
        let old_brk = self.brk;
        let new_brk = addr;
        tracing::trace!("brk: new_brk={:#x} cur_brk={old_brk:#x}", addr);

        // brk(0) is used to query the current break
        // brk returns the old program break on failure
        if new_brk == 0 {
            return Ok(old_brk);
        }

        // brk may not grow into the mmap region (rv32 layout) or the stack
        // reservation (rv64 layout), nor shrink below the loaded image.
        let limit = if X::MMAP_GROWS_DOWN {
            self.mmap_cursor.min(self.brk_limit)
        } else {
            self.brk_limit
        };
        if new_brk >= limit || new_brk < self.brk_floor {
            return Ok(old_brk);
        }

        if mem.grow_to(new_brk).is_err() {
            return Ok(old_brk);
        }

        self.brk = new_brk;
        // Zero memory
        let base = new_brk.min(old_brk);
        let size = new_brk.abs_diff(old_brk);
        mem.memset(base, 0, size).map_err(|_| {
            tracing::warn!("brk: failed to zero memory");
            libc_riscv32::ENOMEM
        })?;

        // brk returns the new program break on success
        Ok(new_brk)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn mmap(
        &mut self,
        mem: &mut X::Memory,
        addr: u64,
        len: u64,
        _prot: u64,
        _flags: u64,
        fd: i32,
        _offset: u64,
    ) -> Result<u64, i32> {
        tracing::trace!(
            "mmap: addr={addr:#x} len={len:#x} prot={_prot:#x} flags={_flags:#x} fd={fd} offset={_offset:#x}"
        );

        let Some(size) = len.checked_add(PAGE_SIZE - 1).map(|l| l & !(PAGE_SIZE - 1)) else {
            return Err(libc_riscv32::ENOMEM);
        };
        if fd != -1 {
            tracing::warn!("mmap: file mappings not supported");
            return Err(libc_riscv32::ENOSYS);
        }
        if size == 0 {
            return Err(libc_riscv32::EINVAL);
        }

        // Placement hints are advisory (POSIX): allocation always comes from
        // the cursor, so hinted requests can never overlap the bump region.
        let map_addr = if X::MMAP_GROWS_DOWN {
            self.mmap_cursor.saturating_sub(size) & !(PAGE_SIZE - 1)
        } else {
            self.mmap_cursor
        };

        let Some(end) = map_addr.checked_add(size) else {
            tracing::warn!("mmap: address overflow");
            return Err(libc_riscv32::ENOMEM);
        };

        // The grows-down region bottoms out at the program break.
        if X::MMAP_GROWS_DOWN && map_addr < self.brk {
            tracing::warn!("mmap: region collided with brk");
            return Err(libc_riscv32::ENOMEM);
        }

        if mem.grow_to(end).is_err() {
            tracing::warn!("mmap: out of guest address space");
            return Err(libc_riscv32::ENOMEM);
        }

        self.mmap_cursor = if X::MMAP_GROWS_DOWN { map_addr } else { end };

        // Zero out the region
        mem.memset(map_addr, 0, size).map_err(|_| {
            tracing::warn!("mmap: failed to zero memory");
            libc_riscv32::ENOMEM
        })?;

        tracing::debug!("mmap: returning region at {map_addr:#x} of size {size:#x}");

        Ok(map_addr)
    }

    pub(crate) fn mprotect(
        &mut self,
        _mem: &X::Memory,
        _addr: u64,
        _len: u64,
        _prot: u64,
    ) -> Result<u64, i32> {
        Ok(0)
    }

    pub(crate) fn getrlimit(
        &mut self,
        mem: &mut X::Memory,
        resource: u32,
        rlim_ptr: u64,
    ) -> Result<u64, i32> {
        // All-ones at width == RLIM_INFINITY.
        let infinity = X::from_i64(-1);
        let rlim = match resource {
            // RLIMIT_STACK = 3, 8MB of stack
            3 => RLimit {
                rlim_cur: X::from_u64(crate::STACK_RESERVE),
                rlim_max: X::from_u64(crate::STACK_RESERVE),
            },
            // For other resources, return "unlimited"
            _ => RLimit {
                rlim_cur: infinity,
                rlim_max: infinity,
            },
        };
        mem.copy_to(rlim_ptr, &[rlim])
            .map_err(|_| libc_riscv32::EFAULT)?;

        Ok(0)
    }

    pub(crate) fn riscv_hwprobe(
        &mut self,
        _mem: &X::Memory,
        _pairs: u64,
        _pair_count: u64,
        _cpusetsize: u64,
        _cpus: u64,
        _flags: u64,
    ) -> Result<u64, i32> {
        Err(libc_riscv32::ENOSYS)
    }

    pub(crate) fn getrandom(
        &mut self,
        mem: &mut X::Memory,
        buf: u64,
        len: u64,
        _flags: u64,
    ) -> Result<u64, i32> {
        // TODO: Stubbed to zero buffer
        mem.memset(buf, 0, len).map_err(|_| libc_riscv32::EFAULT)?;

        Ok(len)
    }

    pub(crate) fn statx(
        &mut self,
        _mem: &X::Memory,
        _dirfd: i32,
        _pathname: u64,
        _flags: u64,
        _mask: u64,
        _statxbuf: u64,
    ) -> Result<u64, i32> {
        // stubbed out
        Ok(0)
    }

    pub(crate) fn ppoll(
        &mut self,
        mem: &X::Memory,
        fds: u64,
        nfds: u64,
        _tsp: u64,
        _sigmask: u64,
        _sigsetsize: u64,
    ) -> Result<u64, i32> {
        // This method is called during CRT init for stdin/out/err
        let fds = mem
            .slice::<PollFd>(fds, nfds)
            .map_err(|_| libc_riscv32::EFAULT)?;
        for fd in fds {
            if fd.fd > 2 {
                // We don't support any other file descriptors
                return Err(libc_riscv32::ENOSYS);
            }
        }

        // We can just return 0, as stdin/out/err are always ready
        Ok(0)
    }
}
