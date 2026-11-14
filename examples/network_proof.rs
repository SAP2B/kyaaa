#![no_main]
#![no_std]

use kyaaa::prelude::*;

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}

page!(
    align(64) struct DecodeBuffer {
        data: [u8; 256],
    }
);

kmain!(
    align(64);

    fn kmain(_sp: *const usize) -> i32 {
        let raw_stream = &[
            0x80,
            0x40, 0x00,
            0x20, 0x00, 0x00,
            0x00,
        ];

        let mut out = DecodeBuffer { data: [0u8; 256] };

            let stream = Stream::new(raw_stream);
            let mut pos = 0;
            let mut count = 0;

            while let Some((token, next_pos)) = stream.decode(pos) {
                pos = next_pos;
                out.data[count] = match token {
                    DecodedToken::T8(_) => 8,
                    DecodedToken::T16(_) => 16,
                    DecodedToken::T32(_) => 32,
                };
                count += 1;
            }

            black_box(count);

        0
    }
);
