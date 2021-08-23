struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let user1 = build_user(String::from("cinob@foxmail.com"), String::from("cinob"));
    let user2 = User {
        username: String::from("wahaha"),
        ..user1
    };
    println!("Hello, {} {}", user1.username, user2.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
