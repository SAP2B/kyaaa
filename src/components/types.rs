// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Str<const N: usize>(pub [u8; N]);

impl<const N: usize> core::fmt::Debug for Str<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self.as_str(), f)
    }
}

impl<const N: usize> core::fmt::Display for Str<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self.as_str(), f)
    }
}

impl<const N: usize> Default for Str<N> {
    fn default() -> Self {
        Self([0u8; N])
    }
}

impl<const N: usize> Str<N> {
    pub const fn empty() -> Self {
        Self([0u8; N])
    }

    pub const fn from_str(s: &str) -> Self {
        Self::from_bytes(s.as_bytes())
    }

    pub const fn from_bytes(bytes: &[u8]) -> Self {
        let mut buf = [0u8; N];
        let mut i = 0;
        while i < bytes.len() && i < N {
            buf[i] = bytes[i];
            i += 1;
        }
        Self(buf)
    }

    pub fn as_str(&self) -> &str {
        let len = self.0.iter().position(|&b| b == 0).unwrap_or(N);
        core::str::from_utf8(&self.0[..len]).unwrap_or("")
    }
}

pub trait KyaaaConstInto<T> {
    fn kyaaa_into(self) -> T;
}

impl<T> KyaaaConstInto<T> for T {
    #[inline(always)]
    fn kyaaa_into(self) -> T {
        self
    }
}

impl<const N: usize> KyaaaConstInto<Str<N>> for &'static str {
    #[inline(always)]
    fn kyaaa_into(self) -> Str<N> {
        Str::from_str(self)
    }
}

impl<const N: usize, const M: usize> KyaaaConstInto<Str<N>> for &'static [u8; M] {
    #[inline(always)]
    fn kyaaa_into(self) -> Str<N> {
        Str::from_bytes(self)
    }
}

impl<const N: usize> KyaaaConstInto<Str<N>> for &'static [u8] {
    #[inline(always)]
    fn kyaaa_into(self) -> Str<N> {
        Str::from_bytes(self)
    }
}
