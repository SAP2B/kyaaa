// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

use core::cell::UnsafeCell;
use core::hint::spin_loop;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct SeqLock<T> {
    seq: AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T> core::marker::Sync for SeqLock<T> {}
unsafe impl<T> core::marker::Send for SeqLock<T> {}

impl<T> SeqLock<T> {
    #[inline(always)]
    pub const fn new(val: T) -> Self {
        Self {
            seq: AtomicUsize::new(0),
            data: UnsafeCell::new(val),
        }
    }
}

impl<T: core::marker::Copy> SeqLock<T> {
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

#[repr(transparent)]
#[derive(core::marker::Copy, core::clone::Clone)]
pub struct Book<const N: usize>(pub [*const (); N]);

unsafe impl<const N: usize> core::marker::Sync for Book<N> {}
unsafe impl<const N: usize> core::marker::Send for Book<N> {}

impl<const N: usize> Book<N> {
    #[inline(always)]
    pub const fn new(entries: [*const (); N]) -> Self {
        Self(entries)
    }
}

#[macro_export]
macro_rules! book {
    (@munch
        [$($labels:ident,)*]
        [$($ptrs:expr,)*]
        [$($statics:item)*]
        [$($methods:tt)*]
    ) => {{
        $($statics)*

        #[allow(non_camel_case_types, dead_code)]
        #[repr(u8)]
        enum Id { $($labels,)* }

        #[allow(non_camel_case_types, dead_code)]
        struct Page {
            book: $crate::Book<{ [ $(core::stringify!($labels)),* ].len() }>,
        }

        impl core::clone::Clone for Page {
            #[inline(always)]
            fn clone(&self) -> Self { *self }
        }
        impl core::marker::Copy for Page {}

        #[allow(non_camel_case_types, non_snake_case, dead_code)]
        impl Page {
            #[inline(always)]
            const fn new() -> Self {
                Self { book: $crate::Book::new([ $($ptrs,)* ]) }
            }

            #[inline(always)]
            pub const fn get<T>(&self, id: u8) -> &'static T {
                unsafe { &*(self.book.0[id as usize] as *const T) }
            }

            $($methods)*
        }

        Page::new()
    }};

    (@munch
        [$($labels:ident,)*]
        [$($ptrs:expr,)*]
        [$($statics:item)*]
        [$($methods:tt)*]
        lockfree $label:ident => $Type:ident { $($fields:tt)* }
        $(, $($rest:tt)*)?
    ) => {
        $crate::book! { @munch
            [$($labels,)* $label,]
            [$($ptrs,)* ( core::ptr::addr_of!($label) as *const () ),]
            [
                $($statics)*

                #[allow(non_upper_case_globals)]
                static $label: $crate::components::book::SeqLock<$Type> =
                    $crate::components::book::SeqLock::new($Type { $($fields)* });
            ]
            [
                $($methods)*
                #[inline(always)]
                pub const fn $label(&self) -> &'static $crate::components::book::SeqLock<$Type> {
                    &$label
                }
            ]
            $($($rest)*)?
        }
    };

    (@munch
        [$($labels:ident,)*]
        [$($ptrs:expr,)*]
        [$($statics:item)*]
        [$($methods:tt)*]
        mut $label:ident => $Type:ident { $($fields:tt)* }
        $(, $($rest:tt)*)?
    ) => {
        $crate::book! { @munch
            [$($labels,)* $label,]
            [$($ptrs,)* ( $label.inner.get() as *const () ),]
            [
                $($statics)*

                #[allow(non_camel_case_types)]
                struct $label { inner: core::cell::UnsafeCell<$Type> }
                unsafe impl core::marker::Sync for $label {}

                #[allow(non_upper_case_globals)]
                static $label: $label = $label { inner: core::cell::UnsafeCell::new($Type { $($fields)* }) };
            ]
            [
                $($methods)*
                #[inline(always)]
                #[allow(clippy::mut_from_ref)]
                pub unsafe fn $label(&self) -> &'static mut $Type {
                    unsafe { &mut *$label.inner.get() }
                }
            ]
            $($($rest)*)?
        }
    };

    (@munch
        [$($labels:ident,)*]
        [$($ptrs:expr,)*]
        [$($statics:item)*]
        [$($methods:tt)*]
        $label:ident => $Type:ident { $($fields:tt)* }
        $(, $($rest:tt)*)?
    ) => {
        $crate::book! { @munch
            [$($labels,)* $label,]
            [$($ptrs,)* ( core::ptr::addr_of!($label) as *const () ),]
            [
                $($statics)*
                #[allow(non_upper_case_globals)]
                static $label: $Type = $Type { $($fields)* };
            ]
            [
                $($methods)*
                #[inline(always)]
                pub const fn $label(&self) -> &'static $Type {
                    &$label
                }
            ]
            $($($rest)*)?
        }
    };

    ( $($tt:tt)* ) => {
        $crate::book! { @munch [] [] [] [] $($tt)* }
    };
}
