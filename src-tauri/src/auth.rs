// auth.rs

use bcrypt::{hash, verify};

// Define a struct to represent user credentials
pub struct User {
    username: String,
    password_hash: String,
}

impl User {
    // Function to create a new user
    pub fn new(username: &str, password: &str) -> Self {
        // Hash the password before storing it
        let password_hash = hash(password, 10).expect("Failed to hash password");
        User {
            username: username.to_string(),
            password_hash,
        }
    }

    // Function to authenticate a user
    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        // Check if provided credentials match
        self.username == username && verify(password, &self.password_hash).unwrap_or(false)
    }
}
