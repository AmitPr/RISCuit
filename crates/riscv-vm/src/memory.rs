mod _sealed {
    /// A trait to ensure that only Primitives can be loaded/stored.
    pub trait Primitive {}
    macro_rules! impl_primitive {
        ($($t:ty),*) => {$(impl Primitive for $t {})*}
    }
    impl_primitive!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
}
use _sealed::Primitive;

use crate::error::{MemoryAccess, MemoryError};

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
    pub mmap_top: u32,
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
            brk: 0,
            mmap_top: 0xC000_0000u32, // Start mmap at 3GB, downwards
        }
    }

    pub fn load<T: Primitive>(&self, addr: u32) -> T {
        // Safety: Primitive types are guaranteed not to overflow the address space,
        // where `addr + size_of::<T>() < MEMORY_SIZE` is guaranteed.
        unsafe { (self.ptr.add(addr as usize) as *const T).read_unaligned() }
    }

    pub fn store<T: Primitive>(&mut self, addr: u32, val: T) {
        // Safety: Primitive types are guaranteed not to overflow the address space,
        // where `addr + size_of::<T>() < MEMORY_SIZE` is guaranteed.
        unsafe { (self.ptr.add(addr as usize) as *mut T).write_unaligned(val) }
    }

    pub const fn ptr(&self, addr: u32) -> *const u8 {
        unsafe { self.ptr.add(addr as usize) }
    }

    pub const fn ptr_mut(&mut self, addr: u32) -> *mut u8 {
        unsafe { self.ptr.add(addr as usize) }
    }

    pub fn slice<T: Sized>(&self, addr: u32, len: u32) -> Result<&[T], MemoryError> {
        let _len_bytes = len
            .checked_mul(std::mem::size_of::<T>() as u32)
            .and_then(|l| l.checked_add(addr))
            .ok_or(MemoryError::OverflowMemoryAccess {
                access: MemoryAccess::Load,
                addr,
                len: len * std::mem::size_of::<T>() as u32,
            })?;
        Ok(unsafe { std::slice::from_raw_parts(self.ptr(addr) as *const T, len as usize) })
    }

    pub fn bytes_null_terminated(
        &self,
        addr: u32,
        max_len: Option<u32>,
    ) -> Result<&[u8], MemoryError> {
        let max_addr = max_len
            .map_or(Some(u32::MAX), |m| addr.checked_add(m))
            .ok_or(MemoryError::OverflowMemoryAccess {
                access: MemoryAccess::Load,
                addr,
                len: max_len.unwrap(),
            })?;

        let mut cur = addr;
        while cur < max_addr && self.load::<u8>(cur) != 0 {
            cur += 1;
        }

        self.slice(addr, cur - addr)
    }

    pub fn copy_to<T>(&mut self, dst: u32, src: &[T]) -> Result<(), MemoryError> {
        if dst.checked_add(std::mem::size_of_val(src) as u32).is_none() {
            Err(MemoryError::OverflowMemoryAccess {
                access: MemoryAccess::Store,
                addr: dst,
                len: std::mem::size_of_val(src) as u32,
            })
        } else {
            unsafe {
                std::ptr::copy_nonoverlapping(src.as_ptr(), self.ptr_mut(dst) as *mut T, src.len());
            }

            Ok(())
        }
    }

    pub fn memset(&mut self, addr: u32, val: u8, len: u32) -> Result<(), MemoryError> {
        if addr.checked_add(len).is_none() {
            Err(MemoryError::OverflowMemoryAccess {
                access: MemoryAccess::Store,
                addr,
                len,
            })
        } else {
            unsafe {
                std::ptr::write_bytes(self.ptr_mut(addr), val, len as usize);
            }
            Ok(())
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, MEMORY_SIZE);
        }
    }
}
