// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#[repr(transparent)]
#[derive(core::marker::Copy, core::clone::Clone)]
pub struct Proto<'a>(pub &'a [u8]);

impl<'a> Proto<'a> {
    pub const MAGIC_HEAD: u8 = 0xAA;
    pub const MAGIC_TAIL: u8 = 0x55;

    pub const HEADER_SIZE: usize = 2;
    pub const FOOTER_SIZE: usize = 2;
    pub const MIN_PACKET_SIZE: usize = Self::HEADER_SIZE + Self::FOOTER_SIZE;

    #[inline(always)]
    pub const fn new(data: &'a [u8]) -> Self {
        Self(data)
    }

    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let data = self.0;
        if data.len() < Self::MIN_PACKET_SIZE {
            return false;
        }

        let first = data[0];
        let last = data[data.len() - 1];

        first == Self::MAGIC_HEAD && last == Self::MAGIC_TAIL
    }

    #[inline(always)]
    pub fn version(&self) -> u8 {
        self.0[1]
    }

    #[inline(always)]
    pub fn payload(&self) -> &'a [u8] {
        let len = self.0.len();
        &self.0[Self::HEADER_SIZE..len - Self::FOOTER_SIZE]
    }

    #[inline(always)]
    pub fn footer_flag(&self) -> u8 {
        self.0[self.0.len() - 2]
    }
}
