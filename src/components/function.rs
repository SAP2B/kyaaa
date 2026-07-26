//PDX short identifier: AGPL-3.0
//Copyright (C) 2026 SAP2B

#[macro_export]
macro_rules! cfn {
    ($name:ident, $arg:ident: $arg_ty:ty => $ret_ty:ty, $body:block) => {
        #[inline(always)]
        pub const fn $name($arg: $arg_ty) -> $ret_ty {
            $body
        }
    };
}

#[macro_export]
macro_rules! ufn {
    ($name:ident, $($arg:ident: $arg_ty:ty ),* => $ret_ty:ty, $body:block) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($( $arg: $arg_ty ),*) -> $ret_ty {
            $body
        }
    };
}
