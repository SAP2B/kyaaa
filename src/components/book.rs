// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#[macro_export]
macro_rules! book {
    ( $( $name:ident => $Type:ident { $( $f_name:ident : $f_val:expr ),* $(,)? } ),* $(,)? ) => {{
        #[repr(transparent)]
        struct KyaaaSyncCell<T>(core::cell::UnsafeCell<T>);
        unsafe impl<T> core::marker::Sync for KyaaaSyncCell<T> {}

        $(
            #[allow(non_upper_case_globals)]
            static $name: KyaaaSyncCell<$Type> = KyaaaSyncCell(core::cell::UnsafeCell::new(
                $Type {
                    $( $f_name : $f_val ),*
                }
            ));
        )*

        #[repr(transparent)]
        #[derive(Debug, core::marker::Copy, core::clone::Clone)]
        pub struct Book<const N: usize>(pub [*const (); N]);

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
            pub fn set(&self) -> &'static mut T {
                unsafe { &mut *self.0 }
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
            pub fn get<T: 'static>(&self, id: u8) -> Option<&'static T> {
                let idx = id as usize;
                if idx < N && TYPES[idx] == core::any::TypeId::of::<T>() {
                    unsafe { Some(&*(self.0[idx] as *const T)) }
                } else {
                    None
                }
            }

            #[inline(always)]
            pub fn get_mut<T: 'static>(&self, id: u8) -> Option<&'static mut T> {
                let idx = id as usize;
                if idx < N && TYPES[idx] == core::any::TypeId::of::<T>() {
                    unsafe { Some(&mut *(self.0[idx] as *mut T)) }
                } else {
                    None
                }
            }
        }

        impl Book<{ [ $( core::stringify!($name) ),* ].len() }> {
            $(
                #[inline(always)]
                pub fn $name(&self) -> BookRef<$Type, { BookIds::$name as u8 }> {
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
