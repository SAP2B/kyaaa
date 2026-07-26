// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#[derive(Copy, Clone)]
pub struct Book<const N: usize>(pub [(usize, &'static [u8]); N]);

impl<const N: usize> Book<N> {
    #[inline(always)]
    pub const fn new(entries: [(usize, &'static [u8]); N]) -> Self {
        assert!(N <= 256, "Book supports max 256 entries");
        Self(entries)
    }

    #[inline(always)]
    pub const fn get<T>(&self, id: u8) -> &'static T {
        let id = id as usize;
        assert!(id < N, "ID out of bounds");
        let bytes = self.0[id].1;
        assert!(
            bytes.len() == ::core::mem::size_of::<T>(),
            "Type size mismatch"
        );
        unsafe { &*(bytes.as_ptr() as *const T) }
    }

    #[inline(always)]
    pub const fn find<T>(&self, val: &T) -> Option<u8> {
        let target_bytes = unsafe {
            ::core::slice::from_raw_parts(val as *const _ as *const u8, ::core::mem::size_of::<T>())
        };

        let mut i = 0;
        while i < N {
            let stored = self.0[i].1;
            if stored.len() == target_bytes.len() {
                let mut j = 0;
                let mut matches = true;
                while j < stored.len() {
                    if stored[j] != target_bytes[j] {
                        matches = false;
                        break;
                    }
                    j += 1;
                }
                if matches {
                    return Some(self.0[i].0 as u8);
                }
            }
            i += 1;
        }
        None
    }
}

#[macro_export]
macro_rules! book {
    ($($id:expr => $val:expr),* $(,)?) => {
        Book::new([
            $((
                $id as usize,
                unsafe {
                    ::core::slice::from_raw_parts(
                        (&$val) as *const _ as *const u8,
                        ::core::mem::size_of_val(&$val)
                    )
                }
            )),*
        ])
    };
}

const _: () = {
    #[derive(Copy, Clone)]
    struct User {
        name: &'static str,
        password: &'static str,
    }

    let lib = book!(
        0 => "",
        1 => User { name: "1", password: "2" },
        2 => None::<usize>,
        3 => 2_000_000_000
    );

    assert!(lib.get::<&str>(0).is_empty());
    assert!(matches!(lib.get::<User>(1).name.as_bytes(), b"1"));
    assert!(matches!(lib.get::<User>(1).password.as_bytes(), b"2"));
    assert!(lib.get::<Option<usize>>(2).is_none());
    assert!(*lib.get::<i32>(3) == 2_000_000_000);
};
