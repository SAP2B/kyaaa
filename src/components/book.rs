// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#[macro_export]
macro_rules! page {
    ($(
        struct $name:ident {
            $( $field_name:ident : $field_type:ty ),* $(,)?
        }
    )*) => {
        $(
            #[repr(C)]
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub struct $name {
                $( pub $field_name: $field_type, )*
            }

            impl $name {
                pub const FIELDS: &'static [&'static str] = &[
                    $( stringify!($field_name) ),*
                ];

                #[inline(always)]
                pub const fn to_bytes(&self) -> &[u8] {
                    unsafe {
                        core::slice::from_raw_parts(
                            (self as *const Self) as *const u8,
                            core::mem::size_of::<Self>(),
                        )
                    }
                }

                #[inline(always)]
                pub const fn from_bytes(input: &[u8]) -> Option<(Self, &[u8])> {
                    let size = core::mem::size_of::<Self>();
                    if input.len() < size {
                        return None;
                    }
                    let (chunk, rest) = input.split_at(size);
                    let instance = unsafe { core::ptr::read_unaligned(chunk.as_ptr() as *const Self) };
                    Some((instance, rest))
                }

                $(
                    #[inline(always)]
                    pub const fn $field_name(&mut self, new: $field_type) -> &mut Self {
                        self.$field_name = new;
                        self
                    }
                )*
            }
        )*
    };
}

#[macro_export]
macro_rules! book {
    ( $( $name:ident => $Type:ident { $($fields:tt)* } ),* $(,)? ) => {{

        $(
            #[allow(non_upper_case_globals)]
            static mut $name: $Type = $Type { $($fields)* };
        )*

        #[repr(transparent)]
        #[derive(Debug, core::marker::Copy, core::clone::Clone)]
        pub struct Book<const N: usize>(pub [*const (); N]);

        unsafe impl<const N: usize> core::marker::Sync for Book<N> {}
        unsafe impl<const N: usize> core::marker::Send for Book<N> {}

        #[repr(transparent)]
        pub struct BookRef<T>(pub *mut T);

        impl<T: core::marker::Copy> BookRef<T> {
            #[inline(always)]
            pub const fn new(ptr: *mut T) -> Self {
                Self(ptr)
            }

            #[inline(always)]
            pub const fn get(&self) -> &'static T {
                unsafe { &*self.0 }
            }

            #[inline(always)]
            pub fn update<F>(&self, f: F)
            where
            F: FnOnce(T) -> T {
                unsafe {
                    *self.0 = f(*self.0);
                }
            }

            #[inline(always)]
            pub const fn set(&self) -> &'static mut T {
                unsafe {
                    &mut *self.0
                }
            }
        }

        impl<const N: usize> Book<N> {
            #[inline(always)]
            pub const fn new(entries: [*const (); N]) -> Self {
                Self(entries)
            }

            #[inline(always)]
            pub const fn get<T>(&self, id: u8) -> Option<&'static T> {
                if (id as usize) < N {
                    unsafe { Some(&*(self.0[id as usize] as *const T)) }
                } else {
                    None
                }
            }

            #[inline(always)]
            pub const fn get_mut<T>(&self, id: u8) -> Option<&'static mut T> {
                if (id as usize) < N {
                    unsafe { Some(&mut *(self.0[id as usize] as *mut T)) }
                } else {
                    None
                }
            }
        }

        impl Book<{ [ $( core::stringify!($name) ),* ].len() }> {
            $(
                #[inline(always)]
                #[allow(static_mut_refs)]
                pub const fn $name(&self) -> BookRef<$Type> {
                    BookRef::new(core::ptr::addr_of_mut!($name))
                }
            )*
        }

        Book::new([
            $(
                core::ptr::addr_of_mut!($name) as *const ()
            ),*
        ])
    }};
}
