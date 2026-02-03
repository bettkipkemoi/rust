struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

//tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//unit-like structs
struct AlwaysEqual;

fn main() {
    let mut user1 = User{
        active: true,
        username: String::from("Bett Kipkemoi"),
        email: String::from("bett@student.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("bett@example.com"),
        ..user1
    };
    user1.email = String::from("bettse@student.com");

    //tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //unit-like structs
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}