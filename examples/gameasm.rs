kyaaa::page!(
    struct Game {
        name: &'static str,
        favorite: bool,
    }
    struct User {
        name: &'static str,
    }
);

fn main() {
    let hlist = kyaaa::book!(
        zelda => Game { name: "Zelda", favorite: true },
        nier => Game { name: "Nier Automata", favorite: true },
        lol => Game { name: "League of Legends", favorite: false },
        sap => User { name: "SAP2B" }
    );

    hlist.zelda().set().name("New Zelda");
    hlist.nier().set().name("New Nier Automata");
    hlist
        .lol()
        .update(|mut game| *game.name("cs2").favorite(true));

    let after_hlist = [hlist.zelda().get(), hlist.nier().get(), hlist.lol().get()];

    black_box(after_hlist);
    black_box(hlist.sap().get());
}

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}
