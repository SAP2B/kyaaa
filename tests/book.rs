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

#[test]
fn page_struct_zeroed_memory_and_padding() {
    let game = Game::default();
    let bytes = game.to_bytes();

    assert_eq!(bytes.len(), std::mem::size_of::<Game>());
    assert!(bytes.iter().all(|&b| b == 0));
}

#[test]
fn book_ids_sequential_assignment() {
    let hlist = book!(
        first  => Game { name: Str::empty(), favorite: false },
        second => User { name: Str::empty() },
        third  => Game { name: Str::empty(), favorite: false }
    );

    assert_eq!(hlist.first().id(), 0);
    assert_eq!(hlist.second().id(), 1);
    assert_eq!(hlist.third().id(), 2);
}

#[test]
fn book_multiple_entries_same_type() {
    let hlist = book!(
        g1 => Game { name: Str::empty(), favorite: false },
        g2 => Game { name: Str::empty(), favorite: false }
    );

    hlist.g1().set().name("G1");
    hlist.g2().set().name("G2");

    let games: Vec<&'static Game> = hlist.list::<Game>().collect();
    assert_eq!(games.len(), 2);
    assert_eq!(hlist.g1().get().name, games[0].name);
    assert_eq!(hlist.g2().get().name, games[1].name);
}

#[test]
fn page_bytes_roundtrip_identity() {
    let mut original = Game::default();
    original.name("Elden Ring").favorite(true);

    let bytes = original.to_bytes();
    let (restored, _) = Game::from_bytes(bytes).unwrap();
    let double_bytes = restored.to_bytes();

    assert_eq!(bytes, double_bytes);
}

#[test]
fn page_struct_derived_traits() {
    let default_game = Game::default();
    assert_eq!(default_game.favorite, false);

    let mut game1 = Game::default();
    game1.name("Mario").favorite(true);

    let game2 = game1;
    let game3 = game1;

    assert_eq!(game1, game2);
    assert_eq!(game2, game3);
}

#[test]
fn page_from_bytes_with_remainder() {
    let mut original = Game::default();
    original.name("Tetris").favorite(true);

    let mut bytes = original.to_bytes().to_vec();
    bytes.extend_from_slice(&[0xFF, 0xEE, 0xDD]);

    let (restored, rest) = Game::from_bytes(&bytes).unwrap();

    assert_eq!(original.name, restored.name);
    assert_eq!(original.favorite, restored.favorite);
    assert_eq!(rest, &[0xFF, 0xEE, 0xDD]);
}

#[test]
fn book_dynamic_get_mut() {
    let hlist = book!(
        player1 => User { name: Str::empty() },
        player2 => User { name: Str::empty() }
    );

    hlist.player1().set().name("P1");
    hlist.player2().set().name("P2");

    let p1_id = hlist.player1().id();

    let mut_user = hlist.get_mut::<User>(p1_id).unwrap();
    mut_user.name("P1_Edited");

    assert_eq!(
        hlist.player1().get().name,
        hlist.get::<User>(p1_id).unwrap().name
    );
}

#[test]
fn book_invalid_id_access() {
    let hlist = book!(
        solo => User { name: Str::empty() }
    );

    let invalid_id = 99;

    assert!(hlist.get::<User>(invalid_id).is_none());
    assert!(hlist.get_mut::<User>(invalid_id).is_none());
}

#[test]
fn page_struct_methods() {
    let mut game = Game::default();
    game.name("Zelda").favorite(true);

    assert_eq!(game.favorite, true);
    assert_eq!(Game::FIELDS, &["name", "favorite"]);
}

#[test]
fn page_bytes_conversion() {
    let mut original = User::default();
    original.name("admin");

    let bytes = original.to_bytes();
    let (restored, rest) = User::from_bytes(bytes).unwrap();

    assert_eq!(original.name, restored.name);
    assert!(rest.is_empty());
}

#[test]
fn page_from_bytes_insufficient_length() {
    let empty_bytes: &[u8] = &[];
    assert!(User::from_bytes(empty_bytes).is_none());
}

#[test]
fn book_static_references_and_mutation() {
    let hlist = book!(
        zelda => Game { name: Str::empty(), favorite: false },
        admin => User { name: Str::empty() }
    );

    hlist.zelda().set().name("Zelda").favorite(true);
    hlist.admin().set().name("admin");

    assert_eq!(hlist.zelda().get().favorite, true);

    hlist.zelda().set().favorite(false);
    assert_eq!(hlist.zelda().get().favorite, false);
}

#[test]
fn book_dynamic_methods() {
    let hlist = book!(
        zelda => Game { name: Str::empty(), favorite: false },
        sap   => User { name: Str::empty() },
        nier  => Game { name: Str::empty(), favorite: false }
    );

    hlist.zelda().set().name("Zelda").favorite(true);
    hlist.sap().set().name("SAP2B");
    hlist.nier().set().name("Nier Automata").favorite(false);

    let games: Vec<&'static Game> = hlist.list::<Game>().collect();
    assert_eq!(games.len(), 2);
    assert_eq!(games[0].favorite, true);
    assert_eq!(games[1].favorite, false);

    let user_id = hlist.sap().id();
    let fetched_user = hlist.get::<User>(user_id).unwrap();
    assert_eq!(fetched_user.name, hlist.sap().get().name);

    assert!(hlist.get::<Game>(user_id).is_none());
}
