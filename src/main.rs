#![no_std] // don't link std library.
#![no_main] // disable all Rust-level entry points.

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // disable name mangling to make sure the func name after compiled remains exactly `_start`.
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default.
    loop {}
}
