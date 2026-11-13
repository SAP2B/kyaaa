#![no_main]
#![no_std]

use kyaaa::prelude::*;

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}

book!(
    pub MarketState {
        btcusdt: BookTop = BookTop {
            bid_price: Q64::from_raw(0),
            bid_qty: Q64::from_raw(0),
            ask_price: Q64::from_raw(0),
            ask_qty: Q64::from_raw(0),
            title: Str::from_str("BTC/USDT")
        }
    }
);

kmain!(
    align(64);

    fn kmain(_sp: *const usize) -> i32 {
        clock!({
            let new_tick = BookTop {
                bid_price: Q64::from_raw(5000050000000),
                bid_qty: Q64::from_raw(200000000),
                ask_price: Q64::from_raw(5000100000000),
                ask_qty: Q64::from_raw(100000000),
                title: Str::from_str("BTC/USDT")
            };

            let prev_snapshot = MarketState.btcusdt().get();

            MarketState.btcusdt().set().bid_price(new_tick.bid_price).bid_qty(new_tick.bid_qty);

            let ofi_result = order_flow_imbalance(*prev_snapshot, new_tick);

            black_box((
                ofi_result,
                MarketState.btcusdt().get(),
            ));
        });

        0
    }
);
