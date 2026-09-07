// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Str<const N: usize>(pub [u8; N]);

pub trait KyaaaAssign<T> {
    fn assign_to(self, dest: &mut T);
}

impl<T> KyaaaAssign<T> for T {
    #[inline(always)]
    fn assign_to(self, dest: &mut T) {
        *dest = self;
    }
}

impl<const N: usize> KyaaaAssign<Str<N>> for &'static str {
    #[inline(always)]
    fn assign_to(self, dest: &mut Str<N>) {
        let bytes = self.as_bytes();
        let len = if bytes.len() < N { bytes.len() } else { N };
        unsafe {
            let ptr = dest.0.as_mut_ptr();
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, len);

            if len < N {
                core::ptr::write_bytes(ptr.add(len), 0, N - len);
            }
        }
    }
}

impl<const N: usize> KyaaaAssign<Str<N>> for &[u8] {
    #[inline(always)]
    fn assign_to(self, dest: &mut Str<N>) {
        let len = if self.len() < N { self.len() } else { N };
        unsafe {
            let ptr = dest.0.as_mut_ptr();
            core::ptr::copy_nonoverlapping(self.as_ptr(), ptr, len);

            if len < N {
                core::ptr::write_bytes(ptr.add(len), 0, N - len);
            }
        }
    }
}

impl<const N: usize, const M: usize> KyaaaAssign<Str<N>> for &[u8; M] {
    #[inline(always)]
    fn assign_to(self, dest: &mut Str<N>) {
        self.as_slice().assign_to(dest);
    }
}

impl<const N: usize> core::fmt::Debug for Str<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self.as_str(), f)
    }
}

impl<const N: usize> Str<N> {
    pub const EMPTY: Self = Self([0u8; N]);

    #[inline(always)]
    pub const fn from_bytes(bytes: &[u8]) -> Self {
        let mut buf = [0u8; N];
        let len = if bytes.len() < N { bytes.len() } else { N };
        let mut i = 0;
        while i < len {
            buf[i] = bytes[i];
            i += 1;
        }
        Self(buf)
    }

    #[inline(always)]
    pub const fn from_str(s: &str) -> Self {
        Self::from_bytes(s.as_bytes())
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        core::ffi::CStr::from_bytes_until_nul(&self.0)
            .map(|s| s.to_bytes().len())
            .unwrap_or(N)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0[0] == 0
    }

    #[inline(always)]
    pub fn push_str(&mut self, s: &str) -> &mut Self {
        let self_len = self.len();
        let bytes = s.as_bytes();
        let copy_len = if bytes.len() < (N - self_len) {
            bytes.len()
        } else {
            N - self_len
        };

        unsafe {
            core::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                self.0.as_mut_ptr().add(self_len),
                copy_len,
            );
        }
        self
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.0.get_unchecked(..self.len())) }
    }
}

impl<const N: usize> core::ops::Deref for Str<N> {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const N: usize> AsRef<str> for Str<N> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> Default for Str<N> {
    #[inline(always)]
    fn default() -> Self {
        Self::EMPTY
    }
}
