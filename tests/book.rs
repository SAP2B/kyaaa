// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

kyaaa::page!(
    struct Game {
        name: &'static str,
        favorite: bool,
    }
    struct User {
        name: &'static str,
    }
);

#[test]
fn book_hlist() {
    let hlist = kyaaa::book!(
        zelda => Game { name: "Zelda", favorite: true },
        nier => Game { name: "Nier Automata", favorite: true },
        lol => Game { name: "League of Legends", favorite: false },
        sap => User { name: "SAP2B" }
    );

    assert_eq!(hlist.list::<Game>().count(), 3);
    assert!(hlist.list::<User>().eq([hlist.sap().get()]));
    assert_eq!(hlist.list::<User>().next().unwrap(), hlist.sap().get());
    assert_eq!(hlist.zelda().get(), hlist.get::<Game>(0).unwrap());
    assert_eq!(hlist.nier().get(), hlist.get::<Game>(1).unwrap());
    assert_eq!(hlist.lol().get(), hlist.get::<Game>(2).unwrap());
    assert_eq!(hlist.sap().get(), hlist.get::<User>(3).unwrap());

    hlist.zelda().set().name = "New Zelda";
    hlist.nier().set().name = "New Nier Automata";
    hlist.sap().set().name = "New SAP";

    hlist.lol().update(|mut game| {
        game.name = "cs2";
        game.favorite = true;
        game
    });

    assert_eq!(
        hlist.get::<Game>(hlist.zelda().id()).unwrap(),
        hlist.get::<Game>(0).unwrap()
    );
    assert_eq!(hlist.zelda().get().name, "New Zelda");
    assert_eq!(hlist.zelda().get(), hlist.get::<Game>(0).unwrap());
    assert_eq!(hlist.nier().get().name, "New Nier Automata");
    assert_eq!(hlist.nier().get(), hlist.get::<Game>(1).unwrap());
    assert_eq!(hlist.lol().get().name, "cs2");
    assert!(hlist.lol().get().favorite);
    assert_eq!(hlist.lol().get(), hlist.get::<Game>(2).unwrap());
    assert_eq!(hlist.sap().get().name, "New SAP");
    assert_eq!(hlist.sap().get(), hlist.get::<User>(3).unwrap());
}

#[test]
fn book_out_of_bounds() {
    let hlist = kyaaa::book!(
        item1 => Game { name: "Item1", favorite: true }
    );

    assert!(hlist.get::<Game>(0).is_some());
    assert!(hlist.get::<Game>(1).is_none());
    assert!(hlist.get_mut::<Game>(99).is_none());
}

kyaaa::page! {
    struct Player {
        id: u64,
        score: u32,
    }
}

#[test]
fn book_with_page_types() {
    let hlist = kyaaa::book!(
        p1 => Player { id: 1, score: 100 },
        p2 => Player { id: 2, score: 200 }
    );

    assert_eq!(hlist.p1().get().id, 1);
    assert_eq!(hlist.p2().get().score, 200);

    hlist.p1().set().score(150);
    assert_eq!(hlist.p1().get().score, 150);

    let bytes = hlist.p1().get().to_bytes();
    let (decoded, _) = Player::from_bytes(bytes).unwrap();
    assert_eq!(decoded.score, 150);
}

#[test]
fn book_heterogeneous_indices() {
    let hlist = kyaaa::book!(
        g1 => Game { name: "A", favorite: true },
        u1 => User { name: "Admin" },
        g2 => Game { name: "B", favorite: false },
        u2 => User { name: "Guest" }
    );

    assert_eq!(hlist.g1().get().name, "A");
    assert_eq!(hlist.g2().get().name, "B");
    assert_eq!(hlist.u1().get().name, "Admin");
    assert_eq!(hlist.u2().get().name, "Guest");

    assert_eq!(hlist.get::<Game>(0).unwrap().name, "A");
    assert_eq!(hlist.get::<User>(1).unwrap().name, "Admin");
    assert_eq!(hlist.get::<Game>(2).unwrap().name, "B");
    assert_eq!(hlist.get::<User>(3).unwrap().name, "Guest");
}

#[test]
fn book_ref_update() {
    let hlist = kyaaa::book!(
        item => Game { name: "Original", favorite: false }
    );

    hlist.item().update(|mut g| {
        g.name = "Updated";
        g.favorite = true;
        g
    });

    assert_eq!(hlist.item().get().name, "Updated");
    assert!(hlist.item().get().favorite);
}

#[test]
fn page_serialization_bounds() {
    kyaaa::page! {
        struct Packet {
            code: u32,
            flag: u8,
        }
    }

    let p = Packet { code: 42, flag: 1 };
    let bytes = p.to_bytes();

    let truncated = &bytes[0..bytes.len() - 1];
    assert!(Packet::from_bytes(truncated).is_none());

    let (decoded, rest) = Packet::from_bytes(bytes).unwrap();
    assert_eq!(decoded.code, 42);
    assert_eq!(decoded.flag, 1);
    assert!(rest.is_empty());
}

#[test]
fn book_zero_sized_types() {
    kyaaa::page! {
        struct Empty {}
    }

    let hlist = kyaaa::book!(
        e1 => Empty {}
    );

    assert!(hlist.get::<Empty>(0).is_some());
    assert!(hlist.get::<Empty>(1).is_none());
}

#[test]
fn page_exact_byte_layout() {
    kyaaa::page! {
        struct Header {
            magic: u16,
            version: u8,
            length: u32,
        }
    }

    let h = Header {
        magic: 0x55AA,
        version: 1,
        length: 1024,
    };
    let bytes = h.to_bytes();
    assert_eq!(bytes.len(), core::mem::size_of::<Header>());

    let (parsed, rest) = Header::from_bytes(bytes).unwrap();
    assert_eq!(parsed.magic, 0x55AA);
    assert_eq!(parsed.version, 1);
    assert_eq!(parsed.length, 1024);
    assert!(rest.is_empty());
}

#[test]
fn page_stream_consumption() {
    kyaaa::page! {
        struct Chunk {
            id: u32,
        }
    }

    let mut buffer = [0u8; 12];
    buffer[0..4].copy_from_slice(&10_u32.to_ne_bytes());
    buffer[4..8].copy_from_slice(&20_u32.to_ne_bytes());
    buffer[8..12].copy_from_slice(&30_u32.to_ne_bytes());

    let (p1, rest1) = Chunk::from_bytes(&buffer).unwrap();
    let (p2, rest2) = Chunk::from_bytes(rest1).unwrap();
    let (p3, rest3) = Chunk::from_bytes(rest2).unwrap();

    assert_eq!(p1.id, 10);
    assert_eq!(p2.id, 20);
    assert_eq!(p3.id, 30);
    assert!(rest3.is_empty());
}
