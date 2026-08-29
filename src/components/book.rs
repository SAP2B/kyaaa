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
        #[repr(transparent)]
        struct KyaaaSyncCell<T>(core::cell::UnsafeCell<T>);
        unsafe impl<T> core::marker::Sync for KyaaaSyncCell<T> {}

        $(
            #[allow(non_upper_case_globals)]
            static $name: KyaaaSyncCell<$Type> = KyaaaSyncCell(core::cell::UnsafeCell::new($Type { $($fields)* }));
        )*

        #[repr(transparent)]
        #[derive(Debug, core::marker::Copy, core::clone::Clone)]
        pub struct Book<const N: usize>(pub [*const (); N]);

        unsafe impl<const N: usize> core::marker::Sync for Book<N> {}
        unsafe impl<const N: usize> core::marker::Send for Book<N> {}

        #[repr(transparent)]
        pub struct BookRef<T, const ID: u8>(pub *mut T);

        #[allow(non_camel_case_types)]
        #[repr(u8)]
        enum BookIds {
            $( $name ),*
        }

        const N: usize = [ $( core::stringify!($name) ),* ].len();
        static TYPES: [core::any::TypeId; N] = [ $( core::any::TypeId::of::<$Type>() ),* ];

        impl<T: core::marker::Copy, const ID: u8> BookRef<T, ID> {
            #[inline(always)]
            pub const fn new(ptr: *mut T) -> Self {
                Self(ptr)
            }

            #[inline(always)]
            pub const fn id(&self) -> u8 {
                ID
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
            pub fn list<T: 'static>(&self) -> impl Iterator<Item = &'static T> + '_ {
                let target = core::any::TypeId::of::<T>();
                self.0.iter().zip(TYPES.iter()).filter_map(move |(&ptr, &type_id)| {
                    if type_id == target {
                        unsafe { Some(&*(ptr as *const T)) }
                    } else {
                        None
                    }
                })
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
                pub const fn $name(&self) -> BookRef<$Type, { BookIds::$name as u8 }> {
                    BookRef::new($name.0.get())
                }
            )*
        }

        Book::new([
            $(
                $name.0.get() as *const ()
            ),*
        ])
    }};
}
