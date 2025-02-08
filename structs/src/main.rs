fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("User1: {}", user1.username); // use dot notation to access fields
    println!("User1: {}", user1.email);

    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    println!("__________________________");

    println!("User2: {}", user2.username); 
    println!("User2: {}", user2.email);

    user2.email = String::from("anotheremail@example.com"); // update field as user2 is mutable
    println!("User2: {}", user2.email);

    println!("__________________________");
    let user3 = build_user(String::from("anewemail.com"), String::from("anotherusername"));
    println!("User3: {}", user3.username);
    println!("User3: {}", user3.email);

    println!("__________________________");
    let user4 = User {
        email: String::from("wowanotheremail.com"),
        ..user3 // use struct update syntax to copy the rest of the fields from user3
    };
    println!("User4: {}", user4.username);
    println!("User4: {}", user4.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}