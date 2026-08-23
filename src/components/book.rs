// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

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
            #[inline(always)] fn clone(&self) -> Self { *self }
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
