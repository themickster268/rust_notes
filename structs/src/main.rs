fn main() {
    /* let mut user1 = User {
        email: String::from("michael@example.com"),
        username: String::from("michael132"),
        active: true,
        sign_in_count: 14,
    };

    user1.email = String::from("michael@gmail.com");

    let user2 = User {
        email: String::from("michael@hotmail.com"),
        ..user1 // Same fields as user1, apart from email
    }; */

    // We can no longer use user1, as the String for username moved
    // from user1 to user2. If we had created a new separate String
    // for user2, then user1 would be valid here. Also if we had only coped the 
    // active and sign_in_count from user1 to user2, user1 would still be valid 
    // as the types of theses properties implement the Copy trait.

    
}

/* struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: false,
        username,
        email,
        sign_in_count: 1,
    }
} */
