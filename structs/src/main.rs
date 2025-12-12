fn main() {
    struct Boook {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user = User {
        active: true,
        username: "Sameer".to_string(),
        email: "sameer@example.com".to_string(),
        sign_in_count: 1,
    };

    user.email = "newemail@example.com".to_string();
    println!("Username: {}, Email: {}", user.username, user.email);

    // Return Struct from a function
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // Create Instance from other instances
    let user2 = User {
        email: String::from("newemail@example.com"),
        ..user
    };

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    // Unit-Like Structs
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
