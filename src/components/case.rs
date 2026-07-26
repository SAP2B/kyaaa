//PDX short identifier: AGPL-3.0
//Copyright (C) 2026 SAP2B

#[macro_export]
macro_rules! bitmatch {
    ($val:expr, {
        $( $pattern:expr => $result:expr ),* $(,)?
    }) => {{
        let target = $val as i64;
        let mut final_result = 0i64;

        $(
            let diff = target ^ ($pattern as i64);
            let is_different = (diff | diff.wrapping_neg()) >> 63;
            let is_equal = is_different ^ 1;
            let mask = -(is_equal & 1);
            final_result |= ($result as i64) & mask;
        )*

        final_result
    }};
}

#[macro_export]
macro_rules! bitif {
    ($condition:expr => $if_true:expr, $if_false:expr) => {{
        let cond_bit = ($condition) as i64;
        let mask = -cond_bit;
        (($if_true as i64) & mask) | (($if_false as i64) & !mask)
    }};
}
