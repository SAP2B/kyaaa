// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

use kyaaa::book;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Name(&'static str);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Title(&'static str);

#[derive(Copy, Clone)]
pub struct Game {
    title: Title,
    best_seller: bool,
}

#[derive(Copy, Clone)]
pub struct Boy {
    name: Name,
    dev: bool,
    age: u8,
}

fn main() {
    let hlist = book!(
        nier_automata => Game { title: Title("Nier Automata"), best_seller: true },
        mut zelda => Game { title: Title("Zelda"), best_seller: true },
        sap => Boy { name: Name("SAP2B"), age: 69, dev: true },
        mut john => Boy { name: Name("John Doe"), age: 15, dev: false },
        lockfree stats => Boy { name: Name("LockFree Boy"), age: 20, dev: true }
    );

    hlist.stats().write(Boy {
        name: Name("LockFree Boy Updated"),
        age: 21,
        dev: true,
    });

    unsafe {
        hlist.zelda().title = Title("Zelda: Breath of the Wild");
        hlist.zelda().best_seller = false;

        hlist.john().name = Name("John Wick");
        hlist.john().dev = true;
        hlist.john().age = 35;

        black_box(hlist.zelda());
        black_box(hlist.john());
    }

    black_box(hlist.stats().read());
    black_box(hlist.nier_automata());
    black_box(hlist.sap());
}

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}
