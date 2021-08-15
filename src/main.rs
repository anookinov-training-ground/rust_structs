struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// struct UserStr {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

fn main() {    
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    user1 = build_user(String::from("someemail@example.com"), String::from("someusername"));

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     username: String::from("anotherusername567"),
    //     active: user1.active,
    //     sign_in_count: user1.sign_in_count,
    // };

    // specifies remaining user1 fields with .. syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("user 1 = {}, {}, {}, {}", user1.username, user1.email, user1.sign_in_count, user1.active);
    println!("user 2 = {}, {}, {}, {}", user2.username, user2.email, user2.sign_in_count, user2.active);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Color(red, green, blue) = black;
    println!("Color = {}, {}, {}", red, green, blue);

    let Point(x, y, z) = origin;
    println!("Point = {}, {}, {}", x, y, z);

    // let user3 = UserStr {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };
}

fn build_user(email: String, username: String) -> User {
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // }

    // field init shorthand
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}