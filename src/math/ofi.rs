use crate::page;
use crate::types::prelude::*;

page!(
    align(64) struct BookTop {
        bid_price: Q64,
        bid_qty: Q64,
        ask_price: Q64,
        ask_qty: Q64,
        title: Str<32>,
    }
);

#[inline(always)]
pub fn order_flow_imbalance(prev: BookTop, curr: BookTop) -> IQ64 {
    let curr_bq = curr.bid_qty.to_raw() as i64;
    let prev_bq = prev.bid_qty.to_raw() as i64;
    let curr_aq = curr.ask_qty.to_raw() as i64;
    let prev_aq = prev.ask_qty.to_raw() as i64;

    let bid_ge = (curr.bid_price >= prev.bid_price) as i64;
    let bid_le = (curr.bid_price <= prev.bid_price) as i64;
    let e_bid = (bid_ge * curr_bq) - (bid_le * prev_bq);

    let ask_le = (curr.ask_price <= prev.ask_price) as i64;
    let ask_ge = (curr.ask_price >= prev.ask_price) as i64;
    let e_ask = (ask_le * curr_aq) - (ask_ge * prev_aq);

    IQ64::from_raw(e_bid - e_ask)
}
