// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#![no_std]
#![no_main]

use core::arch::naked_asm;
use kyaaa::*;

// Define cache-aligned memory layouts (64-byte alignment to prevent false sharing across cores)
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
    // Direct kernel exit on panic in a bare-metal environment
    Syscall::exit(1).expect("Exit error");
    loop {}
}

// Custom raw entry point bypassing standard runtime initialization
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    naked_asm!(
        "xor rbp, rbp",
        "mov rdi, rsp",
        "and rsp, -64", // Align stack to 64 bytes for vector/SIMD safety
        "call {main}",
        "mov rdi, rax",
        "mov rax, 231", // Syscall number for exit
        "syscall",
        "ud2",
        main = sym k_main,
    );
}

// Global compile-time static book instance mapped directly in the data segment
book! {
    pub static HLIST;
    zelda => Game {
        id: Str::from_str("323456789012345678901234"),
        name: Str::from_str("LOL"),
    },
    re9 => Game {
        id: Str::from_str("523456789012345678901234"),
        name: Str::from_str("Requiem"),
    },
    player => User {
        id: Str::from_str("623456789012345678901234"),
        name: Str::from_str("Player"),
    },
    admin => User {
        id: Str::from_str("423456789012345678901234"),
        name: Str::from_str("admin"),
    },
}

#[unsafe(no_mangle)]
pub extern "C" fn k_main(_sp: *const usize) -> i32 {
    // Local expression-based book instance allocated on the stack
    let hlist = book!(
        nier => Game {
            id: Str::from_str("023456789012345678901234"),
            name: Str::from_str("LOL"),
        },
        mario => Game {
            id: Str::from_str("023456789012345678901234"),
            name: Str::from_str("Mario"),
        },
        sap => User {
            id: Str::from_str("223456789012345678901234"),
            name: Str::from_str("SAP2B"),
        },
        admin => User {
            id: Str::from_str("423456789012345678901234"),
            name: Str::from_str("admin"),
        },
    );

    // Zero-cost field mutation via generated reference wrappers
    hlist.nier().set().name("NieR Automata");
    hlist.sap().set().name("SAP2B HFT");
    HLIST.zelda().set().name("Zelda");
    HLIST.admin().set().name("Admin HFT");

    // Type-safe dynamic element retrieval using ID lookups
    if let Some(_nier) = hlist.get::<Game>(hlist.nier().id()) {}
    if let Some(_zelda) = HLIST.get::<Game>(HLIST.zelda().id()) {}

    // Type mismatch check: results in None safely because hlist.sap().id() belongs to a User, not a Game
    if let Some(_nier) = hlist.get::<Game>(hlist.sap().id()) {}

    // Zero-overhead filtered iterations over specific concrete types
    hlist.list::<User>().for_each(|user| {
        let User { name: _, id: _ } = user;
    });

    HLIST.list::<User>().for_each(|user| {
        let User { name: _, id: _ } = user;
    });

    0
}
