#![no_std]

#[cfg(test)]
#[macro_use]
extern crate alloc;

extern crate spin;

/// A number of WebAssembly memory pages.
#[derive(Eq, PartialEq)]
struct PageCount(usize);

impl PageCount {
    fn size_in_bytes(self) -> usize {
        self.0 * PAGE_SIZE
    }
}

/// The WebAssembly page size, in bytes.
const PAGE_SIZE: usize = 65536;

/// Invalid number of pages used to indicate out of memory errors.
const ERROR_PAGE_COUNT: PageCount = PageCount(usize::MAX);

/// Wrapper for core::arch::wasm::memory_grow.
/// Adding this level of indirection allows for improved testing,
/// especially on non wasm platforms.
trait MemoryGrower {
    /// See core::arch::wasm::memory_grow for semantics.
    fn memory_grow(&self, delta: PageCount) -> PageCount;
}

pub struct DefaultGrower;

impl MemoryGrower for DefaultGrower {
    #[cfg(target_arch = "wasm32")]
    fn memory_grow(&self, delta: PageCount) -> PageCount {
        // This should use `core::arch::wasm` instead of `core::arch::wasm32`,
        // but `core::arch::wasm` depends on `#![feature(simd_wasm64)]` on current nightly.
        // See https://github.com/Craig-Macomber/lol_alloc/issues/1
        PageCount(core::arch::wasm32::memory_grow(0, delta.0))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn memory_grow(&self, _delta: PageCount) -> PageCount {
        // This MemoryGrower is not actually supported on non-wasm targets.
        // Just return an out of memory error:
        ERROR_PAGE_COUNT
    }
}

mod free_list_allocator;
mod locked_allocator;
mod trivial_allocators;
pub use crate::free_list_allocator::FreeListAllocator;
pub use crate::locked_allocator::LockedAllocator;
pub use crate::trivial_allocators::{FailAllocator, LeakingAllocator, LeakingPageAllocator};
