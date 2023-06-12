#[derive(Debug)]
struct User {
    name: String,
    age: i64,
    active: bool,
    email: String
}

impl User {
    fn new(name: String, age: i64, active: bool, email: String) -> Self {
        Self {
            name,
            age,
            active,
            email,
        }
    }
}
fn main() {
    let User = User::new(
        String::from("Bob"),
        17,
        true,
        String::from("bob@gmail.com")
    );
    println!("{:#?}", User);
}