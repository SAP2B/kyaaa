#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(target_arch = "wasm32")]
    core::arch::wasm32::unreachable();

    #[cfg(not(target_arch = "wasm32"))]
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn init() {}
