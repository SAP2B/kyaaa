// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

use kyaaa::Ring;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

#[test]
fn drop_on_push_failure() {
    static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[derive(Debug, PartialEq)]
    struct PanicOnDrop;
    impl Drop for PanicOnDrop {
        fn drop(&mut self) {
            DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    let ring = Ring::<PanicOnDrop, 2>::new();
    assert_eq!(ring.push(PanicOnDrop), Ok(()));
    assert_eq!(ring.push(PanicOnDrop), Ok(()));

    let err = ring.push(PanicOnDrop);
    assert!(err.is_err());
    drop(err);

    assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn consecutive_push_failures_state_integrity() {
    let ring = Ring::<i32, 2>::new();
    ring.push(10).unwrap();
    ring.push(20).unwrap();

    for _ in 0..100 {
        assert_eq!(ring.push(30), Err(30));
    }

    assert_eq!(ring.len(), 2);
    assert_eq!(ring.pop(), Some(10));
    assert_eq!(ring.pop(), Some(20));
    assert_eq!(ring.pop(), None);
}

#[test]
fn concurrent_empty_pop_spins() {
    let ring = std::sync::Arc::new(Ring::<usize, 4>::new());
    let ring_c = std::sync::Arc::clone(&ring);

    let consumer = std::thread::spawn(move || {
        let mut failed_pops = 0;
        while ring_c.len() == 0 {
            if ring_c.pop().is_none() {
                failed_pops += 1;
            }
            std::hint::spin_loop();
        }
        (failed_pops, ring_c.pop())
    });

    std::thread::sleep(std::time::Duration::from_millis(1));
    ring.push(999).unwrap();

    let (failures, item) = consumer.join().unwrap();
    assert!(failures > 0);
    assert_eq!(item, Some(999));
}

#[test]
fn index_overflow_wrap_around() {
    let ring = Ring::<i32, 4>::new();

    ring.head.0.store(usize::MAX - 1, Ordering::Relaxed);
    ring.tail.0.store(usize::MAX - 1, Ordering::Relaxed);

    assert_eq!(ring.push(10), Ok(()));
    assert_eq!(ring.push(20), Ok(()));
    assert_eq!(ring.push(30), Ok(()));
    assert_eq!(ring.push(40), Ok(()));
    assert!(ring.is_full());

    assert_eq!(ring.pop(), Some(10));
    assert_eq!(ring.pop(), Some(20));

    assert_eq!(ring.push(50), Ok(()));
    assert_eq!(ring.push(60), Ok(()));

    assert_eq!(ring.pop(), Some(30));
    assert_eq!(ring.pop(), Some(40));
    assert_eq!(ring.pop(), Some(50));
    assert_eq!(ring.pop(), Some(60));
    assert!(ring.is_empty());
}

#[test]
fn nested_heap_allocations() {
    let ring = Ring::<Vec<Box<usize>>, 4>::new();

    ring.push(vec![Box::new(1), Box::new(2)]).unwrap();
    ring.push(vec![Box::new(3)]).unwrap();

    let v1 = ring.pop().unwrap();
    assert_eq!(*v1[0], 1);
    assert_eq!(*v1[1], 2);

    let v2 = ring.pop().unwrap();
    assert_eq!(*v2[0], 3);

    assert_eq!(ring.pop(), None);
}

#[test]
fn strict_alignment_types() {
    #[repr(align(256))]
    #[derive(Debug, PartialEq)]
    struct OverAligned(usize);

    let ring = Ring::<OverAligned, 2>::new();

    ring.push(OverAligned(100)).unwrap();
    ring.push(OverAligned(200)).unwrap();

    assert_eq!(ring.pop(), Some(OverAligned(100)));
    assert_eq!(ring.pop(), Some(OverAligned(200)));
}

#[test]
fn scoped_thread_concurrency() {
    const CAP: usize = 16;
    const ITERS: usize = 10_000;

    let ring = Ring::<usize, CAP>::new();

    std::thread::scope(|s| {
        s.spawn(|| {
            for i in 0..ITERS {
                while ring.push(i).is_err() {
                    std::hint::spin_loop();
                }
            }
        });

        s.spawn(|| {
            for i in 0..ITERS {
                loop {
                    if let Some(val) = ring.pop() {
                        assert_eq!(val, i);
                        break;
                    }
                    std::hint::spin_loop();
                }
            }
        });
    });
}

#[test]
fn zst_pointer_arithmetic() {
    let ring = Ring::<(), 8>::new();

    for _ in 0..8 {
        assert_eq!(ring.push(()), Ok(()));
    }

    assert!(ring.is_full());

    for _ in 0..8 {
        assert_eq!(ring.pop(), Some(()));
    }

    assert!(ring.is_empty());
}

#[test]
fn concurrent_mutability() {
    let ring = Ring::<Vec<usize>, 4>::new();

    std::thread::scope(|s| {
        s.spawn(|| {
            for i in 0..1_000 {
                let mut v = Vec::with_capacity(5);
                v.push(i);
                while let Err(ret) = ring.push(v) {
                    v = ret;
                    std::hint::spin_loop();
                }
            }
        });

        s.spawn(|| {
            for i in 0..1_000 {
                loop {
                    if let Some(mut v) = ring.pop() {
                        assert_eq!(v[0], i);
                        v.push(i * 2);
                        assert_eq!(v.len(), 2);
                        break;
                    }
                    std::hint::spin_loop();
                }
            }
        });
    });
}

#[test]
fn spsc_with_heap_allocated_strings() {
    const CAP: usize = 32;
    const TOTAL: usize = 200_000;

    let ring = Arc::new(Ring::<String, CAP>::new());
    let prod_ring = Arc::clone(&ring);
    let cons_ring = Arc::clone(&ring);

    let producer = thread::spawn(move || {
        for i in 0..TOTAL {
            let s = format!("payload_data_item_{}", i);
            let mut val = s;
            while let Err(unsent) = prod_ring.push(val) {
                val = unsent;
                thread::yield_now();
            }
        }
    });

    let consumer = thread::spawn(move || {
        for i in 0..TOTAL {
            let expected = format!("payload_data_item_{}", i);
            loop {
                if let Some(val) = cons_ring.pop() {
                    assert_eq!(val, expected);
                    break;
                }
                thread::yield_now();
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

#[test]
fn drop_counter_exact_lifecycles() {
    static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[derive(Debug)]
    struct DropDetector(#[allow(dead_code)] usize);
    impl Drop for DropDetector {
        fn drop(&mut self) {
            DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    let ring = Ring::<DropDetector, 8>::new();
    const ITEMS: usize = 1_000;

    for i in 0..ITEMS {
        ring.push(DropDetector(i)).unwrap();
        let item = ring.pop().unwrap();
        drop(item);
    }

    assert_eq!(DROP_COUNT.load(Ordering::SeqCst), ITEMS);
}

#[test]
fn zero_sized_types() {
    let ring = Ring::<(), 16>::new();
    for _ in 0..16 {
        assert_eq!(ring.push(()), Ok(()));
    }
    assert!(ring.is_full());
    for _ in 0..16 {
        assert_eq!(ring.pop(), Some(()));
    }
    assert!(ring.is_empty());
}

#[test]
fn spsc_asymmetric_latencies() {
    const CAP: usize = 8;
    const TOTAL: usize = 50_000;

    let ring = Arc::new(Ring::<u64, CAP>::new());
    let prod_ring = Arc::clone(&ring);
    let cons_ring = Arc::clone(&ring);

    let producer = thread::spawn(move || {
        for i in 0..TOTAL {
            let mut val = i as u64;
            let mut spins = 0;
            while let Err(unsent) = prod_ring.push(val) {
                val = unsent;
                spins += 1;
                if spins % 100 == 0 {
                    thread::sleep(Duration::from_nanos(10));
                } else {
                    thread::yield_now();
                }
            }
        }
    });

    let consumer = thread::spawn(move || {
        let mut expected = 0u64;
        while expected < TOTAL as u64 {
            if let Some(val) = cons_ring.pop() {
                assert_eq!(val, expected);
                expected += 1;
            } else {
                thread::yield_now();
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

#[test]
fn initial_state() {
    let ring = Ring::<i32, 8>::new();
    assert!(ring.is_empty());
    assert!(!ring.is_full());
    assert_eq!(ring.len(), 0);
    assert_eq!(ring.capacity(), 8);
}

#[test]
fn single_element_capacity() {
    let ring = Ring::<i32, 1>::new();
    assert!(ring.is_empty());
    assert_eq!(ring.push(42), Ok(()));
    assert!(ring.is_full());
    assert_eq!(ring.push(100), Err(100));
    assert_eq!(ring.pop(), Some(42));
    assert!(ring.is_empty());
}

#[test]
fn push_pop_fifo_order() {
    let ring = Ring::<i32, 8>::new();
    for i in 0..8 {
        assert_eq!(ring.push(i), Ok(()));
    }
    for i in 0..8 {
        assert_eq!(ring.pop(), Some(i));
    }
    assert_eq!(ring.pop(), None);
}

#[test]
fn fill_and_empty_cycles() {
    let ring = Ring::<usize, 4>::new();
    for cycle in 0..10 {
        for i in 0..4 {
            assert_eq!(ring.push(cycle * 10 + i), Ok(()));
        }
        assert!(ring.is_full());
        assert_eq!(ring.push(999), Err(999));
        for i in 0..4 {
            assert_eq!(ring.pop(), Some(cycle * 10 + i));
        }
        assert!(ring.is_empty());
    }
}

#[test]
fn interleaved_push_pop() {
    let ring = Ring::<i32, 4>::new();
    assert_eq!(ring.push(1), Ok(()));
    assert_eq!(ring.push(2), Ok(()));
    assert_eq!(ring.pop(), Some(1));
    assert_eq!(ring.push(3), Ok(()));
    assert_eq!(ring.push(4), Ok(()));
    assert_eq!(ring.pop(), Some(2));
    assert_eq!(ring.pop(), Some(3));
    assert_eq!(ring.push(5), Ok(()));
    assert_eq!(ring.pop(), Some(4));
    assert_eq!(ring.pop(), Some(5));
    assert_eq!(ring.pop(), None);
}

#[test]
fn large_wrap_around() {
    let ring = Ring::<usize, 16>::new();
    for i in 0..100_000 {
        assert_eq!(ring.push(i), Ok(()));
        assert_eq!(ring.pop(), Some(i));
    }
    assert!(ring.is_empty());
}

#[test]
fn heap_allocated_types() {
    let ring = Ring::<Vec<u8>, 4>::new();
    assert_eq!(ring.push(vec![1, 2, 3]), Ok(()));
    assert_eq!(ring.push(vec![4, 5, 6]), Ok(()));
    assert_eq!(ring.pop(), Some(vec![1, 2, 3]));
    assert_eq!(ring.pop(), Some(vec![4, 5, 6]));
    assert_eq!(ring.pop(), None);
}

#[test]
fn custom_struct_type() {
    #[derive(Debug, PartialEq)]
    struct Custom {
        id: u64,
        data: String,
    }

    let ring = Ring::<Custom, 2>::new();
    let c1 = Custom {
        id: 1,
        data: "a".into(),
    };
    let c2 = Custom {
        id: 2,
        data: "b".into(),
    };

    assert_eq!(ring.push(c1), Ok(()));
    assert_eq!(ring.push(c2), Ok(()));
    assert_eq!(
        ring.pop(),
        Some(Custom {
            id: 1,
            data: "a".into()
        })
    );
    assert_eq!(
        ring.pop(),
        Some(Custom {
            id: 2,
            data: "b".into()
        })
    );
}

#[test]
fn len_and_is_empty_tracking() {
    let ring = Ring::<i32, 8>::new();
    assert_eq!(ring.len(), 0);
    assert!(ring.is_empty());

    ring.push(10).unwrap();
    assert_eq!(ring.len(), 1);
    assert!(!ring.is_empty());

    ring.push(20).unwrap();
    assert_eq!(ring.len(), 2);

    ring.pop().unwrap();
    assert_eq!(ring.len(), 1);

    ring.pop().unwrap();
    assert_eq!(ring.len(), 0);
    assert!(ring.is_empty());
}

#[test]
fn is_full_tracking() {
    let ring = Ring::<i32, 2>::new();
    assert!(!ring.is_full());
    ring.push(1).unwrap();
    assert!(!ring.is_full());
    ring.push(2).unwrap();
    assert!(ring.is_full());
    ring.pop().unwrap();
    assert!(!ring.is_full());
}

#[test]
fn spsc_stress_high_volume() {
    const CAP: usize = 128;
    const TOTAL: usize = 1_000_000;

    let ring = Arc::new(Ring::<usize, CAP>::new());
    let producer_ring = Arc::clone(&ring);
    let consumer_ring = Arc::clone(&ring);

    let producer = thread::spawn(move || {
        for i in 0..TOTAL {
            let mut val = i;
            while let Err(unsent) = producer_ring.push(val) {
                val = unsent;
                thread::yield_now();
            }
        }
    });

    let consumer = thread::spawn(move || {
        let mut count = 0;
        let mut expected = 0;
        while count < TOTAL {
            if let Some(val) = consumer_ring.pop() {
                assert_eq!(val, expected);
                expected += 1;
                count += 1;
            } else {
                thread::yield_now();
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

#[test]
fn spsc_burst_transfers() {
    const CAP: usize = 16;
    const BURSTS: usize = 1_000;

    let ring = Arc::new(Ring::<usize, CAP>::new());
    let producer_ring = Arc::clone(&ring);
    let consumer_ring = Arc::clone(&ring);

    let producer = thread::spawn(move || {
        let mut item = 0;
        for _ in 0..BURSTS {
            for _ in 0..CAP {
                while producer_ring.push(item).is_err() {
                    thread::yield_now();
                }
                item += 1;
            }
        }
    });

    let consumer = thread::spawn(move || {
        let mut received = Vec::with_capacity(CAP * BURSTS);
        while received.len() < CAP * BURSTS {
            if let Some(val) = consumer_ring.pop() {
                received.push(val);
            } else {
                thread::yield_now();
            }
        }
        received
    });

    producer.join().unwrap();
    let res = consumer.join().unwrap();
    let expected: Vec<usize> = (0..(CAP * BURSTS)).collect();
    assert_eq!(res, expected);
}
