struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };
    println!("Username: {}", user1.username);
}
