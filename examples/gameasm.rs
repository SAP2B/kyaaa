// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#![no_main]
#![no_std]

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

book!(
    pub static HLIST;
    zelda => Game {
        id: Str::from_str("323456789012345678901234"),
        name: Str::from_str("LOL"),
    },
    admin => User {
        id: Str::from_str("423456789012345678901234"),
        name: Str::from_str("admin"),
    },
);

kmain!(
    align(64);

    fn kmain(_sp: *const usize) -> i32 {
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
);

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}
