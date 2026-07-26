//PDX short identifier: AGPL-3.0
//Copyright (C) 2026 SAP2B

crate::cfn!(q16, val: f32 => i32, {
    (val * 65536.0) as i32
});

crate::cfn!(q32, val: f64 => i64, {
    (val * 4294967296.0) as i64
});
