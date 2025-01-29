use std::ffi::{CStr, CString};

use riscv_vm::memory::Memory;

use crate::MockLinux;

impl MockLinux {
    pub(crate) fn ioctl(&mut self, _fd: i32, _request: u32) -> Result<u32, i32> {
        // just return 0 for now
        Ok(0)
    }

    pub(crate) fn write(
        &mut self,
        mem: &Memory,
        fd: i32,
        buf: u32,
        count: u32,
    ) -> Result<u32, i32> {
        let slice = mem.slice(buf, count).map_err(|_| {
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
        mem: &Memory,
        fd: i32,
        iov: u32,
        iovcnt: i32,
    ) -> Result<u32, i32> {
        #[repr(C)]
        struct Iovec {
            iov_base: u32,
            iov_len: u32,
        }
        if iovcnt < 0 {
            return Err(libc_riscv32::EINVAL);
        }

        let iovs = mem
            .slice::<Iovec>(iov, iovcnt as u32)
            .map_err(|_| libc_riscv32::EFAULT)?;

        let total = iovs.iter().try_fold(0u32, |total, iov| {
            let count = self.write(mem, fd, iov.iov_base, iov.iov_len)?;
            total.checked_add(count).ok_or(libc_riscv32::EINVAL)
        })?;

        Ok(total)
    }

    pub(crate) fn readlinkat(
        &mut self,
        mem: &mut Memory,
        _dirfd: i32,
        pathname: u32,
        buf: u32,
        bufsiz: usize,
    ) -> Result<u32, i32> {
        let pathname = mem
            .bytes_null_terminated(pathname, None)
            .map_err(|_| libc_riscv32::EINVAL)?;
        let pathname = CStr::from_bytes_with_nul(pathname)
            .map(|s| s.to_str().unwrap_or(""))
            .map_err(|_| libc_riscv32::EINVAL)?;

        match pathname {
            "/proc/self/exe" => {
                let fake_path = "/tmp/main".as_bytes();

                let cstr = CString::new(fake_path).map_err(|_| libc_riscv32::EINVAL)?;
                let as_bytes = cstr.as_bytes_with_nul();
                // readlink doesn't necessarily NULL-terminate.
                let as_bytes: &[u8] = &as_bytes[..as_bytes.len().min(bufsiz)];

                mem.copy_to(buf, as_bytes)
                    .map_err(|_| libc_riscv32::EFAULT)?;

                // Always return the full length
                Ok(fake_path.len() as u32)
            }
            _ => Err(libc_riscv32::ENOENT),
        }
    }

    pub(crate) fn gettid(&mut self) -> Result<u32, i32> {
        Ok(0x1)
    }

    pub(crate) fn set_tid_address(&mut self, mem: &mut Memory, tidptr: u32) -> Result<u32, i32> {
        let tid = self.gettid()?;
        mem.store(tidptr, tid);

        Ok(tid)
    }

    pub(crate) fn getpid(&mut self) -> Result<u32, i32> {
        Ok(0x1)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn futex(
        &mut self,
        mem: &mut Memory,
        uaddr: u32,
        op: u32,
        val: u32,
        _utime: u32,
        _uaddr2: u32,
        val3: u32,
    ) -> Result<u32, i32> {
        let futex_word = mem.load::<u32>(uaddr);
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
        // First pass: set arguments
        // match cmd {
        //     libc_riscv32::FUTEX_WAIT | libc_riscv32::FUTEX_WAKE => {
        //         val3 = libc_riscv32::FUTEX_BITSET_MATCH_ANY
        //     }
        //     _ => {}
        // }
        match cmd {
            libc_riscv32::FUTEX_WAIT => {
                // TODO: Seems like there's an issue where futex is erroneously used
                // in the rust exit code, causing a deadlock... Temporary fix:
                mem.store::<u32>(uaddr, 0);
                Ok(0)
            }
            libc_riscv32::FUTEX_WAIT_BITSET => {
                // TODO: See above
                mem.store::<u32>(uaddr, 0);
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
        _mem: &mut Memory,
        _head: u32,
        _len: u32,
    ) -> Result<u32, i32> {
        Ok(0)
    }

    pub fn tgkill(&mut self, _tgid: i32, _pid: i32, _sig: i32) -> Result<u32, i32> {
        Ok(0)
    }

    pub fn rt_sigaction(
        &mut self,
        _mem: &Memory,
        _sig: u32,
        _act: u32,
        _oldact: u32,
        _sigsetsize: u32,
    ) -> Result<u32, i32> {
        // stubbed out -- this is called w/ SIGPIPE during rust runtime startup
        Ok(0)
    }

    pub fn rt_sigprocmask(
        &mut self,
        mem: &mut Memory,
        _how: u32,
        _set: u32,
        oldset: u32,
        sigsetsize: u32,
    ) -> Result<u32, i32> {
        if oldset != 0 {
            mem.memset(oldset, 0, sigsetsize)
                .map_err(|_| libc_riscv32::EFAULT)?;
        }

        Ok(0)
    }

    pub(crate) fn brk(&mut self, mem: &mut Memory, addr: u32) -> Result<u32, i32> {
        // TODO: Move brk / mmap_top to be managed by Kernel struct.
        // TODO: OOM detection/handling
        let old_brk = mem.brk;
        let new_brk = addr;
        tracing::trace!("brk: new_brk={:#x} cur_brk={old_brk:#x}", addr);

        // brk(0) is used to query the current break
        // brk returns the old program break on failure
        if new_brk == 0 {
            return Ok(old_brk);
        }

        if new_brk >= mem.mmap_top {
            return Ok(old_brk);
        }

        mem.brk = new_brk;
        // Zero memory
        let base = new_brk.min(old_brk);
        let size = new_brk.abs_diff(old_brk);
        mem.memset(base, 0, size).map_err(|_| {
            tracing::warn!("brk: failed to zero memory");
            libc_riscv32::ENOMEM
        })?;

        // brk returns the new program break on success
        Ok(new_brk)
        // Ok(0)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn mmap(
        &mut self,
        mem: &mut Memory,
        addr: u32,
        len: u32,
        _prot: u32,
        _flags: u32,
        fd: i32,
        _offset: u32,
    ) -> Result<u32, i32> {
        tracing::trace!(
            "mmap: addr={addr:#x} len={len:#x} prot={_prot:#x} flags={_flags:#x} fd={fd} offset={_offset:#x}"
        );

        let size = (len + 0xFFF) & !0xFFF;
        if fd != -1 || size == 0 {
            tracing::warn!("mmap: invalid fd or size");
            return Err(libc_riscv32::MAP_FAILED);
        }

        // Align up to page size
        let map_addr = if addr != 0 {
            // Align hint downwards
            addr & !0xFFF
        } else {
            // Use mmap_top, aligning downwards
            mem.mmap_top.saturating_sub(size) & !0xFFF
        };

        if map_addr.checked_add(size).is_none() {
            tracing::warn!("mmap: address overflow");
            return Err(libc_riscv32::MAP_FAILED);
        }

        if addr == 0 {
            mem.mmap_top = map_addr;
        }

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
        _mem: &Memory,
        _addr: u32,
        _len: u32,
        _prot: u32,
    ) -> Result<u32, i32> {
        Ok(0)
    }

    pub(crate) fn getrlimit(
        &mut self,
        mem: &mut Memory,
        resource: u32,
        rlim_ptr: u32,
    ) -> Result<u32, i32> {
        #[repr(C)]
        struct RLimit {
            rlim_cur: u32,
            rlim_max: u32,
        }

        let rlim = match resource {
            // RLIMIT_STACK = 3, 8MB of stack
            3 => RLimit {
                rlim_cur: 0x800000,
                rlim_max: 0x800000,
            },
            // For other resources, return "unlimited"
            _ => RLimit {
                rlim_cur: libc_riscv32::RLIM_INFINITY,
                rlim_max: libc_riscv32::RLIM_INFINITY,
            },
        };
        mem.copy_to(rlim_ptr, &[rlim])
            .map_err(|_| libc_riscv32::EFAULT)?;

        Ok(0)
    }

    pub(crate) fn riscv_hwprobe(
        &mut self,
        _mem: &Memory,
        _pairs: u32,
        _pair_count: u32,
        _cpusetsize: u32,
        _cpus: u32,
        _flags: u32,
    ) -> Result<u32, i32> {
        Err(libc_riscv32::ENOSYS)
    }

    pub(crate) fn getrandom(
        &mut self,
        mem: &mut Memory,
        buf: u32,
        len: u32,
        _flags: u32,
    ) -> Result<u32, i32> {
        // TODO: Stubbed to zero buffer
        mem.memset(buf, 0, len).map_err(|_| libc_riscv32::EFAULT)?;
        // let slice = mem
        //     .slice::<u8>(buf, len)
        //     .map_err(|_| libc_riscv32::EFAULT)?;

        // let mut rng = rand::thread_rng();
        // for byte in slice {
        //     *byte = rng.gen();
        // }

        Ok(len)
    }

    pub(crate) fn statx(
        &mut self,
        _mem: &Memory,
        _dirfd: i32,
        _pathname: u32,
        _flags: u32,
        _mask: u32,
        _statxbuf: u32,
    ) -> Result<u32, i32> {
        // stubbed out
        Ok(0)
    }

    pub(crate) fn ppoll_time64(
        &mut self,
        mem: &Memory,
        fds: u32,
        nfds: u32,
        _tsp: u32,
        _sigmask: u32,
        _sigsetsize: u32,
    ) -> Result<u32, i32> {
        #[repr(C)]
        struct PollFd {
            fd: i32,
            events: i16,
            revents: i16,
        }
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
