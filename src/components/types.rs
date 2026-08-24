// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Bool(pub bool);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Char(pub char);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Str(pub &'static str);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct U8(pub u8);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct U16(pub u16);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct U32(pub u32);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct U64(pub u64);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct U128(pub u128);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Usize(pub usize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct I8(pub i8);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct I16(pub i16);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct I32(pub i32);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct I64(pub i64);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct I128(pub i128);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Isize(pub isize);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct F32(pub f32);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct F64(pub f64);

