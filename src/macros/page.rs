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

                let fields_size = 0 $( + core::mem::size_of::<$field_type>() )*;

                assert!(
                    fields_size % declared_align == 0,
                    concat!(
                        "Alignment mismatch in struct `", stringify!($name),
                        "`! The sum of field sizes must be a multiple of the declared alignment."
                    )
                );

                assert!(
                    fields_size == struct_size,
                    concat!(
                        "Struct `", stringify!($name),
                        "` has internal/tail padding! Reorder fields or pad manually."
                    )
                );
            };

            impl $name {
                #[inline(always)]
                pub const fn to_bytes(&self) -> &[u8; core::mem::size_of::<Self>()] {
                    unsafe {
                        &*(self as *const Self as *const [u8; core::mem::size_of::<Self>()])
                    }
                }

                #[inline(always)]
                pub const fn from_bytes(bytes: &[u8; core::mem::size_of::<Self>()]) -> &Self {
                    unsafe { &*(bytes.as_ptr() as *const Self) }
                }

                $(
                    #[inline(always)]
                    pub fn $field_name<V: $crate::types::contracts::Assign<$field_type>>(&mut self, new: V) -> &mut Self {
                        $crate::types::contracts::Assign::assign_to(new, &mut self.$field_name);
                        self
                    }
                )*
            }
        )*
    };
}
