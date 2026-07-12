mod _sealed {
    /// A trait to ensure that only Primitives can be loaded/stored.
    pub trait Primitive {}
    macro_rules! impl_primitive {
        ($($t:ty),*) => {$(impl Primitive for $t {})*}
    }
    impl_primitive!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
}
pub use _sealed::Primitive;

/// Types that may be viewed over raw guest bytes: every bit pattern must be
/// a valid value and the type must be padding-free.
///
/// # Safety
/// Implementing this for a type with invalid bit patterns (`bool`, most
/// enums, references) or padding makes [`Memory::slice`]/[`Memory::copy_to`]
/// unsound.
pub unsafe trait Pod: Copy {}
unsafe impl<T: Primitive + Copy> Pod for T {}

use std::marker::PhantomData;

use crate::error::{MemoryAccess, MemoryError};

pub const PAGE_SIZE: usize = 4096;

const fn page_align_up(v: u64) -> u64 {
    v.saturating_add(PAGE_SIZE as u64 - 1) & !(PAGE_SIZE as u64 - 1)
}

#[cold]
#[inline(never)]
fn fault(access: MemoryAccess, addr: u64) -> MemoryError {
    MemoryError::Fault { access, addr }
}

/// A guest access outside the mapped window.
///
/// Zero-sized: the faulting address and access kind are already known at
/// every call site, so the hot path never materializes a diagnostic --
/// callers attach context when propagating (see `MemoryError::Fault`).
#[derive(Debug, Clone, Copy)]
pub struct Fault;

/// A by-value snapshot of an arena's access window (base pointer + mapped
/// top) for a run of hot accesses.
///
/// Accesses through `&self` methods reload both fields per access: guest
/// stores go through a raw pointer the compiler cannot prove disjoint from
/// the arena struct itself. A `Copy` snapshot lives in registers instead.
///
/// A view mutably borrows the arena for `'m` (compare
/// [`std::cell::Cell::from_mut`]): anything that could invalidate the
/// window -- `grow_to` (which may move the arena), any other `&mut` use
/// such as a kernel call, forming references over guest bytes, or taking
/// another view -- cannot compile while a view is live, so a view never
/// dangles. The view itself is a shared, `Copy` handle, which is sound
/// because its accesses are raw-pointer reads and writes (free to alias)
/// and it never hands out references; exclusivity is spent once, in
/// [`Memory::view`]'s `&mut self`. The phantom only ties `'m` to the
/// type, hence `&'m ()`. Run loops release the borrow across kernel calls
/// and re-take it after.
#[derive(Clone, Copy)]
pub struct MemView<'m> {
    ptr: *mut u8,
    /// Exclusive access limit; accesses at or beyond it fault.
    mapped: u64,
    _mem: PhantomData<&'m ()>,
}

impl MemView<'_> {
    #[inline(always)]
    pub fn load<T: Primitive>(self, addr: u64) -> Result<T, Fault> {
        if addr >= self.mapped {
            return Err(Fault);
        }
        // Safety: addr < mapped, and a primitive's overhang past `mapped`
        // lands in the guard page.
        Ok(unsafe { (self.ptr.add(addr as usize) as *const T).read_unaligned() })
    }

    #[inline(always)]
    pub fn store<T: Primitive>(self, addr: u64, val: T) -> Result<(), Fault> {
        if addr >= self.mapped {
            return Err(Fault);
        }
        // Safety: see `load`.
        unsafe { (self.ptr.add(addr as usize) as *mut T).write_unaligned(val) };
        Ok(())
    }
}

/// A guest physical memory arena.
///
/// `Addr` is the guest address width. The hot `load`/`store` paths take it
/// exactly (no conversions); the bulk helpers used by kernels take `u64`.
pub trait Memory {
    type Addr: Copy + Into<u64>;

    fn load<T: Primitive>(&self, addr: Self::Addr) -> Result<T, Fault>;
    fn store<T: Primitive>(&mut self, addr: Self::Addr, val: T) -> Result<(), Fault>;

    /// Snapshot the current access window; the view holds the `&mut`
    /// borrow (see [`MemView`]).
    fn view(&mut self) -> MemView<'_>;

    /// Configured cap on guest addresses (exclusive). Growth beyond this
    /// always fails.
    fn max_addr(&self) -> u64;

    /// Make `[0, top)` accessible, growing the arena if needed.
    fn grow_to(&mut self, top: u64) -> Result<(), MemoryError>;

    /// Host pointer to guest range `[addr, addr + len)`, if fully accessible.
    fn ptr_range(&self, access: MemoryAccess, addr: u64, len: u64) -> Result<*mut u8, MemoryError>;

    // --- Bulk helpers (kernel-facing, cold relative to load/store) ---

    /// Load at a `u64` address, whatever the width of `Addr`.
    fn load_at<T: Primitive>(&self, addr: u64) -> Result<T, MemoryError> {
        let ptr = self.ptr_range(MemoryAccess::Load, addr, std::mem::size_of::<T>() as u64)?;
        Ok(unsafe { (ptr as *const T).read_unaligned() })
    }

