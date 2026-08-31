// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

kyaaa::page!(
    struct Game {
        name: kyaaa::Str<32>,
        favorite: bool,
    }
    struct User {
        name: kyaaa::Str<32>,
    }
);

fn main() {
    let hlist = kyaaa::book!(
        nier  => Game::new().name("Nier Automata").favorite(true),
        sap   => User::new().name("SAP2B"),
    );

    hlist.nier().set().name("NiER Automata").favorite(true);

    black_box(hlist.nier().get());
    black_box(hlist.sap().get());
}

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}
