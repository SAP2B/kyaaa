use kyaaa::prelude::*;

#[test]
fn ofi_comprehensive() {
    let base = BookTop {
        bid_price: Q64::<8>::parse_ascii(b"100.0"),
        bid_qty: Q64::<8>::parse_ascii(b"10.0"),
        ask_price: Q64::<8>::parse_ascii(b"101.0"),
        ask_qty: Q64::<8>::parse_ascii(b"10.0"),
        title: Str::from_str("BTC/USDT"),
    };

    let t1 = BookTop {
        bid_price: Q64::<8>::parse_ascii(b"100.5"),
        bid_qty: Q64::<8>::parse_ascii(b"15.0"),
        ask_price: Q64::<8>::parse_ascii(b"101.0"),
        ask_qty: Q64::<8>::parse_ascii(b"10.0"),
        title: Str::from_str("BTC/USDT"),
    };
    assert_eq!(order_flow_imbalance(base, t1).to_raw(), 1_500_000_000);

    let t2 = BookTop {
        bid_price: Q64::<8>::parse_ascii(b"100.0"),
        bid_qty: Q64::<8>::parse_ascii(b"10.0"),
        ask_price: Q64::<8>::parse_ascii(b"100.8"),
        ask_qty: Q64::<8>::parse_ascii(b"12.0"),
        title: Str::from_str("BTC/USDT"),
    };
    assert_eq!(order_flow_imbalance(base, t2).to_raw(), -1_200_000_000);

    let t3 = BookTop {
        bid_price: Q64::<8>::parse_ascii(b"99.5"),
        bid_qty: Q64::<8>::parse_ascii(b"20.0"),
        ask_price: Q64::<8>::parse_ascii(b"101.0"),
        ask_qty: Q64::<8>::parse_ascii(b"10.0"),
        title: Str::from_str("BTC/USDT"),
    };
    assert_eq!(order_flow_imbalance(base, t3).to_raw(), -1_000_000_000);

    let t4 = BookTop {
        bid_price: Q64::<8>::parse_ascii(b"100.0"),
        bid_qty: Q64::<8>::parse_ascii(b"10.0"),
        ask_price: Q64::<8>::parse_ascii(b"101.5"),
        ask_qty: Q64::<8>::parse_ascii(b"5.0"),
        title: Str::from_str("BTC/USDT"),
    };
    assert_eq!(order_flow_imbalance(base, t4).to_raw(), 1_000_000_000);

    let t5 = BookTop {
        bid_price: Q64::<8>::parse_ascii(b"100.0"),
        bid_qty: Q64::<8>::parse_ascii(b"4.0"),
        ask_price: Q64::<8>::parse_ascii(b"101.0"),
        ask_qty: Q64::<8>::parse_ascii(b"18.0"),
        title: Str::from_str("BTC/USDT"),
    };
    assert_eq!(order_flow_imbalance(base, t5).to_raw(), -1_400_000_000);

    let t6 = BookTop {
        bid_price: Q64::<8>::parse_ascii(b"100.0"),
        bid_qty: Q64::<8>::parse_ascii(b"10.0"),
        ask_price: Q64::<8>::parse_ascii(b"101.0"),
        ask_qty: Q64::<8>::parse_ascii(b"10.0"),
        title: Str::from_str("BTC/USDT"),
    };
    assert_eq!(order_flow_imbalance(base, t6).to_raw(), 0);
}
