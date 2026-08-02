// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

#[derive(Copy, Clone)]
pub struct Book<const N: usize>(pub [*const (); N]);

impl<const N: usize> Book<N> {
    #[inline(always)]
    pub const fn new(entries: [*const (); N]) -> Self {
        assert!(N <= 256, "Book supports max 256 entries");
        Self(entries)
    }

    #[inline(always)]
    pub const fn get<T>(&self, id: u8) -> &'static T {
        let idx = id as usize;
        assert!(idx < N, "ID out of bounds");
        unsafe { &*(self.0[idx] as *const T) }
    }
}

#[macro_export]
macro_rules! book {
    ($($id:expr => $val:expr),* $(,)?) => {
        Book::new([
            $({
                let _ = $id;
                &const { $val } as *const _ as *const ()
            }),*
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
