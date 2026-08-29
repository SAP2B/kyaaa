// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(align(64))]
pub struct CacheLineAligned<T>(pub T);

pub struct Ring<T, const N: usize> {
    pub head: CacheLineAligned<AtomicUsize>,
    pub tail: CacheLineAligned<AtomicUsize>,
    pub buffer: UnsafeCell<[MaybeUninit<T>; N]>,
}

impl<T, const N: usize> Drop for Ring<T, N> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

unsafe impl<T: Send, const N: usize> Sync for Ring<T, N> {}
unsafe impl<T: Send, const N: usize> Send for Ring<T, N> {}

impl<T, const N: usize> Default for Ring<T, N> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Ring<T, N> {
    #[inline(always)]
    pub const fn new() -> Self {
        const {
            assert!(
                N.is_power_of_two(),
                "Buffer capacity must be a power of two"
            );
        }

        Self {
            head: CacheLineAligned(AtomicUsize::new(0)),
            tail: CacheLineAligned(AtomicUsize::new(0)),
            buffer: UnsafeCell::new([const { MaybeUninit::uninit() }; N]),
        }
    }

    #[inline(always)]
    pub fn push(&self, item: T) -> Result<(), T> {
        let head = self.head.0.load(Ordering::Relaxed);
        let tail = self.tail.0.load(Ordering::Acquire);

        if head.wrapping_sub(tail) >= N {
            return Err(item);
        }

        let index = head & (N - 1);
        unsafe {
            let slot = &mut (*self.buffer.get())[index];
            slot.write(item);
        }

        self.head.0.store(head.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    #[inline(always)]
    pub fn pop(&self) -> Option<T> {
        let tail = self.tail.0.load(Ordering::Relaxed);
        let head = self.head.0.load(Ordering::Acquire);

        if head == tail {
            return None;
        }

        let index = tail & (N - 1);
        let item = unsafe {
            let slot = &(*self.buffer.get())[index];
            slot.assume_init_read()
        };

        self.tail.0.store(tail.wrapping_add(1), Ordering::Release);
        Some(item)
    }

    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        N
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        let head = self.head.0.load(Ordering::Relaxed);
        let tail = self.tail.0.load(Ordering::Relaxed);
        head.wrapping_sub(tail)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.len() >= N
    }
}
