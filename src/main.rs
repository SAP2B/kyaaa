//PDX short identifier: AGPL-3.0
//Copyright (C) 2026 SAP2B

#![no_std]
#![no_main]

mod components;

use components::prelude::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() {}
