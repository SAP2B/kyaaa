// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#[macro_export]
macro_rules! book {
    ( @core $( $name:ident => $Type:ident { $( $f_name:ident : $f_val:expr ),* $(,)? } ),* ) => {
        use core::iter::Iterator;
        use core::option::Option::{Some, None};
        use core::any::TypeId;

        #[repr(transparent)]
        pub struct KyaaaSyncCell<T>(pub core::cell::UnsafeCell<T>);
        unsafe impl<T> core::marker::Sync for KyaaaSyncCell<T> {}
        unsafe impl<T> core::marker::Send for KyaaaSyncCell<T> {}

        $(
            #[allow(non_upper_case_globals)]
            static $name: KyaaaSyncCell<$Type> = KyaaaSyncCell(core::cell::UnsafeCell::new(
                $Type {
                    $( $f_name : $f_val ),*
                }
            ));
        )*

        #[repr(transparent)]
        pub struct Book<const N: usize>(pub [*const (); N]);

        unsafe impl<const N_ENTRIES: usize> core::marker::Sync for Book<N_ENTRIES> {}
        unsafe impl<const N_ENTRIES: usize> core::marker::Send for Book<N_ENTRIES> {}

        #[repr(transparent)]
        pub struct BookRef<T, const ID: u8>(pub *mut T);

        #[allow(non_camel_case_types)]
        #[repr(u8)]
        pub enum BookIds {
            $( $name ),*
        }

        pub const N: usize = [ $( core::stringify!($name) ),* ].len();

        impl<T, const ID: u8> BookRef<T, ID> {
            #[inline(always)]
            pub const fn new(ptr: *mut T) -> Self { Self(ptr) }
            #[inline(always)]
            pub const fn id(&self) -> u8 { ID }
            #[inline(always)]
            pub const fn get(&self) -> &'static T { unsafe { &*self.0 } }
            #[inline(always)]
            pub fn set(&self) -> &'static mut T { unsafe { &mut *self.0 } }
        }

        impl<const N_ENTRIES: usize> Book<N_ENTRIES> {
            #[inline(always)]
            pub const fn new(entries: [*const (); N_ENTRIES]) -> Self { Self(entries) }

            #[inline(always)]
            pub fn list<T: 'static>(&self) -> impl Iterator<Item = &'static T> + '_ {
                let types = [ $( TypeId::of::<$Type>() ),* ];
                let target = TypeId::of::<T>();
                self.0.iter().zip(types).filter_map(move |(&ptr, type_id)| {
                    if type_id == target {
                        unsafe { Some(&*(ptr as *const T)) }
                    } else {
                        None
                    }
                })
            }

            #[inline(always)]
            pub fn list_mut<T: 'static>(&self) -> impl Iterator<Item = &'static mut T> + '_ {
                let types = [ $( TypeId::of::<$Type>() ),* ];
                let target = TypeId::of::<T>();
                self.0.iter().zip(types).filter_map(move |(&ptr, type_id)| {
                    if type_id == target {
                        unsafe { Some(&mut *(ptr as *mut T)) }
                    } else {
                        None
                    }
                })
            }

            #[inline(always)]
            pub fn get<T: 'static>(&self, id: u8) -> Option<&'static T> {
                let idx = id as usize;
                let types = [ $( TypeId::of::<$Type>() ),* ];
                if let Some(&type_id) = types.get(idx) {
                    if type_id == TypeId::of::<T>() {
                        return unsafe { Some(&*(self.0[idx] as *const T)) };
                    }
                }
                None
            }

            #[inline(always)]
            pub fn get_mut<T: 'static>(&self, id: u8) -> Option<&'static mut T> {
                let idx = id as usize;
                let types = [ $( TypeId::of::<$Type>() ),* ];
                if let Some(&type_id) = types.get(idx) {
                    if type_id == TypeId::of::<T>() {
                        return unsafe { Some(&mut *(self.0[idx] as *mut T)) };
                    }
                }
                None
            }
        }

        impl Book<N> {
            $(
                #[inline(always)]
                pub fn $name(&self) -> BookRef<$Type, { BookIds::$name as u8 }> {
                    BookRef::new($name.0.get())
                }
            )*
        }
    };

    (
        $vis:vis static $BookName:ident;
        $( $name:ident => $Type:ident { $( $f_name:ident : $f_val:expr ),* $(,)? } ),* $(,)?
    ) => {
        $vis use $BookName::INSTANCE as $BookName;

        #[allow(non_snake_case)]
        $vis mod $BookName {
            use super::*;

            book!(@core $( $name => $Type { $( $f_name : $f_val ),* } ),* );

            pub static INSTANCE: Book<N> = Book::new([
                $(
                    $name.0.get() as *const ()
                ),*
            ]);
        }
    };

    ( $( $name:ident => $Type:ident { $( $f_name:ident : $f_val:expr ),* $(,)? } ),* $(,)? ) => {{
        book!(@core $( $name => $Type { $( $f_name : $f_val ),* } ),* );

        Book::new([
            $(
                $name.0.get() as *const ()
            ),*
        ])
    }};
}
