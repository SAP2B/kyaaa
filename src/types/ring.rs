use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(align(64))]
pub struct CacheAligned<T>(pub T);

#[repr(C)]
pub struct Ring<T: Copy, const N: usize> {
    pub head: CacheAligned<AtomicUsize>,
    pub tail: CacheAligned<AtomicUsize>,
    pub data: [UnsafeCell<MaybeUninit<T>>; N],
}

unsafe impl<T: Copy + Send, const N: usize> Sync for Ring<T, N> {}

impl<T: Copy, const N: usize> Default for Ring<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy, const N: usize> Ring<T, N> {
    pub const MASK: usize = N - 1;
    pub const BATCH: usize = if N >= 8 { N / 4 } else { 1 };

    #[inline(always)]
    pub const fn new() -> Self {
        assert!(N.is_power_of_two());
        Self {
            head: CacheAligned(AtomicUsize::new(0)),
            tail: CacheAligned(AtomicUsize::new(0)),
            data: [const { UnsafeCell::new(MaybeUninit::uninit()) }; N],
        }
    }

    #[inline(always)]
    pub fn split(&self) -> (Producer<'_, T, N>, Consumer<'_, T, N>) {
        (
            Producer {
                ring: self,
                cached_head: 0,
                local_tail: 0,
            },
            Consumer {
                ring: self,
                cached_tail: 0,
                local_head: 0,
                last_published_head: 0,
            },
        )
    }
}

#[repr(C)]
#[repr(align(64))]
pub struct Producer<'a, T: Copy, const N: usize> {
    ring: &'a Ring<T, N>,
    cached_head: usize,
    local_tail: usize,
}

impl<'a, T: Copy, const N: usize> Producer<'a, T, N> {
    #[inline(always)]
    pub fn reserve(&mut self) -> Option<&mut MaybeUninit<T>> {
        let tail = self.local_tail;

        if tail.wrapping_sub(self.cached_head) >= N {
            self.cached_head = self.ring.head.0.load(Ordering::Acquire);
            if tail.wrapping_sub(self.cached_head) >= N {
                return None;
            }
        }

        unsafe {
            let slot = &mut *self
                .ring
                .data
                .get_unchecked(tail & Ring::<T, N>::MASK)
                .get();
            Some(slot)
        }
    }

    #[inline(always)]
    pub fn commit(&mut self) {
        self.local_tail = self.local_tail.wrapping_add(1);
        self.ring.tail.0.store(self.local_tail, Ordering::Release);
    }

    #[inline(always)]
    pub fn spin<F: FnOnce(&mut MaybeUninit<T>)>(&mut self, f: F) {
        loop {
            if let Some(slot) = self.reserve() {
                f(slot);
                self.commit();
                break;
            }
            core::hint::spin_loop();
        }
    }

    #[inline(always)]
    pub fn push(&mut self, value: T) -> bool {
        if let Some(slot) = self.reserve() {
            slot.write(value);
            self.commit();
            true
        } else {
            false
        }
    }
}

#[repr(C)]
#[repr(align(64))]
pub struct Consumer<'a, T: Copy, const N: usize> {
    ring: &'a Ring<T, N>,
    cached_tail: usize,
    local_head: usize,
    last_published_head: usize,
}

impl<'a, T: Copy, const N: usize> Consumer<'a, T, N> {
    #[inline(always)]
    pub fn peek(&mut self) -> Option<&T> {
        let head = self.local_head;

        if head == self.cached_tail {
            self.cached_tail = self.ring.tail.0.load(Ordering::Acquire);
            if head == self.cached_tail {
                return None;
            }
        }

        unsafe {
            let src = self
                .ring
                .data
                .get_unchecked(head & Ring::<T, N>::MASK)
                .get()
                .cast::<T>();
            Some(&*src)
        }
    }

    #[inline(always)]
    pub fn advance(&mut self) {
        let next_head = self.local_head.wrapping_add(1);
        self.local_head = next_head;

        if next_head.wrapping_sub(self.last_published_head) >= Ring::<T, N>::BATCH {
            self.ring.head.0.store(next_head, Ordering::Release);
            self.last_published_head = next_head;
        }
    }

    #[inline(always)]
    pub fn eat<F: FnOnce(&T)>(&mut self, f: F) -> bool {
        if let Some(val) = self.peek() {
            f(val);
            self.advance();
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        self.peek().copied().inspect(|_| {
            self.advance();
        })
    }
}
