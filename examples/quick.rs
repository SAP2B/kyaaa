// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#![no_std]
#![no_main]

use core::arch::naked_asm;
use kyaaa::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    Syscall::exit(1);
    loop {}
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    naked_asm!(
        "xor rbp, rbp",
        "mov rdi, rsp",
        "and rsp, -16",
        "call {main}",
        "mov rdi, rax",
        "mov rax, 231",
        "syscall",
        "ud2",
        main = sym k_main,
    );
}

page!(
 align(32)
    struct Order {
        symbol: Str<16>,
        price: u64,
        volume: u64,
    }
);

extern "C" fn k_main(_sp: *const usize) -> i32 {
    let hlist = book!(
        order_sol => Order {
            symbol: Str::from_str("SOLANA"),
            price: 525000,
            volume: 1000,
        },
        order_btc => Order {
            symbol: Str::from_str("Bitcoin"),
            price: 3850,
            volume: 50000,
        },
        order_eth => Order {
            symbol: Str::from_str("Etherum"),
            price: 6120,
            volume: 200000,
        },
    );

    hlist.order_sol().set().volume(15);

    black_box(hlist.order_sol().get());
    black_box(hlist.order_eth().get());
    black_box(hlist.order_btc().get());
    0
}

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}
