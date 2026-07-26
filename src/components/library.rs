//PDX short identifier: AGPL-3.0
//Copyright (C) 2026 SAP2B

use super::book::*;

pub struct Library<const N: usize, const CAP: usize = 256> {
    pub book: Book<N>,
    pub buf: [u8; CAP],
    pub head: usize,
    pub tail: usize,
}

impl<const N: usize, const CAP: usize> Library<N, CAP> {
    #[inline(always)]
    pub const fn push(&mut self, id: u8) {
        self.buf[self.tail] = id;
        self.tail = (self.tail + 1) % CAP;
    }

    #[inline(always)]
    pub const fn pop<T>(&mut self) -> Option<&'static T> {
        if self.head == self.tail {
            return None;
        }
        let id = self.buf[self.head];
        self.head = (self.head + 1) % CAP;
        Some(self.book.get::<T>(id))
    }
}

#[macro_export]
macro_rules! library {
    ($($id:expr => $val:expr),* $(,)?) => {
        Library {
            book: $crate::book!($($id => $val),*),
            buf: [0u8; 256],
            head: 0,
            tail: 0,
        }
    };
}

const _: () = {
    let mut lib = library!(
        0 => 100_i32,
        1 => 200_i32,
    );

    lib.push(0);
    lib.push(1);

    match lib.pop::<i32>() {
        Some(val) => assert!(*val == 100),
        None => panic!("Error FIFO"),
    }

    match lib.pop::<i32>() {
        Some(val) => assert!(*val == 200),
        None => panic!("Error FIFO"),
    }

    assert!(lib.pop::<i32>().is_none());
};
