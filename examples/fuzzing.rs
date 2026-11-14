#![no_main]
#![no_std]

use kyaaa::prelude::*;

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}

page!(
    align(2)  struct A01 { dst: [u8; 6], src: [u8; 6], eth_type: u16 }
    align(4)  struct A02 { ver_ihl: u8, tos: u8, total_len: u16, ident: u16, flags_frag: u16, ttl: u8, proto: u8, checksum: u16, src: [u8; 4], dst: [u8; 4] }
    align(4)  struct A03 { sport: u16, dport: u16, seq: u32, ack: u32, flags: u16, window: u16, checksum: u16, urgent: u16 }
    align(2)  struct A04 { sport: u16, dport: u16, len: u16, checksum: u16 }
    align(8)  struct A05 { id: u64, ts: u64, state: u32, flags: u32 }
    align(8)  struct A06 { price: u64, volume: u64, ts: u64 }
    align(8)  struct A07 { id: u64, price: u64, qty: u64, side: Str<8> }
    align(8)  struct A08 { order_id: u64, price: u64, qty: u64, ts: u64 }
    align(8)  struct A09 { order_id: u64, reason: Str<8> }
    align(8)  struct A10 { bid: u64, ask: u64, bid_sz: u64, ask_sz: u64 }
    align(8)  struct A11 { sym_id: u32, qty: i32, avg_px: u64, pnl: i64 }
    align(8)  struct A12 { id: u64, limit: u64 }
    align(8)  struct A13 { order_id: u64, price: u64, qty: u64, ts: u64, flags: u32, pad: u32 }
    align(4)  struct A14 { id: u32, hp: u16, mp: u16, level: u8, class: u8, reserved: [u8; 2] }
    align(4)  struct A15 { id: u32, hp: u32, damage: u32 }
    align(4)  struct A16 { id: u32, count: u16, quality: u8, flag: u8 }
    align(4)  struct A17 { x: i32, y: i32, vx: i32, vy: i32, owner: u32 }
    align(4)  struct A18 { id: u32, cooldown: u32, mana: u32 }
    align(4)  struct A19 { id: u32, state: u8, progress: u8, reserved: [u8; 2] }
    align(8)  struct A20 { sender: u32, len: u16, kind: u8, reserved: u8, text: Str<56> }
    align(8)  struct A21 { owner: u32, slots: u32, used: u32, cap: u32 }
    align(4)  struct A22 { x: i32, y: i32 }
    align(4)  struct A23 { x: i32, y: i32, z: i32 }
    align(16) struct A24 { x: i32, y: i32, z: i32, w: i32 }
    align(4)  struct A25 { m: [i32; 9] }
    align(16) struct A26 { m: [i32; 16] }
    align(4)  struct A27 { min: [i32; 3], max: [i32; 3] }
    align(4)  struct A28 { ox: i32, oy: i32, oz: i32, dx: i32, dy: i32, dz: i32 }
    align(4)  struct A29 { pid: u32, ppid: u32, uid: u32, state: u32 }
    align(8)  struct A30 { tid: u64, state: u32, prio: u32 }
    align(4)  struct A31 { fd: u32, family: u16, sock_type: u16, proto: u16, reserved: u16, timeout: u32 }
    align(8)  struct A32 { fd: u32, flags: u32, mode: u16, pad: u16, pad2: u32, offset: u64 }
    align(8)  struct A33 { base: u64, size: u64, prot: u32, flags: u32 }
    align(4)  struct A34 { id: u32, name: Str<28> }
    align(4)  struct A35 { id: u16, len: u16, text: Str<60> }
    align(1)  struct A36 { n: Str<64> }
    align(1)  struct A37 { n: Str<16> }
    align(1)  struct A38 { n: Str<48> }
    align(1)  struct A39 { v: u8 }
    align(2)  struct A40 { v: u16 }
    align(4)  struct A41 { v: u32 }
    align(8)  struct A42 { v: u64 }
    align(4)  struct A43 { a: u16, b: u16 }
    align(8)  struct A44 { a: u32, b: u32 }
    align(16) struct A45 { a: u64, b: u64 }
    align(64) struct A46 { a: [u8; 64] }
    align(4)  struct A47 { a: u32, b: u32, c: u32, d: u32 }
    align(8)  struct A48 { a: u64, b: u64, c: u64, d: u64 }
    align(4)  struct A49 { a: u32, b: u16, c: u8, d: u8 }
    align(8)  struct A50 { a: u64, b: u32, c: u16, d: u8, e: u8 }
);

