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
            #[derive(Debug, Clone, Copy, PartialEq, Default)]
            pub struct $name {
                $( pub $field_name: $field_type, )*
            }

            const _: () = {
                let sum_fields_size = 0usize $( + core::mem::size_of::<$field_type>() )*;
                let struct_size = core::mem::size_of::<$name>();
                assert!(
                    struct_size == sum_fields_size,
                    concat!(
                        "Padding detected in struct `", stringify!($name),
                        "`! Sum of field sizes does not match total struct size."
                    )
                );
            };

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
                    pub fn $field_name<V: $crate::KyaaaConstInto<$field_type>>(&mut self, new: V) -> &mut Self {
                        self.$field_name = $crate::KyaaaConstInto::kyaaa_into(new);
                        self
                    }
                )*
            }
        )*
    };
}
