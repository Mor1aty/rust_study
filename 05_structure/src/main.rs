struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        username: String::from("moriaty"),
        email: String::from("971896352@qq.com"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User {
        username: String::from("moriaty"),
        email: String::from("971896352@qq.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
