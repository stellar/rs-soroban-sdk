// provide bits of rust's runtime interface: allocator, panic handling, etc.

#[cfg(target_family = "wasm")]
#[inline(always)]
pub fn trap() -> ! {
    core::arch::wasm32::unreachable()
}

#[cfg(not(target_family = "wasm"))]
pub fn trap() -> ! {
    panic!()
}

#[cfg(target_family = "wasm")]
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    trap();
}
