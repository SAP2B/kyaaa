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

dbook!(
    pub HLIST {
        nier: Game = Game {
            id: Str::from_str("023456789012345678901234"),
            name: Str::from_str("NieR Automata"),
        },
        sap: User = User {
            id: Str::from_str("223456789012345678901234"),
            name: Str::from_str("SAP2B HFT"),
        },
        admin: User = User {
            id: Str::from_str("423456789012345678901234"),
            name: Str::from_str("admin"),
        },
        zelda: Game = Game {
            id: Str::from_str("323456789012345678901234"),
            name: Str::from_str("LOL"),
        },
    }
);

static GAME_RING: Ring<(u8, u8), 256> = Ring::new();

kmain!(
    align(64);

    fn kmain(_sp: *const usize) -> i32 {
        let (mut producer, mut consumer) = GAME_RING.split();

        let raw_stream = &[0x00, 0x00];
        let stream = Stream::new(raw_stream);

        clock!({
            if let Some((DecodedToken::T8(_), _)) = stream.decode(0) {
                producer.push((HLIST.nier().id(), Tu8Book::hello().id()));
            }

            consumer.update(|entity_id, token_id| {
                let token_ref = Tu8Book.get(token_id).unwrap();
                HLIST.get_mut::<Game>(entity_id)
                     .unwrap()
                     .name(token_ref.text.as_str());
            });
        });

        black_box((
            HLIST.nier().get(),
            HLIST.sap().get(),
            HLIST.admin().get(),
            HLIST.zelda().get(),
        ));

        0
    }
);
