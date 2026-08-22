use kyaaa::kyaaa;

fn main() {
    #[derive(PartialEq, Debug)]
    struct User {
        name: &'static str,
        password: &'static str,
    }

    let obj = kyaaa!(
        bob => User { name: "Bob", password: "12345" }
    );

    let user = obj.bob();

    println!("User: {}", user.name);
}
