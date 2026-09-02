// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

use colorful::{Colorful, HSL};
use kyaaa::Str;

kyaaa::page!(
    struct Game {
        name: Str<32>,
        favorite: bool,
    }
    struct User {
        name: Str<32>,
    }
);

trait Hsl: Colorful + Sized {
    fn print_hsl(self) {
        let msg = self.gradient_with_color(HSL::new(0.0, 1.0, 0.5), HSL::new(0.833, 1.0, 0.5));
        println!("{msg}");
    }
}

impl Hsl for String {}
impl Hsl for &str {}

fn main() {
    let hlist = kyaaa::book!(
        zelda => Game {
            name: Str::from_str("Zelda"),
            favorite: true,
        },
        nier => Game {
            name: Str::from_str("Nier Automata"),
            favorite: true,
        },
        lol => Game {
            name: Str::from_str("League of Legends"),
            favorite: false,
        },
        sap => User {
            name: Str::from_str("SAP2B"),
        }
    );

    let before_hlist = [hlist.zelda().get(), hlist.nier().get(), hlist.lol().get()];
    let sap = hlist.sap().get();

    format!("Before Book: {hlist:?} -> {before_hlist:#?}, {sap:#?}").print_hsl();

    hlist.zelda().set().name("New Zelda").favorite(true);
    hlist.nier().set().name("New Nier Automata").favorite(true);
    hlist.lol().set().name("cs2").favorite(true);

    let after_hlist = [hlist.zelda().get(), hlist.nier().get(), hlist.lol().get()];
    format!("After Book: {hlist:?} -> {after_hlist:#?}, {sap:#?}").print_hsl();
}
