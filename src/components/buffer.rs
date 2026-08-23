// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#[repr(transparent)]
#[derive(core::marker::Copy, core::clone::Clone)]
pub struct Buffer<const N: usize = 4096>(pub [u8; N]);

impl<const N: usize> Buffer<N> {
    #[inline(always)]
    pub const fn new(init: [u8; N]) -> Self {
        Self(init)
    }

    #[inline(always)]
    pub const fn zero() -> Self {
        Self([0; N])
    }

    #[inline(always)]
    pub fn fifo(&mut self) -> Ring<'_, N> {
        Ring {
            data: &mut self.0,
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    #[inline(always)]
    pub fn lifo(&mut self) -> Stack<'_, N> {
        Stack {
            data: &mut self.0,
            top: 0,
        }
    }

    #[inline(always)]
    pub fn arena(&mut self) -> Arena<'_, N> {
        Arena {
            data: &mut self.0,
            offset: 0,
        }
    }
}

pub struct Ring<'a, const N: usize> {
    data: &'a mut [u8; N],
    head: usize,
    tail: usize,
    len: usize,
}

impl<'a, const N: usize> Ring<'a, N> {
    #[inline(always)]
    pub fn push(&mut self, val: u8) -> Result<(), u8> {
        if self.len == N {
            return Err(val);
        }
        self.data[self.tail] = val;
        self.tail = (self.tail + 1) % N;
        self.len += 1;
        Ok(())
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<u8> {
        if self.len == 0 {
            return None;
        }
        let val = self.data[self.head];
        self.head = (self.head + 1) % N;
        self.len -= 1;
        Some(val)
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline(always)]
    pub const fn is_full(&self) -> bool {
        self.len == N
    }
}

pub struct Stack<'a, const N: usize> {
    data: &'a mut [u8; N],
    top: usize,
}

impl<'a, const N: usize> Stack<'a, N> {
    #[inline(always)]
    pub fn push(&mut self, val: u8) -> Result<(), u8> {
        if self.top == N {
            return Err(val);
        }
        self.data[self.top] = val;
        self.top += 1;
        Ok(())
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<u8> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        Some(self.data[self.top])
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.top
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.top == 0
    }

    #[inline(always)]
    pub const fn is_full(&self) -> bool {
        self.top == N
    }
}

pub struct Arena<'a, const N: usize> {
    data: &'a mut [u8; N],
    offset: usize,
}

impl<'a, const N: usize> Arena<'a, N> {
    #[inline(always)]
    pub fn alloc(&mut self, size: usize) -> Option<&'a mut [u8]> {
        if self.offset + size > N {
            return None;
        }
        let start = self.offset;
        self.offset += size;

        let ptr = self.data.as_mut_ptr();
        Some(unsafe { core::slice::from_raw_parts_mut(ptr.add(start), size) })
    }
    #[inline(always)]
    pub fn reset(&mut self) {
        self.offset = 0;
    }

    #[inline(always)]
    pub const fn remaining(&self) -> usize {
        N - self.offset
    }
}
