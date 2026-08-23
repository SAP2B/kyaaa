// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

use kyaaa::book;

#[derive(Debug, PartialEq)]
struct User {
    name: &'static str,
    password: &'static str,
}

#[derive(Debug, PartialEq)]
struct Admin {
    name: &'static str,
    password: &'static str,
    dev: bool,
}

fn main() {
    let hlist = book!(
        bob => User { name: "Bob", password: "12345" },
        mut sap => Admin { name: "SAP", password: "UwU", dev: true },
        admin => Admin { name: "Admin", password: "OwO", dev: true },
        mut alice => User { name: "Alice", password: "67890" }
    );

    unsafe {
        hlist.alice().password = "New_Alice";
        hlist.sap().password = "New_Sap";
        hlist.sap().dev = false;
        black_box(hlist.alice());
        black_box(hlist.sap());
    }
}

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}
