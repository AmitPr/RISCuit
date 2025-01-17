mod _sealed {
    /// A trait to ensure that only Primitives can be loaded/stored.
    pub trait Primitive {}
    macro_rules! impl_primitive {
        ($($t:ty),*) => {$(impl Primitive for $t {})*}
    }
    impl_primitive!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
}
use _sealed::Primitive;

pub const PAGE_SIZE: usize = 4096;
pub const MEMORY_SIZE: usize = const {
    // Assert that usize > u32.
    assert!(std::mem::size_of::<usize>() > std::mem::size_of::<u32>());
    u32::MAX as usize + PAGE_SIZE
};

/// A fully encompassing memory struct, using `mmap` to allocate
/// the full 32-bit (+1 page) address space.
///
/// Since we're only using rv32, we can safely use this implementation
/// without bounds-checking.
pub struct Memory {
    ptr: *mut u8,
    pub brk: u32,
}

impl Memory {
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
            brk: 0x1000000, // 16MB
        }
    }

    pub fn load<T: Primitive>(&self, addr: u32) -> T {
        unsafe { (self.ptr.add(addr as usize) as *const T).read_unaligned() }
    }

    pub fn store<T: Primitive>(&mut self, addr: u32, val: T) {
        unsafe {
            *(self.ptr.add(addr as usize) as *mut T) = val;
        }
    }

    pub fn pointer(&self, addr: u32) -> *mut u8 {
        unsafe { self.ptr.add(addr as usize) }
    }

    pub fn host_to_guest_ptr(&self, ptr: *const u8) -> u32 {
        ptr as u32 - self.ptr as u32
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, MEMORY_SIZE);
        }
    }
}
