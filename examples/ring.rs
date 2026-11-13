#![no_main]
#![no_std]

use kyaaa::prelude::*;

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}

page!(
    align(64) struct Game {
        id: Str<64>,
        name: Str<64>,
    }

    align(64) struct User {
        id: Str<64>,
        name: Str<64>,
    }
);

book!(
    pub HLIST {
        zelda: Game = Game {
            id: Str::from_str("323456789012345678901234"),
            name: Str::from_str("LOL"),
        },
        admin: User = User {
            id: Str::from_str("423456789012345678901234"),
            name: Str::from_str("admin"),
        },
    }
);

static GAME_RING: Ring<u8, 256> = Ring::new();

kmain!(
    align(64);

    fn kmain(_sp: *const usize) -> i32 {
        clock!({
            let hlist = book!(
                nier: Game = Game {
                    id: Str::from_str("023456789012345678901234"),
                    name: Str::from_str("LOL"),
                },
                sap: User = User {
                    id: Str::from_str("223456789012345678901234"),
                    name: Str::from_str("SAP2B"),
                }
            );

            hlist.nier().set().id("1").name("NieR Automata");
            hlist.sap().set().name("SAP2B HFT");

            let (mut producer, mut consumer) = GAME_RING.split();

            producer.spin(|slot| {
                slot.write(hlist.sap().id());
            });

            consumer.eat(|&received| {
                if received == 1 {
                    HLIST.zelda().set().id("777").name("Zelda Updated");
                }
            });

            HLIST.admin().set().name("Admin HFT");

            black_box((
                hlist.nier().get(),
                hlist.sap().get(),
                HLIST.admin().get(),
                HLIST.zelda().get(),
            ));
        });

        0
    }
);
