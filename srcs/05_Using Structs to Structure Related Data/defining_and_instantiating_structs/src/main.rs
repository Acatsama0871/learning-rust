
// define a struct
struct User {
    active: bool,
    username: String,  // we want the struct to own its data, so we don't need to use &str
    email: String,
    sign_in_account: u64,
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit struct: used to add trait but not need to store any data
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("some_email"),
        sign_in_account: 12344
    };
    user1.email = String::from("iamemail@emai.com");
    let user2 = build_user(String::from("1223"), String::from("1233"));
    let user3 = User {
        active: true,
        ..user2
    };
    
    let black = Color(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,  // field init shorthand
        email,  // field init shorthand
        sign_in_account: 12345
    }
}
