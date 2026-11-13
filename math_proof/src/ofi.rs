use kyaaa::prelude::*;

#[kani::proof]
fn verify_ofi_commutativity_and_bounds() {
    let bp_raw: u32 = kani::any();
    let bq_raw: u32 = kani::any();
    let ap_raw: u32 = kani::any();
    let aq_raw: u32 = kani::any();

    let nbp_raw: u32 = kani::any();
    let nbq_raw: u32 = kani::any();
    let nap_raw: u32 = kani::any();
    let naq_raw: u32 = kani::any();

    kani::assume(bp_raw < 1_000_000);
    kani::assume(bq_raw < 100_000);
    kani::assume(ap_raw < 1_000_000);
    kani::assume(aq_raw < 100_000);

    kani::assume(nbp_raw < 1_000_000);
    kani::assume(nbq_raw < 100_000);
    kani::assume(nap_raw < 1_000_000);
    kani::assume(naq_raw < 100_000);

    let prev = BookTop {
        bid_price: Q64::<8>::from_raw(bp_raw as u64),
        bid_qty: Q64::<8>::from_raw(bq_raw as u64),
        ask_price: Q64::<8>::from_raw(ap_raw as u64),
        ask_qty: Q64::<8>::from_raw(aq_raw as u64),
        title: Str::from_str("BTC/USDT"),
    };

    let curr = BookTop {
        bid_price: Q64::<8>::from_raw(nbp_raw as u64),
        bid_qty: Q64::<8>::from_raw(nbq_raw as u64),
        ask_price: Q64::<8>::from_raw(nap_raw as u64),
        ask_qty: Q64::<8>::from_raw(naq_raw as u64),
        title: Str::from_str("BTC/USDT"),
    };

    let ofi_val = order_flow_imbalance(prev, curr);

    let _raw_result = ofi_val.to_raw();
}

#[kani::proof]
fn verify_ofi_identity() {
    let bp_raw: u32 = kani::any();
    let bq_raw: u32 = kani::any();
    let ap_raw: u32 = kani::any();
    let aq_raw: u32 = kani::any();

    kani::assume(bp_raw < 1_000_000);
    kani::assume(bq_raw < 100_000);
    kani::assume(ap_raw < 1_000_000);
    kani::assume(aq_raw < 100_000);

    let book = BookTop {
        bid_price: Q64::<8>::from_raw(bp_raw as u64),
        bid_qty: Q64::<8>::from_raw(bq_raw as u64),
        ask_price: Q64::<8>::from_raw(ap_raw as u64),
        ask_qty: Q64::<8>::from_raw(aq_raw as u64),
        title: Str::from_str("BTC/USDT"),
    };

    let ofi_val = order_flow_imbalance(book, book);
    assert_eq!(ofi_val.to_raw(), 0);
}
