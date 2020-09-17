struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // entire instance must be mutable
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Access specific field by dot syntax
    user1.email = String::from("anotheremail@example.com");

    // Struct update syntax to change some of the fields
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // tuple struct
    // Don't have names associated with their fields
    struct Color(i32, i32, i32);
    let black = Color(0,0,0);

    // How about different types? It works!
    struct TupleUser(String, String, u64, bool);
    let user3 = TupleUser(String::from("email@exmple.com"), String::from("yetanotherusername890"), 1, true);
}

fn build_user(email: String, username: String) -> User {
    User {
        //email: email,         // inconvinience
        email,                  // parameter have same name as field
        //username: username,
        username,
        active: true,
        sign_in_count: 1,
    }
}

