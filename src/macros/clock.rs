#[macro_export]
macro_rules! clock {
    ($body:block) => {{
        use core::sync::atomic::{AtomicU64, Ordering};

        static TSC_HZ: AtomicU64 = AtomicU64::new(0);

        let mut tsc_hz = TSC_HZ.load(Ordering::Relaxed);
        if tsc_hz == 0 {
            #[repr(C)]
            struct Timespec {
                tv_sec: i64,
                tv_nsec: i64,
            }

            let mut start_time = Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            let mut end_time = Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };

            let _ = clock_gettime(1, &mut start_time as *mut Timespec as *mut u8);
            let start_tsc = unsafe { core::arch::x86_64::_rdtsc() };

            loop {
                let _ = clock_gettime(1, &mut end_time as *mut Timespec as *mut u8);
                let elapsed_ns = (end_time.tv_sec - start_time.tv_sec) * 1_000_000_000
                    + (end_time.tv_nsec - start_time.tv_nsec);
                if elapsed_ns >= 5_000_000 {
                    break;
                }
            }
            let end_tsc = unsafe { core::arch::x86_64::_rdtsc() };

            let elapsed_ns = (end_time.tv_sec - start_time.tv_sec) * 1_000_000_000
                + (end_time.tv_nsec - start_time.tv_nsec);
            let delta_cycles = end_tsc - start_tsc;

            tsc_hz = (delta_cycles as u128 * 1_000_000_000 / elapsed_ns as u128) as u64;
            TSC_HZ.store(tsc_hz, Ordering::Relaxed);
        }

        let start = unsafe { core::arch::x86_64::_rdtsc() };
        let _res = $body;
        let end = unsafe { core::arch::x86_64::_rdtsc() };
        let cycles = (end - start) as usize;

        let ns = (cycles as u128 * 1_000_000_000 / tsc_hz as u128) as usize;

        let ghz_int = tsc_hz / 1_000_000_000;
        let ghz_rem = (tsc_hz % 1_000_000_000) / 100_000_000;

        let mut out_buf = [0_u8; 512];
        let mut pos = 0;

        let mut write_str = |s: &[u8], buf: &mut [u8], p: &mut usize| {
            buf[*p..*p + s.len()].copy_from_slice(s);
            *p += s.len();
        };

        let mut write_num = |mut n: usize, buf: &mut [u8], p: &mut usize| {
            let mut temp = [0_u8; 32];
            let mut i = temp.len();
            if n == 0 {
                i -= 1;
                temp[i] = b'0';
            } else {
                while n > 0 {
                    i -= 1;
                    temp[i] = b'0' + (n % 10) as u8;
                    n /= 10;
                }
            }
            let slice = &temp[i..];
            buf[*p..*p + slice.len()].copy_from_slice(slice);
            *p += slice.len();
        };

        write_str(b"[ CLOCK | ", &mut out_buf, &mut pos);
        write_num(ghz_int as usize, &mut out_buf, &mut pos);
        if ghz_rem > 0 {
            write_str(b".", &mut out_buf, &mut pos);
            write_num(ghz_rem as usize, &mut out_buf, &mut pos);
        }
        write_str(b" GHz | ", &mut out_buf, &mut pos);
        write_num(cycles, &mut out_buf, &mut pos);
        write_str(b" cycles | ", &mut out_buf, &mut pos);
        write_num(ns, &mut out_buf, &mut pos);
        write_str(b" ns ]\n", &mut out_buf, &mut pos);

        let _ = write(1, out_buf.as_ptr(), pos);

        _res
    }};
    ($expr:expr) => {{ $crate::clock!({ $expr }) }};
}
