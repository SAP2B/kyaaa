//PDX short identifier: AGPL-3.0
//Copyright (C) 2026 SAP2B

use super::book::*;
use core::arch::asm;

#[inline(always)]
pub unsafe fn sys_write(fd: i32, buf: *const u8, count: usize) -> isize {
    let ret: isize;
    asm!(
        "syscall",
        in("rax") 1isize,
        in("rdi") fd as isize,
        in("rsi") buf,
        in("rdx") count,
        lateout("rax") ret,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

pub struct Library<const N: usize, const CAP: usize = 256> {
    pub book: Book<N>,
    pub buf: [u8; CAP],
    pub head: usize,
    pub tail: usize,
}

impl<const N: usize, const CAP: usize> Library<N, CAP> {
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    #[inline(always)]
    pub const fn is_full(&self) -> bool {
        (self.tail + 1) % CAP == self.head
    }

    #[inline(always)]
    pub const fn push(&mut self, id: u8) -> bool {
        if self.is_full() {
            return false;
        }
        self.buf[self.tail] = id;
        self.tail = (self.tail + 1) % CAP;
        true
    }

    #[inline(always)]
    pub const fn pop<T>(&mut self) -> Option<&'static T> {
        if self.is_empty() {
            return None;
        }
        let id = self.buf[self.head];
        self.head = (self.head + 1) % CAP;
        Some(self.book.get::<T>(id))
    }

    #[inline(always)]
    pub unsafe fn flush_sys(&mut self, fd: i32) -> isize {
        if self.is_empty() {
            return 0;
        }

        let (ptr, count) = if self.tail > self.head {
            (self.buf.as_ptr().add(self.head), self.tail - self.head)
        } else {
            (self.buf.as_ptr().add(self.head), CAP - self.head)
        };

        let written = unsafe { sys_write(fd, ptr, count) };
        if written > 0 {
            self.head = (self.head + written as usize) % CAP;
        }
        written
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