kmain!(
    align(64);

    fn kmain(_sp: *const usize) -> i32 {
        let hlist = dbook!(
            a01: A01 = A01 { dst: [0; 6], src: [0; 6], eth_type: 0x0800 },
            a02: A02 = A02 { ver_ihl: 0x45, tos: 0, total_len: 0, ident: 0, flags_frag: 0, ttl: 64, proto: 6, checksum: 0, src: [0; 4], dst: [0; 4] },
            a03: A03 = A03 { sport: 0, dport: 0, seq: 0, ack: 0, flags: 0, window: 0, checksum: 0, urgent: 0 },
            a04: A04 = A04 { sport: 0, dport: 0, len: 0, checksum: 0 },
            a05: A05 = A05 { id: 0, ts: 0, state: 0, flags: 0 },
            a06: A06 = A06 { price: 0, volume: 0, ts: 0 },
            a07: A07 = A07 { id: 0, price: 0, qty: 0, side: Str::from_str("BUY") },
            a08: A08 = A08 { order_id: 0, price: 0, qty: 0, ts: 0 },
            a09: A09 = A09 { order_id: 0, reason: Str::from_str("NONE") },
            a10: A10 = A10 { bid: 0, ask: 0, bid_sz: 0, ask_sz: 0 },
            a11: A11 = A11 { sym_id: 0, qty: 0, avg_px: 0, pnl: 0 },
            a12: A12 = A12 { id: 0, limit: 0 },
            a13: A13 = A13 { order_id: 0, price: 0, qty: 0, ts: 0, flags: 0, pad: 0 },
            a14: A14 = A14 { id: 0, hp: 0, mp: 0, level: 0, class: 0, reserved: [0; 2] },
            a15: A15 = A15 { id: 0, hp: 0, damage: 0 },
            a16: A16 = A16 { id: 0, count: 0, quality: 0, flag: 0 },
            a17: A17 = A17 { x: 0, y: 0, vx: 0, vy: 0, owner: 0 },
            a18: A18 = A18 { id: 0, cooldown: 0, mana: 0 },
            a19: A19 = A19 { id: 0, state: 0, progress: 0, reserved: [0; 2] },
            a20: A20 = A20 { sender: 0, len: 0, kind: 0, reserved: 0, text: Str::from_str("") },
            a21: A21 = A21 { owner: 0, slots: 0, used: 0, cap: 0 },
            a22: A22 = A22 { x: 0, y: 0 },
            a23: A23 = A23 { x: 0, y: 0, z: 0 },
            a24: A24 = A24 { x: 0, y: 0, z: 0, w: 0 },
            a25: A25 = A25 { m: [0; 9] },
            a26: A26 = A26 { m: [0; 16] },
            a27: A27 = A27 { min: [0; 3], max: [0; 3] },
            a28: A28 = A28 { ox: 0, oy: 0, oz: 0, dx: 0, dy: 0, dz: 0 },
            a29: A29 = A29 { pid: 0, ppid: 0, uid: 0, state: 0 },
            a30: A30 = A30 { tid: 0, state: 0, prio: 0 },
            a31: A31 = A31 { fd: 0, family: 0, sock_type: 0, proto: 0, reserved: 0, timeout: 0 },
            a32: A32 = A32 { fd: 0, flags: 0, mode: 0, pad: 0, pad2: 0, offset: 0 },
            a33: A33 = A33 { base: 0, size: 0, prot: 0, flags: 0 },
            a34: A34 = A34 { id: 0, name: Str::from_str("") },
            a35: A35 = A35 { id: 0, len: 0, text: Str::from_str("") },
            a36: A36 = A36 { n: Str::from_str("") },
            a37: A37 = A37 { n: Str::from_str("") },
            a38: A38 = A38 { n: Str::from_str("") },
            a39: A39 = A39 { v: 0 },
            a40: A40 = A40 { v: 0 },
            a41: A41 = A41 { v: 0 },
            a42: A42 = A42 { v: 0 },
            a43: A43 = A43 { a: 0, b: 0 },
            a44: A44 = A44 { a: 0, b: 0 },
            a45: A45 = A45 { a: 0, b: 0 },
            a46: A46 = A46 { a: [0; 64] },
            a47: A47 = A47 { a: 0, b: 0, c: 0, d: 0 },
            a48: A48 = A48 { a: 0, b: 0, c: 0, d: 0 },
            a49: A49 = A49 { a: 0, b: 0, c: 0, d: 0 },
            a50: A50 = A50 { a: 0, b: 0, c: 0, d: 0, e: 0 },
        );

        clock!({
            hlist.a01().set().eth_type(0x86DD);
            hlist.a02().set().ttl(128);
            hlist.a03().set().window(65535);
            hlist.a04().set().len(512);
            hlist.a05().set().state(1);
            hlist.a06().set().price(9500000);
            hlist.a07().set().side("SELL");
            hlist.a08().set().qty(100);
            hlist.a09().set().reason("USER");
            hlist.a10().set().bid(9500000);
            hlist.a11().set().pnl(1500);
            hlist.a12().set().limit(100000);
            hlist.a13().set().flags(1);
            hlist.a14().set().hp(100);
            hlist.a15().set().damage(50);
            hlist.a16().set().count(10);
            hlist.a17().set().owner(1);
            hlist.a18().set().cooldown(500);
            hlist.a19().set().state(1);
            hlist.a20().set().text("hello");
            hlist.a21().set().cap(40);
            hlist.a22().set().x(100);
            hlist.a23().set().z(100);
            hlist.a24().set().w(1);
            hlist.a25().set().m([1; 9]);
            hlist.a26().set().m([1; 16]);
            hlist.a27().set().min([-1; 3]);
            hlist.a28().set().dx(1);
            hlist.a29().set().pid(1);
            hlist.a30().set().prio(10);
            hlist.a31().set().timeout(30);
            hlist.a32().set().offset(4096);
            hlist.a33().set().size(1 << 20);
            hlist.a34().set().name("admin");
            hlist.a35().set().text("token");
            hlist.a36().set().n("/usr/bin");
            hlist.a37().set().n("BTCUSDT");
            hlist.a38().set().n("a@b.c");
            hlist.a39().set().v(42);
            hlist.a40().set().v(42);
            hlist.a41().set().v(42);
            hlist.a42().set().v(42);
            hlist.a43().set().a(1);
            hlist.a44().set().a(1);
            hlist.a45().set().a(1);
            hlist.a46().set().a([1; 64]);
            hlist.a47().set().a(1);
            hlist.a48().set().a(1);
            hlist.a49().set().a(1);
            hlist.a50().set().a(1);

            black_box(hlist.a01().get());
            black_box(hlist.a02().get());
            black_box(hlist.a03().get());
            black_box(hlist.a04().get());
            black_box(hlist.a05().get());
            black_box(hlist.a06().get());
            black_box(hlist.a07().get());
            black_box(hlist.a08().get());
            black_box(hlist.a09().get());
            black_box(hlist.a10().get());
            black_box(hlist.a11().get());
            black_box(hlist.a12().get());
            black_box(hlist.a13().get());
            black_box(hlist.a14().get());
            black_box(hlist.a15().get());
            black_box(hlist.a16().get());
            black_box(hlist.a17().get());
            black_box(hlist.a18().get());
            black_box(hlist.a19().get());
            black_box(hlist.a20().get());
            black_box(hlist.a21().get());
            black_box(hlist.a22().get());
            black_box(hlist.a23().get());
            black_box(hlist.a24().get());
            black_box(hlist.a25().get());
            black_box(hlist.a26().get());
            black_box(hlist.a27().get());
            black_box(hlist.a28().get());
            black_box(hlist.a29().get());
            black_box(hlist.a30().get());
            black_box(hlist.a31().get());
            black_box(hlist.a32().get());
            black_box(hlist.a33().get());
            black_box(hlist.a34().get());
            black_box(hlist.a35().get());
            black_box(hlist.a36().get());
            black_box(hlist.a37().get());
            black_box(hlist.a38().get());
            black_box(hlist.a39().get());
            black_box(hlist.a40().get());
            black_box(hlist.a41().get());
            black_box(hlist.a42().get());
            black_box(hlist.a43().get());
            black_box(hlist.a44().get());
            black_box(hlist.a45().get());
            black_box(hlist.a46().get());
            black_box(hlist.a47().get());
            black_box(hlist.a48().get());
            black_box(hlist.a49().get());
            black_box(hlist.a50().get());
        });

        0
    }
);
