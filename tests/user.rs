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

#[test]
fn test_hlist_book() {
    let hlist = book!(
        bob => User { name: "Bob", password: "12345" },
        mut sap => Admin { name: "SAP", password: "UwU", dev: true },
        admin => Admin { name: "Admin", password: "OwO", dev: true },
        mut alice => User { name: "Alice", password: "67890" }
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
    let bob_retrieved: &User = hlist.get(0);
    assert_eq!(bob_retrieved.name, "Bob");
    assert_eq!(bob_retrieved.password, "12345");

    let sap_retrieved: &Admin = hlist.get(1);
    assert_eq!(sap_retrieved.name, "SAP");
    assert_eq!(sap_retrieved.password, "New_Sap_Password");
    assert!(!sap_retrieved.dev);

    let admin_retrieved: &Admin = hlist.get(2);
    assert_eq!(admin_retrieved.name, "Admin");
    assert!(admin_retrieved.dev);

    let alice_retrieved: &User = hlist.get(3);
    assert_eq!(alice_retrieved.name, "Alice");
    assert_eq!(alice_retrieved.password, "New_Alice_Password");
}
