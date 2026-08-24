// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

use kyaaa::{SeqLock, book};

#[derive(Debug, PartialEq, Copy, Clone)]
struct User {
    name: &'static str,
    password: &'static str,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Admin {
    name: &'static str,
    password: &'static str,
    dev: bool,
}

#[test]
fn test_hlist_book() {
    let hlist = book!(
        bob => User { name: "Bob", password: "12345" },
        mut sap => Admin { name: "SAP", password: "UwU", dev: true },
        readfull stats => Admin { name: "Stats", password: "LockFreePass", dev: false },
        admin => Admin { name: "Admin", password: "OwO", dev: true },
        mut alice => User { name: "Alice", password: "67890" },
        readfull lock_user => User { name: "LockBob", password: "abc" },
    );

    assert_eq!(hlist.bob().name, "Bob");
    assert_eq!(hlist.bob().password, "12345");
    assert_eq!(hlist.admin().name, "Admin");
    assert!(hlist.admin().dev);

    unsafe {
        hlist.alice().password = "New_Alice_Password";
        hlist.sap().password = "New_Sap_Password";
        hlist.sap().dev = false;

        assert_eq!(hlist.alice().password, "New_Alice_Password");
        assert_eq!(hlist.sap().password, "New_Sap_Password");
        assert!(!hlist.sap().dev);
    }

    assert_eq!(
        hlist.stats().read(),
        Admin {
            name: "Stats",
            password: "LockFreePass",
            dev: false
        }
    );

    hlist.stats().write(Admin {
        name: "Stats_Updated",
        password: "SecurePass123",
        dev: true,
    });

    assert_eq!(
        hlist.stats().read(),
        Admin {
            name: "Stats_Updated",
            password: "SecurePass123",
            dev: true
        }
    );

    hlist.lock_user().write(User {
        name: "LockBob_Updated",
        password: "xyz",
    });

    assert_eq!(hlist.lock_user().read().password, "xyz");

    let bob_retrieved: &User = hlist.get(0).unwrap();
    assert_eq!(bob_retrieved.name, "Bob");
    assert_eq!(bob_retrieved.password, "12345");

    let sap_retrieved: &Admin = hlist.get(1).unwrap();
    assert_eq!(sap_retrieved.name, "SAP");
    assert_eq!(sap_retrieved.password, "New_Sap_Password");
    assert!(!sap_retrieved.dev);

    let stats_retrieved: &SeqLock<Admin> = hlist.get(2).unwrap();
    assert_eq!(stats_retrieved.read().name, "Stats_Updated");

    let admin_retrieved: &Admin = hlist.get(3).unwrap();
    assert_eq!(admin_retrieved.name, "Admin");
    assert!(admin_retrieved.dev);

    let alice_retrieved: &User = hlist.get(4).unwrap();
    assert_eq!(alice_retrieved.name, "Alice");
    assert_eq!(alice_retrieved.password, "New_Alice_Password");

    let lock_user_retrieved: &SeqLock<User> = hlist.get(5).unwrap();
    assert_eq!(lock_user_retrieved.read().password, "xyz");

    let out_of_bounds: Option<&User> = hlist.get(200);
    assert!(out_of_bounds.is_none());
}
