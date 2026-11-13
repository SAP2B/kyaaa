use kyaaa::prelude::*;

#[test]
fn q64_parsing() {
    assert_eq!(Q64::<8>::parse_ascii(b"").to_raw(), 0);
    assert_eq!(Q64::<8>::parse_ascii(b"123.abc").to_raw(), 0);
    assert_eq!(Q64::<8>::parse_ascii(b"42").to_raw(), 4_200_000_000);
    assert_eq!(Q64::<8>::parse_ascii(b"0.00000001").to_raw(), 1);
    assert_eq!(
        Q64::<8>::parse_ascii(b"123.45678901").to_raw(),
        12_345_678_901
    );
}

#[test]
fn iq64_parsing() {
    assert_eq!(IQ64::<8>::parse_ascii(b"-10").to_raw(), -1_000_000_000);
    assert_eq!(IQ64::<8>::parse_ascii(b"+10").to_raw(), 1_000_000_000);
    assert_eq!(
        IQ64::<8>::parse_ascii(b"-1234.5678").to_raw(),
        -123_456_780_000
    );
    assert_eq!(IQ64::<8>::parse_ascii(b"-abc").to_raw(), 0);
}

#[test]
fn q64_arithmetic() {
    let a = Q64::<8>::from_integer(10);
    let b = Q64::<8>::parse_ascii(b"2.5");

    assert_eq!((a + b).to_raw(), 1_250_000_000);
    assert_eq!((a - b).to_raw(), 750_000_000);
    assert_eq!((a * b).to_raw(), 2_500_000_000);
    assert_eq!((a / b).to_raw(), 400_000_000);
    assert_eq!((a / Q64::<8>::ZERO).to_raw(), 0);
}

#[test]
fn iq64_arithmetic() {
    let ia = IQ64::<8>::from_integer(-5);
    let ib = IQ64::<8>::parse_ascii(b"2.5");

    assert_eq!((ia + ib).to_raw(), -250_000_000);
    assert_eq!((ia - ib).to_raw(), -750_000_000);
    assert_eq!((ia * ib).to_raw(), -1_250_000_000);
    assert_eq!((ia / ib).to_raw(), -200_000_000);
    assert_eq!((-ia).to_raw(), 500_000_000);
}

#[test]
fn compound_assignments() {
    let a = Q64::<8>::from_integer(10);
    let b = Q64::<8>::parse_ascii(b"2.5");

    let mut val = a;
    val += b;
    assert_eq!(val.to_raw(), 1_250_000_000);

    val -= b;
    assert_eq!(val.to_raw(), 1_000_000_000);

    val *= b;
    assert_eq!(val.to_raw(), 2_500_000_000);

    val /= b;
    assert_eq!(val.to_raw(), 1_000_000_000);
}
