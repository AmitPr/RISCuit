pub const PAGE_SIZE: usize = 4096;
pub const MEMORY_SIZE: usize = const {
    // Assert that usize > u32.
    assert!(u32::MAX as usize + PAGE_SIZE > u32::MAX as usize);
    u32::MAX as usize + PAGE_SIZE
};

/// A fully encompassing memory struct, using `mmap` to allocate
/// the full 32-bit (+1 page) address space.
///
/// Since we're only using rv32, we can safely use this implementation
/// without bounds-checking.
pub struct Memory64 {
    ptr: *mut u8,
}

impl Memory64 {
    pub fn new() -> Self {
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                MEMORY_SIZE,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            )
        };

        if ptr == libc::MAP_FAILED {
            panic!("Failed to allocate memory");
        }

        Self {
            ptr: ptr as *mut u8,
        }
    }

    pub fn load<T>(&self, addr: u64) -> T
    where
        T: Copy,
    {
        unsafe { (self.ptr.add(addr as usize) as *const T).read_unaligned() }
    }

    pub fn load_dword(&self, addr: u64) -> u64 {
        self.load(addr)
    }

    pub fn load_word(&self, addr: u64) -> u32 {
        self.load(addr)
    }

    pub fn load_half(&self, addr: u64) -> u16 {
        self.load(addr)
    }

    pub fn load_byte(&self, addr: u64) -> u8 {
        self.load(addr)
    }

    pub fn store<T>(&mut self, addr: u64, val: T)
    where
        T: Copy,
    {
        unsafe {
            *(self.ptr.add(addr as usize) as *mut T) = val;
        }
    }

    pub fn store_dword(&mut self, addr: u64, val: u64) {
        self.store(addr, val);
    }

    pub fn store_word(&mut self, addr: u64, val: u32) {
        self.store(addr, val);
    }

    pub fn store_half(&mut self, addr: u64, val: u16) {
        self.store(addr, val);
    }

    pub fn store_byte(&mut self, addr: u64, val: u8) {
        self.store(addr, val);
    }
}

impl Drop for Memory64 {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, MEMORY_SIZE);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory() {
        let memory = Memory64::new();
        assert!(!memory.ptr.is_null());
    }
}
