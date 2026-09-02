// SPDX-License-Identifier: AGPL-3.0
// Copyright (C) 2026 SAP2B

use kyaaa::{Str, book, page};

// 1. Schemas defined via page!
page!(
    struct Dog {
        name: Str<16>,
        age: u8,
    }
    struct Cat {
        name: Str<16>,
        age: u8,
    }
);

fn main() {
    // 2. Heterogeneous registry with 2 Dogs and 2 Cats using struct literals
    let shelter = book!(
        dog1 => Dog {
            name: Str::from_str("Rex"),
            age: 3,
        },
        dog2 => Dog {
            name: Str::from_str("Thor"),
            age: 5,
        },
        cat1 => Cat {
            name: Str::from_str("Luna"),
            age: 2,
        },
        cat2 => Cat {
            name: Str::from_str("Mina"),
            age: 1,
        }
    );

    // 3. Direct access to all 4 elements via generated tokens
    assert_eq!(shelter.dog1().get().age, 3);
    assert_eq!(shelter.dog2().get().age, 5);
    assert_eq!(shelter.cat1().get().age, 2);
    assert_eq!(shelter.cat2().get().age, 1);

    // 4. Safe mutation on specific instances (setters continue working via KyaaaConstInto)
    shelter.cat1().set().age(3); // Luna had a birthday!
    shelter.dog2().set().age(6); // Thor had a birthday!

    assert_eq!(shelter.cat1().get().age, 3);
    assert_eq!(shelter.dog2().get().age, 6);

    // 5. Type filtering now returns exactly 2 of each
    let dog_count = shelter.list::<Dog>().count();
    let cat_count = shelter.list::<Cat>().count();

    assert_eq!(dog_count, 2);
    assert_eq!(cat_count, 2);

    println!("Shelter updated: {dog_count} dogs and {cat_count} cats registered!");
}
