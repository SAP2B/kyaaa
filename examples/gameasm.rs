// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

use kyaaa::{Str, book, page};

page!(
    struct Game {
        name: Str<32>,
        favorite: bool,
    }
    struct User {
        name: Str<32>,
    }
);

fn main() {
    let hlist = book!(
        nier => Game {
            name: Str::from_str("Nier Automata"),
            favorite: true,
        },
        sap => User {
            name: Str::from_str("SAP2B"),
        },
    );

    hlist.nier().set().name("NiER Automata").favorite(true);

    black_box(hlist.nier().get());
    black_box(hlist.sap().get());
}

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}