    /// Store at a `u64` address, whatever the width of `Addr`.
    fn store_at<T: Primitive>(&mut self, addr: u64, val: T) -> Result<(), MemoryError> {
        let ptr = self.ptr_range(MemoryAccess::Store, addr, std::mem::size_of::<T>() as u64)?;
        unsafe { (ptr as *mut T).write_unaligned(val) };
        Ok(())
    }

    /// View guest memory as a slice of `T`. Errors if the guest address is
    /// not aligned for `T` on the host.
    fn slice<T: Pod>(&self, addr: u64, len: u64) -> Result<&[T], MemoryError> {
        let bytes = len.checked_mul(std::mem::size_of::<T>() as u64).ok_or(
            MemoryError::OverflowMemoryAccess {
                access: MemoryAccess::Load,
                addr,
                len: len.min(u32::MAX as u64) as u32,
            },
        )?;
        let ptr = self.ptr_range(MemoryAccess::Load, addr, bytes)?;
        if ptr as usize % std::mem::align_of::<T>() != 0 {
            return Err(MemoryError::UnalignedMemoryAccess {
                access: MemoryAccess::Load,
                addr,
                required: std::mem::align_of::<T>() as u32,
            });
        }
        Ok(unsafe { std::slice::from_raw_parts(ptr as *const T, len as usize) })
    }

    fn copy_to<T: Pod>(&mut self, dst: u64, src: &[T]) -> Result<(), MemoryError> {
        let bytes = std::mem::size_of_val(src);
        let ptr = self.ptr_range(MemoryAccess::Store, dst, bytes as u64)?;
        // Byte-wise copy: the destination has no alignment guarantee.
        unsafe { std::ptr::copy_nonoverlapping(src.as_ptr() as *const u8, ptr, bytes) };
        Ok(())
    }

    fn memset(&mut self, addr: u64, val: u8, len: u64) -> Result<(), MemoryError> {
        let ptr = self.ptr_range(MemoryAccess::Store, addr, len)?;
        unsafe { std::ptr::write_bytes(ptr, val, len as usize) };
        Ok(())
    }

    /// Bytes starting at `addr` up to (not including) the first NUL, bounded
    /// by `max_len` when given.
    fn bytes_null_terminated(&self, addr: u64, max_len: Option<u64>) -> Result<&[u8], MemoryError> {
        let max_addr = max_len
            .map_or(Some(self.max_addr()), |m| addr.checked_add(m))
            .ok_or_else(|| MemoryError::OverflowMemoryAccess {
                access: MemoryAccess::Load,
                addr,
                len: max_len.unwrap_or(u64::MAX).min(u32::MAX as u64) as u32,
            })?;

        let mut cur = addr;
        while cur < max_addr {
            let b: u8 = {
                let ptr = self.ptr_range(MemoryAccess::Load, cur, 1)?;
                unsafe { *ptr }
            };
            if b == 0 {
                break;
            }
            cur += 1;
        }

        self.slice(addr, cur - addr)
    }
}

/// rv32 arena: the full 32-bit address space (+1 guard page) is mapped up
/// front, so any `u32` address is in-bounds by construction and the hot
/// paths compile to a single unchecked access.
pub struct Memory32 {
    ptr: *mut u8,
}

pub const MEMORY32_SIZE: usize = const {
    // The scheme requires a host with a wider address space than the guest.
    assert!(std::mem::size_of::<usize>() > std::mem::size_of::<u32>());
    u32::MAX as usize + PAGE_SIZE
};

impl Memory for Memory32 {
    type Addr = u32;

    #[inline(always)]
    fn load<T: Primitive>(&self, addr: u32) -> Result<T, Fault> {
        // Safety: every 32-bit address (plus a primitive's overhang into the
        // guard page) is mapped.
        Ok(unsafe { (self.ptr.add(addr as usize) as *const T).read_unaligned() })
    }

    #[inline(always)]
    fn store<T: Primitive>(&mut self, addr: u32, val: T) -> Result<(), Fault> {
        // Safety: see `load`.
        unsafe { (self.ptr.add(addr as usize) as *mut T).write_unaligned(val) };
        Ok(())
    }

    #[inline(always)]
    fn view(&mut self) -> MemView<'_> {
        // `mapped` is a constant the width of the guest space: after
        // inlining, every `u32`-derived address compares below it and the
        // view's limit check folds away, keeping rv32 accesses unchecked.
        MemView {
            ptr: self.ptr,
            mapped: 1 << 32,
            _mem: PhantomData,
        }
    }

    fn max_addr(&self) -> u64 {
        1 << 32
    }

    fn grow_to(&mut self, top: u64) -> Result<(), MemoryError> {
        if top <= 1 << 32 {
            Ok(())
        } else {
            Err(fault(MemoryAccess::Store, top))
        }
    }

    fn ptr_range(&self, access: MemoryAccess, addr: u64, len: u64) -> Result<*mut u8, MemoryError> {
        if addr.checked_add(len).is_none_or(|end| end > 1 << 32) {
            return Err(fault(access, addr));
        }
        Ok(unsafe { self.ptr.add(addr as usize) })
    }
}

