#![no_main]
#![no_std]

use kyaaa::prelude::*;

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}

kmain!(
    align(64);

    fn kmain(_sp: *const usize) -> i32 {
        clock!({
            let p1 = Q64::<8>::parse_ascii(b"");
            let p2 = Q64::<8>::parse_ascii(b"123.abc");
            let p3 = Q64::<8>::parse_ascii(b"42");
            let p4 = Q64::<8>::parse_ascii(b"0.00000001");

            let ip1 = IQ64::<8>::parse_ascii(b"-10");
            let ip2 = IQ64::<8>::parse_ascii(b"+10");
            let ip3 = IQ64::<8>::parse_ascii(b"-abc");

            let a = Q64::<8>::from_integer(10);
            let b = Q64::<8>::parse_ascii(b"2.5");

            let res_add = a + b;
            let res_sub = a - b;
            let res_mul = a * b;
            let res_div = a / b;

            let ia = IQ64::<8>::from_integer(-5);
            let ib = IQ64::<8>::parse_ascii(b"2.5");

            let i_add = ia + ib;
            let i_sub = ia - ib;
            let i_mul = ia * ib;
            let i_div = ia / ib;
            let i_neg = -ia;

            black_box((
                p1, p2, p3, p4,
                ip1, ip2, ip3,
                res_add, res_sub, res_mul, res_div,
                i_add, i_sub, i_mul, i_div, i_neg
            ));
        });

        0
    }
);
