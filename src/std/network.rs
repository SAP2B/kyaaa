use crate::types::prelude::*;
use crate::{page, sbook};

page!(
    align(16) struct Token8 {
        text: Str<15>,
        len: u8,
    }

    align(32) struct Token16 {
        text: Str<31>,
        len: u8,
    }

    align(64) struct Token32 {
        text: Str<63>,
        len: u8,
    }
);

sbook!(
    pub Tu8Book: u8 => Token8 {
        hello = Token8 { text: Str::from_str("hello "), len: 6 },
        world = Token8 { text: Str::from_str("world "), len: 6 },
    }
);

sbook!(
    pub Tu16Book: u16 => Token16 {
        rust = Token16 { text: Str::from_str("rust programming "), len: 17 },
        wasm = Token16 { text: Str::from_str("wasm target "), len: 12 },
    }
);

sbook!(
    pub Tu24Book: u32 => Token32 {
        kyaaa = Token32 { text: Str::from_str("kyaaa high frequency engine "), len: 27 },
        monster = Token32 { text: Str::from_str("monster distributed runtime "), len: 28 },
    }
);

#[repr(u8)]
pub enum DecodedToken {
    T8(&'static Token8),
    T16(&'static Token16),
    T32(&'static Token32),
}

#[repr(transparent)]
pub struct Stream<'a>(pub &'a [u8]);

impl<'a> Stream<'a> {
    #[inline(always)]
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }

    #[inline(always)]
    pub fn decode(&self, mut pos: usize) -> Option<(DecodedToken, usize)> {
        if pos >= self.0.len() {
            return None;
        }

        let tag = self.0[pos];
        pos += 1;

        match tag {
            0 => {
                if pos >= self.0.len() {
                    return None;
                }
                let id = self.0[pos];
                pos += 1;

                let token = Tu8Book.get(id)?;
                Some((DecodedToken::T8(token), pos))
            }
            1 => {
                if pos + 2 > self.0.len() {
                    return None;
                }
                let id =
                    unsafe { core::ptr::read_unaligned(self.0.as_ptr().add(pos) as *const u16) };
                pos += 2;

                let token = Tu16Book.get(id)?;
                Some((DecodedToken::T16(token), pos))
            }
            _ => {
                if pos + 4 > self.0.len() {
                    return None;
                }
                let id =
                    unsafe { core::ptr::read_unaligned(self.0.as_ptr().add(pos) as *const u32) };
                pos += 4;

                let token = Tu24Book.get(id)?;
                Some((DecodedToken::T32(token), pos))
            }
        }
    }
}
