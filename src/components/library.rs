//PDX short identifier: AGPL-3.0
//Copyright (C) 2026 SAP2B

use super::book::*;
use crate::bitif;
use core::arch::asm;

#[inline(always)]
pub unsafe fn sys_write(fd: i32, buf: *const u8, count: usize) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") 1isize,
            in("rdi") fd as isize,
            in("rsi") buf,
            in("rdx") count,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack)
        );
    }
    ret
}

pub struct Library<const N: usize, const CAP: usize = 256> {
    pub book: Book<N>,
    pub buf: [u8; CAP],
    pub head: usize,
    pub tail: usize,
}

impl<const N: usize, const CAP: usize> Library<N, CAP> {
    const MASK: usize = {
        assert!(CAP.is_power_of_two(), "CAP must be a power of two");
        CAP - 1
    };

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    #[inline(always)]
    pub const fn is_full(&self) -> bool {
        ((self.tail + 1) & Self::MASK) == self.head
    }

    #[inline(always)]
    pub const fn push(&mut self, id: u8) -> bool {
        let full = self.is_full();
        self.buf[self.tail] = id;
        self.tail = (self.tail + (!full as usize)) & Self::MASK;
        !full
    }

    #[inline(always)]
    pub const fn pop<T>(&mut self) -> Option<&'static T> {
        let empty = self.is_empty();
        let id = self.buf[self.head];
        self.head = (self.head + (!empty as usize)) & Self::MASK;
        bitif!(empty => None, Some(self.book.get::<T>(id)))
    }

    #[inline(always)]
    pub unsafe fn flush_sys(&mut self, fd: i32) -> isize {
        bitif!(self.is_empty() => 0, {
            let is_wrapped = (self.tail <= self.head) as usize;
            let count = is_wrapped * (CAP - self.head) + (1 - is_wrapped) * (self.tail - self.head);
            let ptr = unsafe { self.buf.as_ptr().add(self.head) };
            let written = unsafe { sys_write(fd, ptr, count) };
            let actual_bytes = written.max(0) as usize;
            self.head = (self.head + actual_bytes) & Self::MASK;
            written
        })
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
    #[derive(Copy, Clone)]
    struct User {
        name: &'static str,
        password: &'static str,
    }

    let mut lib = library!(
        0 => "",
        1 => User { name: "admin", password: "123" },
        2 => None::<usize>,
        3 => 2_000_000_000_i32,
    );

    lib.push(0);
    lib.push(1);
    lib.push(2);
    lib.push(3);

    assert!(lib.pop::<&str>().unwrap().is_empty());

    let user = lib.pop::<User>().unwrap();
    assert!(matches!(user.name.as_bytes(), b"admin"));
    assert!(matches!(user.password.as_bytes(), b"123"));

    assert!(lib.pop::<Option<usize>>().unwrap().is_none());
    assert!(*lib.pop::<i32>().unwrap() == 2_000_000_000);
    assert!(lib.pop::<i32>().is_none());
    assert!(lib.is_empty());
};
