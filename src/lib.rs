// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#![no_std]

#[repr(transparent)]
#[derive(core::marker::Copy, core::clone::Clone)]
pub struct Kyaaa<const N: usize>(pub [*const (); N]);

unsafe impl<const N: usize> core::marker::Sync for Kyaaa<N> {}
unsafe impl<const N: usize> core::marker::Send for Kyaaa<N> {}

impl<const N: usize> Kyaaa<N> {
    #[inline(always)]
    pub const fn new(entries: [*const (); N]) -> Self {
        Self(entries)
    }
}

#[macro_export]
macro_rules! kyaaa {
    ($( $label:ident => $val:expr ),* $(,)?) => {{
        const _: () = {
            const VALUES: &[&str] = &[ $( core::stringify!($val) ),* ];
            let mut a = 0;
            while a < VALUES.len() {
                let mut b = a + 1;
                while b < VALUES.len() {
                    let v1 = VALUES[a].as_bytes();
                    let v2 = VALUES[b].as_bytes();
                    if v1.len() == v2.len() {
                        let mut k = 0;
                        let mut same = true;
                        while k < v1.len() {
                            if v1[k] != v2[k] {
                                same = false;
                                break;
                            }
                            k += 1;
                        }
                        if same {
                            core::panic!("Duplicate value in kyaaa!");
                        }
                    }
                    b += 1;
                }
                a += 1;
            }
        };

        #[allow(non_camel_case_types, dead_code)]
        #[repr(u8)]
        enum InternalId {
            $( $label, )*
        }

        #[allow(non_camel_case_types, dead_code)]
        struct Page<$( $label ),*> {
            __book: $crate::Kyaaa<{ [ $( core::stringify!($label) ),* ].len() }>,
            __phantom: core::marker::PhantomData<( $( $label, )* )>,
        }

        #[allow(non_camel_case_types)]
        impl<$( $label ),*> core::clone::Clone for Page<$( $label ),*> {
            #[inline(always)]
            fn clone(&self) -> Self { *self }
        }

        #[allow(non_camel_case_types)]
        impl<$( $label ),*> core::marker::Copy for Page<$( $label ),*> {}

        #[allow(non_camel_case_types, dead_code)]
        impl<$( $label ),*> Page<$( $label ),*> {
            #[inline(always)]
            const fn __new_inferred(
                __book: $crate::Kyaaa<{ [ $( core::stringify!($label) ),* ].len() }>,
                $( _: &$label, )*
            ) -> Self {
                Self {
                    __book,
                    __phantom: core::marker::PhantomData,
                }
            }

            #[inline(always)]
            pub const fn get_by_u8<T>(&self, id: u8) -> &'static T {
                unsafe {
                    let ptr = *self.__book.0.as_ptr().add(id as usize);
                    &*(ptr as *const T)
                }
            }

            $(
                #[inline(always)]
                #[allow(non_snake_case, dead_code)]
                pub const fn $label(&self) -> &'static $label {
                    unsafe {
                        let ptr = *self.__book.0.as_ptr().add(InternalId::$label as usize);
                        &*(ptr as *const $label)
                    }
                }
            )*
        }

        Page::__new_inferred(
            $crate::Kyaaa::new([
                $({
                    &const { $val } as *const _ as *const ()
                }),*
            ]),
            $( &const { $val }, )*
        )
    }};
}
