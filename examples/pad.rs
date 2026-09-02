// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

use colorful::{Colorful, HSL};
use core::mem::{offset_of, size_of};

kyaaa::page!(
    struct Game {
        name: kyaaa::Str<32>,
        favorite: bool,
    }

    struct Header {
        length: u32,
        magic: u16,
        version: u8,
        pad: u8,
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
    let game_size = size_of::<Game>();
    let game_fields = size_of::<kyaaa::Str<32>>() + size_of::<bool>();
    format!(
        "Game Total Size: {} bytes | Sum of Fields: {} bytes | Tail Padding: 0 bytes",
        game_size, game_fields
    )
    .print_hsl();

    let header_size = size_of::<Header>();
    let header_fields = size_of::<u32>() + size_of::<u16>() + size_of::<u8>() + size_of::<u8>();
    format!(
        "Header Total Size: {} bytes | Sum of Fields: {} bytes | Internal Padding: 0 byte(s)",
        header_size, header_fields
    )
    .print_hsl();

    format!(
        "Field Offsets -> length: byte {}, magic: byte {}, version: byte {}",
        offset_of!(Header, length),
        offset_of!(Header, magic),
        offset_of!(Header, version)
    )
    .print_hsl();

    let mut h = Header::default();
    h.length(1024).magic(0x55AA).version(1).pad(0);

    let header_bytes = h.to_bytes();
    format!(
        "Header Raw Bytes ({}) -> {:02X?}",
        header_bytes.len(),
        header_bytes
    )
    .print_hsl();

    let mut g = Game::default();
    g.name("Zelda").favorite(true);

    let game_bytes = g.to_bytes();
    format!(
        "Game Raw Bytes ({}) -> {:02X?}",
        game_bytes.len(),
        game_bytes
    )
    .print_hsl();
}
