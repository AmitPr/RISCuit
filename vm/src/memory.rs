mod _sealed {
    /// A trait to ensure that only Primitives can be loaded/stored.
    pub trait Primitive {}
    macro_rules! impl_primitive {
        ($($t:ty),*) => {$(impl Primitive for $t {})*}
    }
    impl_primitive!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
}
use std::{ffi::CStr, marker::PhantomData};

use _sealed::Primitive;
use goblin::pe::import::Bitfield;

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

    pub fn pointer<T: ?Sized>(&self, addr: u32) -> GuestPtr<T> {
        GuestPtr::new(self.ptr, addr)
    }

    pub fn host_to_guest_ptr<T>(&self, ptr: *const u8) -> Option<GuestPtr<T>> {
        let offset = (ptr as usize)
            .checked_sub(self.ptr as usize)?
            .try_into()
            .ok()?;

        Some(GuestPtr::new(self.ptr, offset))
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, MEMORY_SIZE);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuestPtr<T: ?Sized> {
    /// Pointer to the base of the guest memory.
    base: *const u8,
    /// Offset
    offset: u32,
    _phantom: PhantomData<T>,
}

impl<T: ?Sized> GuestPtr<T> {
    pub fn new(base: *const u8, offset: u32) -> Self {
        Self {
            base,
            offset,
            _phantom: PhantomData,
        }
    }

    pub fn is_null(&self) -> bool {
        self.offset.is_zero()
    }

    pub fn offset(&self, offset: i32) -> Self {
        Self {
            base: self.base,
            offset: self.offset.wrapping_add_signed(offset),
            _phantom: PhantomData,
        }
    }

    pub fn as_host_ptr(&self) -> *const u8 {
        unsafe { self.base.add(self.offset as usize) }
    }

    pub fn as_host_ptr_mut(&self) -> *mut u8 {
        unsafe { self.base.add(self.offset as usize) as *mut u8 }
    }

    pub fn addr(&self) -> u32 {
        self.offset
    }

    pub fn base(&self) -> *const u8 {
        self.base
    }
}

impl<T: Sized> GuestPtr<T> {
    pub fn read(&self) -> T {
        let ptr = self.as_host_ptr() as *const T;
        unsafe { ptr.read_unaligned() }
    }

    pub fn store(&self, val: T) {
        let ptr = self.as_host_ptr_mut() as *mut T;
        unsafe { ptr.write_unaligned(val) }
    }

    pub fn as_typed_host_ptr(&self) -> *const T {
        self.as_host_ptr() as *const T
    }
}

impl GuestPtr<CStr> {
    pub fn read(&self) -> &CStr {
        // TODO: Prevent DoS / slowdowns from reading very large CStr
        unsafe { CStr::from_ptr(self.as_host_ptr() as *const i8) }
    }
}

impl<T> GuestPtr<[T]> {
    pub fn read(&self, len: usize) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.as_host_ptr() as *const T, len) }
    }

    pub fn read_mut(&mut self, len: usize) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.as_host_ptr_mut() as *mut T, len) }
    }
}
