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
        [$($ptrs:tt)*]
        [$($statics:tt)*]
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
                Self { book: $crate::Book::new([ $($ptrs)* ]) }
            }

            #[inline(always)]
            pub const fn get<T>(&self, id: u8) -> Option<&'static T> {
                const N: usize = { [ $(core::stringify!($labels)),* ].len() };
                if (id as usize) < N {
                    unsafe { Some(&*(self.book.0[id as usize] as *const T)) }
                } else {
                    None
                }
            }

            $($methods)*
        }

        Page::new()
    }};

    (@munch
        [$($labels:ident,)*]
        [$($ptrs:tt)*]
        [$($statics:tt)*]
        [$($methods:tt)*]
        readfull $label:ident => $Type:ident { $($fields:tt)* }
        $(, $($rest:tt)*)?
    ) => {
        $crate::book! { @munch
            [$($labels,)* $label,]
            [$($ptrs)* ( core::ptr::addr_of!($label) as *const () ),]
            [
                $($statics)*
                #[allow(non_upper_case_globals)]
                static $label: $crate::SeqLock<$Type> =
                    $crate::SeqLock::new($Type { $($fields)* });
            ]
            [
                $($methods)*
                #[inline(always)]
                pub const fn $label(&self) -> &'static $crate::SeqLock<$Type> {
                    &$label
                }
            ]
            $($($rest)*)?
        }
    };

    (@munch
        [$($labels:ident,)*]
        [$($ptrs:tt)*]
        [$($statics:tt)*]
        [$($methods:tt)*]
        writefull $label:ident => $Type:ident { $($fields:tt)* }
        $(, $($rest:tt)*)?
    ) => {
        $crate::book! { @munch
            [$($labels,)* $label,]
            [$($ptrs)* ( core::ptr::addr_of!($label) as *const () ),]
            [
                $($statics)*
                #[allow(non_upper_case_globals)]
                static $label: $crate::WriterLock<$Type> =
                    $crate::WriterLock::new($Type { $($fields)* });
            ]
            [
                $($methods)*
                #[inline(always)]
                pub const fn $label(&self) -> &'static $crate::WriterLock<$Type> {
                    &$label
                }
            ]
            $($($rest)*)?
        }
    };

    (@munch
        [$($labels:ident,)*]
        [$($ptrs:tt)*]
        [$($statics:tt)*]
        [$($methods:tt)*]
        fair $label:ident => $Type:ident { $($fields:tt)* }
        $(, $($rest:tt)*)?
    ) => {
        $crate::book! { @munch
            [$($labels,)* $label,]
            [$($ptrs)* ( core::ptr::addr_of!($label) as *const () ),]
            [
                $($statics)*
                #[allow(non_upper_case_globals)]
                static $label: $crate::TicketLock<$Type> =
                    $crate::TicketLock::new($Type { $($fields)* });
            ]
            [
                $($methods)*
                #[inline(always)]
                pub const fn $label(&self) -> &'static $crate::TicketLock<$Type> {
                    &$label
                }
            ]
            $($($rest)*)?
        }
    };

    (@munch
        [$($labels:ident,)*]
        [$($ptrs:tt)*]
        [$($statics:tt)*]
        [$($methods:tt)*]
        mut $label:ident => $Type:ident { $($fields:tt)* }
        $(, $($rest:tt)*)?
    ) => {
        $crate::book! { @munch
            [$($labels,)* $label,]
            [$($ptrs)* ( $label.inner.get() as *const () ),]
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
        [$($ptrs:tt)*]
        [$($statics:tt)*]
        [$($methods:tt)*]
        fifo $label:ident => $size:expr
        $(, $($rest:tt)*)?
    ) => {
        $crate::book! { @munch
            [$($labels,)* $label,]
            [$($ptrs)* ( core::ptr::addr_of!($label) as *const () ),]
            [
                $($statics)*
                #[allow(non_upper_case_globals)]
                static $label: $crate::TicketLock<$crate::Buffer<$size>> =
                    $crate::TicketLock::new($crate::Buffer::zero());
            ]
            [
                $($methods)*
                #[inline(always)]
                pub const fn $label(&self) -> &'static $crate::TicketLock<$crate::Buffer<$size>> {
                    &$label
                }
            ]
            $($($rest)*)?
        }
    };

    (@munch
        [$($labels:ident,)*]
        [$($ptrs:tt)*]
        [$($statics:tt)*]
        [$($methods:tt)*]
        lifo $label:ident => $size:expr
        $(, $($rest:tt)*)?
    ) => {
        $crate::book! { @munch
            [$($labels,)* $label,]
            [$($ptrs)* ( core::ptr::addr_of!($label) as *const () ),]
            [
                $($statics)*
                #[allow(non_upper_case_globals)]
                static $label: $crate::TicketLock<$crate::Buffer<$size>> =
                    $crate::TicketLock::new($crate::Buffer::zero());
            ]
            [
                $($methods)*
                #[inline(always)]
                pub const fn $label(&self) -> &'static $crate::TicketLock<$crate::Buffer<$size>> {
                    &$label
                }
            ]
            $($($rest)*)?
        }
    };

    (@munch
        [$($labels:ident,)*]
        [$($ptrs:tt)*]
        [$($statics:tt)*]
        [$($methods:tt)*]
        arena $label:ident => $size:expr
        $(, $($rest:tt)*)?
    ) => {
        $crate::book! { @munch
            [$($labels,)* $label,]
            [$($ptrs)* ( core::ptr::addr_of!($label) as *const () ),]
            [
                $($statics)*
                #[allow(non_upper_case_globals)]
                static $label: $crate::TicketLock<$crate::Buffer<$size>> =
                    $crate::TicketLock::new($crate::Buffer::zero());
            ]
            [
                $($methods)*
                #[inline(always)]
                pub const fn $label(&self) -> &'static $crate::TicketLock<$crate::Buffer<$size>> {
                    &$label
                }
            ]
            $($($rest)*)?
        }
    };

    (@munch
        [$($labels:ident,)*]
        [$($ptrs:tt)*]
        [$($statics:tt)*]
        [$($methods:tt)*]
        $label:ident => $Type:ident { $($fields:tt)* }
        $(, $($rest:tt)*)?
    ) => {
        $crate::book! { @munch
            [$($labels,)* $label,]
            [$($ptrs)* ( core::ptr::addr_of!($label) as *const () ),]
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

    (@munch
        [$($labels:ident,)*]
        [$($ptrs:tt)*]
        [$($statics:tt)*]
        [$($methods:tt)*]
        $label:ident => $expr:expr
        $(, $($rest:tt)*)?
    ) => {
        $crate::book! { @munch
            [$($labels,)* $label,]
            [$($ptrs)* ( core::ptr::addr_of!($label) as *const () ),]
            [
                $($statics)*
                #[allow(non_upper_case_globals)]
                static $label: _ = $expr;
            ]
            [
                $($methods)*
                #[inline(always)]
                pub const fn $label(&self) -> &'static _ {
                    &$label
                }
            ]
            $($($rest)*)?
        }
    };

    (@munch $($tt:tt)*) => {
        core::compile_error!(core::concat!("Macro book! error line: ", core::stringify!($($tt)*)));
    };

    (fifo $($tt:tt)*) => {
        $crate::book! { @munch [] [] [] [] fifo $($tt)* }
    };
    (lifo $($tt:tt)*) => {
        $crate::book! { @munch [] [] [] [] lifo $($tt)* }
    };
    (arena $($tt:tt)*) => {
        $crate::book! { @munch [] [] [] [] arena $($tt)* }
    };
    (readfull $($tt:tt)*) => {
        $crate::book! { @munch [] [] [] [] readfull $($tt)* }
    };
    (writefull $($tt:tt)*) => {
        $crate::book! { @munch [] [] [] [] writefull $($tt)* }
    };
    (fair $($tt:tt)*) => {
        $crate::book! { @munch [] [] [] [] fair $($tt)* }
    };
    (mut $($tt:tt)*) => {
        $crate::book! { @munch [] [] [] [] mut $($tt)* }
    };
    ($label:ident => $($tt:tt)*) => {
        $crate::book! { @munch [] [] [] [] $label => $($tt)* }
    };
}
