// This code is adapted from https://github.com/wenyuzhao/bump-allocator-rs
//
// We've altered it to work entirely with usize values internally and only cast
// back to an exposed-provenance pointer when returning from alloc. This gives
// us a richer checked-arithmetic API we can use to trap overflows internally,
// and also avoids some potential UB issues with pointer provenance. Since the
// provenance of __heap_base is a 1-byte value anyway, and all the rest of the
// wasm heap is considered to have exposed provenance, we think this is the best
// we can do. Writing allocators is tricky!
//
// NB: technically these alterations only handle corner cases that cannot be hit
// using safe client code. Safe clients pass `Layout` structs that always meet
// additional size and alignment constraints. But hardening the code to tolerate
// even _unsafe_ inputs -- malformed `Layout` inputs one can only create by
// calling unsafe methods -- is not only easy to do, it makes the code simpler
// and more readable, so we went ahead and did it.

use crate::unwrap::UnwrapOptimized;
use core::alloc::{GlobalAlloc, Layout};

static mut LOCAL_ALLOCATOR: BumpPointerLocal = BumpPointerLocal::new();

struct BumpPointerLocal {
    cursor: usize,
    limit: usize,
}

impl BumpPointerLocal {
    const LOG_PAGE_SIZE: usize = 16;
    const PAGE_SIZE: usize = 1 << Self::LOG_PAGE_SIZE; // 64KB
    const MEM: u32 = 0; // Memory 0 is the only legal one currently

    pub const fn new() -> Self {
        Self {
            cursor: 0,
            limit: 0,
        }
    }

    #[inline(always)]
    fn maybe_init_inline(&mut self) {
        if self.limit == 0 {
            // This is a slight over-estimate and ideally we would use __heap_base
            // but that seems not to be easy to access and in any case it is just a
            // convention whereas this is more guaranteed by the wasm spec to work.
            self.cursor = core::arch::wasm32::memory_size(Self::MEM)
                .checked_mul(Self::PAGE_SIZE)
                .unwrap_optimized();
            self.limit = self.cursor;
        }
    }

    #[inline(never)]
    fn maybe_init(&mut self) {
        self.maybe_init_inline()
    }

    // Allocate `bytes` bytes with `align` alignment.
    #[inline(always)]
    fn alloc(&mut self, bytes: usize, align: usize) -> usize {
        self.maybe_init();
        let start = self
            .cursor
            .checked_next_multiple_of(align)
            .unwrap_optimized();
        let new_cursor = start.checked_add(bytes).unwrap_optimized();
        if new_cursor <= self.limit {
            self.cursor = new_cursor;
            start
        } else {
            self.alloc_slow(bytes, align)
        }
    }

    #[inline(always)]
    fn alloc_slow_inline(&mut self, bytes: usize, align: usize) -> usize {
        let pages = bytes.div_ceil(Self::PAGE_SIZE);
        if core::arch::wasm32::memory_grow(Self::MEM, pages) == usize::MAX {
            core::arch::wasm32::unreachable();
        }
        let bytes_grown = pages.checked_mul(Self::PAGE_SIZE).unwrap_optimized();
        self.limit = self.limit.checked_add(bytes_grown).unwrap_optimized();
        self.alloc(bytes, align)
    }

    #[inline(never)]
    fn alloc_slow(&mut self, bytes: usize, align: usize) -> usize {
        self.alloc_slow_inline(bytes, align)
    }
}

pub struct BumpPointer;

unsafe impl GlobalAlloc for BumpPointer {
    #[inline(always)]
    #[allow(static_mut_refs)]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (bytes, align) = (layout.size(), layout.align());
        let ptr = LOCAL_ALLOCATOR.alloc(bytes, align);
        core::ptr::with_exposed_provenance_mut(ptr)
    }

    #[inline(always)]
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static GLOBAL: BumpPointer = BumpPointer;
