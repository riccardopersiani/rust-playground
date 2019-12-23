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
    println!("user1: {}, {}, {}, {}", user1.username, user1.email, user1.active, user1.sign_in_count);
    let user2 = User {
        email: String::from("example@mail.com"),
        username: String::from("usernametest"),
        ..user1
    };
    println!("user2: {}, {}, {}, {}", user2.username, user2.email, user2.active, user2.sign_in_count);
}