impl Memory32 {
    pub fn new() -> Self {
        Self {
            ptr: map_anon(MEMORY32_SIZE),
        }
    }
}

impl Default for Memory32 {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Memory32 {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.ptr as *mut _, MEMORY32_SIZE) };
    }
}

/// rv64 arena: an elastic window.
///
/// A guest can't be given the full 64-bit space, but it doesn't need it: the
/// kernel controls the entire guest layout (ELF placement, brk, mmap, stack),
/// so it only ever hands out addresses below a configured cap. The arena
/// starts small and grows by `mremap` as the kernel raises the top; growth is
/// O(page tables), guest addresses never move (only the host base may), and
/// an unmapped window keeps host virtual address usage proportional to what
/// the guest actually uses -- which is what lets thousands of harts share one
/// host.
///
/// Every access is limit-checked against the mapped top. The check is a
/// never-taken predictable branch (measured at parity with the unchecked
/// rv32 path) and gives precise faults for wild pointers.
pub struct Memory64 {
    ptr: *mut u8,
    /// Bytes currently mapped (excluding the guard page). Accesses at or
    /// beyond this fault.
    mapped: u64,
    /// Configured cap on the guest address space (exclusive).
    max: u64,
}

/// Default initial window: 32 MiB (grown on demand).
pub const MEMORY64_DEFAULT_INITIAL: u64 = 32 << 20;
/// Default guest address-space cap: 64 GiB.
pub const MEMORY64_DEFAULT_MAX: u64 = 64 << 30;

impl Memory for Memory64 {
    type Addr = u64;

    #[inline(always)]
    fn load<T: Primitive>(&self, addr: u64) -> Result<T, Fault> {
        if addr >= self.mapped {
            return Err(Fault);
        }
        // Safety: addr < mapped, and a primitive's overhang past `mapped`
        // lands in the guard page.
        Ok(unsafe { (self.ptr.add(addr as usize) as *const T).read_unaligned() })
    }

    #[inline(always)]
    fn store<T: Primitive>(&mut self, addr: u64, val: T) -> Result<(), Fault> {
        if addr >= self.mapped {
            return Err(Fault);
        }
        // Safety: see `load`.
        unsafe { (self.ptr.add(addr as usize) as *mut T).write_unaligned(val) };
        Ok(())
    }

    #[inline(always)]
    fn view(&mut self) -> MemView<'_> {
        MemView {
            ptr: self.ptr,
            mapped: self.mapped,
            _mem: PhantomData,
        }
    }

    fn max_addr(&self) -> u64 {
        self.max
    }

    fn grow_to(&mut self, top: u64) -> Result<(), MemoryError> {
        if top <= self.mapped {
            return Ok(());
        }
        if top > self.max {
            return Err(fault(MemoryAccess::Store, top));
        }
        // Grow geometrically to amortize mremap calls, capped at max.
        let new_mapped = page_align_up(top.max(self.mapped * 2).min(self.max));
        let new_ptr = unsafe {
            libc::mremap(
                self.ptr as *mut _,
                self.mapped as usize + PAGE_SIZE,
                new_mapped as usize + PAGE_SIZE,
                libc::MREMAP_MAYMOVE,
            )
        };
        if new_ptr == libc::MAP_FAILED {
            return Err(fault(MemoryAccess::Store, top));
        }
        self.ptr = new_ptr as *mut u8;
        self.mapped = new_mapped;
        Ok(())
    }

    fn ptr_range(&self, access: MemoryAccess, addr: u64, len: u64) -> Result<*mut u8, MemoryError> {
        if addr.checked_add(len).is_none_or(|end| end > self.mapped) {
            return Err(fault(access, addr));
        }
        Ok(unsafe { self.ptr.add(addr as usize) })
    }
}

impl Memory64 {
    pub fn new(initial: u64, max: u64) -> Self {
        const {
            assert!(std::mem::size_of::<usize>() == 8);
        }
        let max = page_align_up(max);
        let mapped = page_align_up(initial).min(max);
        Self {
            ptr: map_anon(mapped as usize + PAGE_SIZE),
            mapped,
            max,
        }
    }
}

impl Default for Memory64 {
    fn default() -> Self {
        Self::new(MEMORY64_DEFAULT_INITIAL, MEMORY64_DEFAULT_MAX)
    }
}

impl Drop for Memory64 {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.ptr as *mut _, self.mapped as usize + PAGE_SIZE) };
    }
}

fn map_anon(len: usize) -> *mut u8 {
    let ptr = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE,
            -1,
            0,
        )
    };
    if ptr == libc::MAP_FAILED {
        panic!("failed to map {len:#x} bytes of guest memory");
    }
    ptr as *mut u8
}
