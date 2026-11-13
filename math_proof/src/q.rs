use kyaaa::prelude::*;

#[kani::proof]
fn verify_iq64_add_sub() {
    let ra: i32 = kani::any();
    let rb: i32 = kani::any();
    kani::assume(ra > -1_000_000 && ra < 1_000_000);
    kani::assume(rb > -1_000_000 && rb < 1_000_000);
    let a = IQ64::<8>::from_raw(ra as i64);
    let b = IQ64::<8>::from_raw(rb as i64);
    assert_eq!((a + b).to_raw(), (ra as i64).wrapping_add(rb as i64));
    assert_eq!((a - b).to_raw(), (ra as i64).wrapping_sub(rb as i64));
}

#[kani::proof]
fn verify_iq64_negation() {
    let ra: i32 = kani::any();
    kani::assume(ra > -1_000_000 && ra < 1_000_000);
    let a = IQ64::<8>::from_raw(ra as i64);
    assert_eq!((-(-a)).to_raw(), a.to_raw());
}

#[kani::proof]
fn verify_q64_mul() {
    let ra: u16 = kani::any();
    let rb: u16 = kani::any();
    kani::assume(ra < 10_000);
    kani::assume(rb < 10_000);
    let a = Q64::<8>::from_raw(ra as u64);
    let b = Q64::<8>::from_raw(rb as u64);
    let expected = ((ra as u128 * rb as u128) / (Q64::<8>::SCALE as u128)) as u64;
    assert_eq!((a * b).to_raw(), expected);
}

#[kani::proof]
fn verify_q64_commutativity() {
    let ra: u16 = kani::any();
    let rb: u16 = kani::any();
    kani::assume(ra < 10_000);
    kani::assume(rb < 10_000);
    let a = Q64::<8>::from_raw(ra as u64);
    let b = Q64::<8>::from_raw(rb as u64);
    assert_eq!((a + b).to_raw(), (b + a).to_raw());
    assert_eq!((a * b).to_raw(), (b * a).to_raw());
}
