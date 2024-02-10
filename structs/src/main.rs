// Structs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like structs
struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("my@email.com"), String::from("myUser"));

    // The fields that are not specified will be the same as user1
    // It's important to note that the values will move from user1
    let user3 = User {
        email: String::from("another@example.com"),
        // Using the Struct update syntax
        ..user1
    };

    // Even though they share the same field types, Color and Point are not interchangeable
    // Because the types would are not the same since they are instances of different structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // Field init shorthand
        username,
        email,
        sign_in_count: 1,
    }
}