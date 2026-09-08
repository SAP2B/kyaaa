// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#![no_std]
#![no_main]

use core::arch::naked_asm;
use kyaaa::*;

page!(
    align(64) struct Game {
        id: Str<32>,
        name: Str<32>,
    }

    align(64) struct User {
        id: Str<32>,
        name: Str<32>,
    }
);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    Syscall::exit(1).expect("Exit error");
    loop {}
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    naked_asm!(
        "xor rbp, rbp",
        "mov rdi, rsp",
        "and rsp, -64",
        "call {main}",
        "mov rdi, rax",
        "mov rax, 231",
        "syscall",
        "ud2",
        main = sym k_main,
    );
}

book! {
    pub static HLIST;
    zelda => Game {
        id: Str::from_str("323456789012345678901234"),
        name: Str::from_str("LOL"),
    },
    admin => User {
        id: Str::from_str("423456789012345678901234"),
        name: Str::from_str("admin"),
    },
}

#[unsafe(no_mangle)]
pub extern "C" fn k_main(_sp: *const usize) -> i32 {
    let hlist = book!(
        nier => Game {
            id: Str::from_str("023456789012345678901234"),
            name: Str::from_str("LOL"),
        },
        sap => User {
            id: Str::from_str("223456789012345678901234"),
            name: Str::from_str("SAP2B"),
        },
    );

    hlist.nier().set().name("NieR Automata");
    hlist.sap().set().name("SAP2B HFT");
    HLIST.zelda().set().name("Zelda");
    HLIST.admin().set().name("Admin HFT");

    black_box(hlist.nier().get());
    black_box(hlist.sap().get());
    black_box(HLIST.admin().get());
    black_box(HLIST.zelda().get());

    0
}

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}
