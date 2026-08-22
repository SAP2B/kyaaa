use kyaaa::kyaaa;

#[test]
fn test_kyaaa_lookup() {
    #[derive(PartialEq, Debug)]
    struct User {
        name: &'static str,
        password: &'static str,
    }

    let obj = kyaaa!(
        bob => User { name: "Bob", password: "12345" }
    );

    let user_by_method = obj.bob();
    let user_by_index: &User = obj.get_by_u8(0);

    assert_eq!(user_by_method.name, "Bob");
    assert_eq!(user_by_method.password, "12345");
    assert_eq!(user_by_method, user_by_index);
}
