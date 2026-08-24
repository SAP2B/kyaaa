// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

    // falta implementar o que copia o valor para todos os cores/threads da cpu

use core::cell::UnsafeCell;
use core::hint::spin_loop;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

pub struct SeqLock<T> {
    seq: AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SeqLock<T> {}
unsafe impl<T: Send> Send for SeqLock<T> {}

impl<T> SeqLock<T> {
    #[inline(always)]
    pub const fn new(val: T) -> Self {
        Self {
            seq: AtomicUsize::new(0),
            data: UnsafeCell::new(val),
        }
    }
}

impl<T: Copy> SeqLock<T> {
    #[inline(always)]
    pub fn read(&self) -> T {
        loop {
            let seq1 = self.seq.load(Ordering::Acquire);
            if seq1 & 1 != 0 {
                spin_loop();
                continue;
            }

            let val = unsafe { self.data.get().read_volatile() };

            let seq2 = self.seq.load(Ordering::Acquire);
            if seq1 == seq2 {
                return val;
            }
            spin_loop();
        }
    }

    #[inline(always)]
    pub fn write(&self, val: T) {
        loop {
            let seq = self.seq.load(Ordering::Relaxed);
            if seq & 1 != 0 {
                spin_loop();
                continue;
            }

            if self
                .seq
                .compare_exchange_weak(
                    seq,
                    seq.wrapping_add(1),
                    Ordering::Acquire,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                unsafe {
                    self.data.get().write_volatile(val);
                }
                self.seq.store(seq.wrapping_add(2), Ordering::Release);
                break;
            }
            spin_loop();
        }
    }
}

pub struct TicketLock<T> {
    next: AtomicUsize,
    serving: AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for TicketLock<T> {}
unsafe impl<T: Send> Send for TicketLock<T> {}

pub struct TicketGuard<'a, T> {
    lock: &'a TicketLock<T>,
}

impl<T> TicketLock<T> {
    #[inline(always)]
    pub const fn new(val: T) -> Self {
        Self {
            next: AtomicUsize::new(0),
            serving: AtomicUsize::new(0),
            data: UnsafeCell::new(val),
        }
    }

    #[inline(always)]
    pub fn lock(&self) -> TicketGuard<'_, T> {
        let ticket = self.next.fetch_add(1, Ordering::Relaxed);
        while self.serving.load(Ordering::Acquire) != ticket {
            spin_loop();
        }
        TicketGuard { lock: self }
    }
}

impl<'a, T> Deref for TicketGuard<'a, T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> DerefMut for TicketGuard<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for TicketGuard<'a, T> {
    #[inline(always)]
    fn drop(&mut self) {
        self.lock.serving.fetch_add(1, Ordering::Release);
    }
}

const READ_COUNT_MASK: u32 = 0x00FF_FFFF;
const WRITER_WAITING: u32 = 0x0100_0000;
const WRITER_LOCKED: u32 = 0x0200_0000;

pub struct WriterLock<T> {
    state: AtomicU32,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for WriterLock<T> {}
unsafe impl<T: Send> Send for WriterLock<T> {}

pub struct ReadGuard<'a, T> {
    lock: &'a WriterLock<T>,
}

pub struct WriteGuard<'a, T> {
    lock: &'a WriterLock<T>,
}

impl<T> WriterLock<T> {
    #[inline(always)]
    pub const fn new(val: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            data: UnsafeCell::new(val),
        }
    }

    #[inline(always)]
    pub fn read(&self) -> ReadGuard<'_, T> {
        loop {
            let current = self.state.load(Ordering::Relaxed);
            if (current & (WRITER_LOCKED | WRITER_WAITING)) != 0 {
                spin_loop();
                continue;
            }

            if self
                .state
                .compare_exchange_weak(current, current + 1, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                return ReadGuard { lock: self };
            }
        }
    }

    #[inline(always)]
    pub fn write(&self) -> WriteGuard<'_, T> {
        while (self.state.fetch_or(WRITER_WAITING, Ordering::AcqRel) & WRITER_WAITING) != 0 {
            spin_loop();
        }

        let mut current = self.state.load(Ordering::Relaxed);
        loop {
            if (current & READ_COUNT_MASK) == 0 {
                match self.state.compare_exchange_weak(
                    current,
                    (current & !WRITER_WAITING) | WRITER_LOCKED,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => return WriteGuard { lock: self },
                    Err(actual) => {
                        current = actual;
                        continue;
                    }
                }
            }

            spin_loop();
            current = self.state.load(Ordering::Relaxed);
        }
    }
}

impl<'a, T> Deref for ReadGuard<'a, T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> Drop for ReadGuard<'a, T> {
    #[inline(always)]
    fn drop(&mut self) {
        self.lock.state.fetch_sub(1, Ordering::Release);
    }
}

impl<'a, T> Deref for WriteGuard<'a, T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> DerefMut for WriteGuard<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for WriteGuard<'a, T> {
    #[inline(always)]
    fn drop(&mut self) {
        self.lock.state.fetch_and(!WRITER_LOCKED, Ordering::Release);
    }
}
