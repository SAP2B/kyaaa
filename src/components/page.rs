// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#[macro_export]
macro_rules! page {
    ($(
        align($align:expr) struct $name:ident {
            $( $field_name:ident : $field_type:ty ),* $(,)?
        }
    )*) => {
        $(
            #[repr(C, align($align))]
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $name {
                $( pub $field_name: $field_type, )*
            }

            const _: () = {
                let declared_align = $align;
                let struct_size = core::mem::size_of::<$name>();

                assert!(
                    struct_size % declared_align == 0,
                    concat!(
                        "Alignment mismatch in struct `", stringify!($name),
                        "`! Total struct size must be a multiple of the declared alignment."
                    )
                );
            };

            impl $name {
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
                    pub fn $field_name<V: $crate::KyaaaAssign<$field_type>>(&mut self, new: V) -> &mut Self {
                        $crate::KyaaaAssign::assign_to(new, &mut self.$field_name);
                        self
                    }
                )*
            }
        )*
    };
}
