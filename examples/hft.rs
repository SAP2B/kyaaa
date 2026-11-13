#![no_main]
#![no_std]

use kyaaa::prelude::*;

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}

page!(
    align(64) struct MarketTick {
        price: u64,
        volume: u64,
        timestamp: u64,
        id: u8,
        symbol: Str<39>,
    }

    align(64) struct OrderStatus {
        id: u8,
        order_id: Str<63>,
        status: Str<64>,
    }
);

book!(
    pub HFT_BOOK {
        best_bid: MarketTick = MarketTick {
            id: 1,
            symbol: Str::from_str("BTCUSDT"),
            price: 9500000,
            volume: 100,
            timestamp: 0,
        },
        execution: OrderStatus = OrderStatus {
            id: 2,
            order_id: Str::from_str("ORD-INIT"),
            status: Str::from_str("IDLE"),
        },
    }
);

static TICK_RING: Ring<u8, 256> = Ring::new();

kmain!(
    align(64);

    fn kmain(_sp: *const usize) -> i32 {
        clock!({
            let feed = book!(
                tick: MarketTick = MarketTick {
                    id: 1,
                    symbol: Str::from_str("BTCUSDT"),
                    price: 9501500,
                    volume: 250,
                    timestamp: 1776512345,
                },
                order: OrderStatus = OrderStatus {
                    id: 2,
                    order_id: Str::from_str("ORD-77788"),
                    status: Str::from_str("MATCHED"),
                }
            );

            feed.tick().set().price(9501800);
            feed.order().set().status("FILLED");

            let (mut producer, mut consumer) = TICK_RING.split();

            producer.spin(|slot| {
                slot.write(feed.tick().id());
            });

            consumer.eat(|&received_id| {
                if received_id == 1 {
                    HFT_BOOK.best_bid().set()
                        .symbol("BTCUSDT-UPD")
                        .price(9501800)
                        .volume(250)
                        .timestamp(1776512345);
                }
            });

            HFT_BOOK.execution().set().status("PROCESSED");

            black_box((
                feed.tick().get(),
                feed.order().get(),
                HFT_BOOK.best_bid().get(),
                HFT_BOOK.execution().get(),
            ));
        });

        0
    }
);
