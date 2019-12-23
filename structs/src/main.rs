struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(String::from("riccardopersiani"),String::from("test@mail.com"));
    println!("User: {}, {}, {}, {}", user1.username, user1.email, user1.active, user1.sign_in_count);
}
