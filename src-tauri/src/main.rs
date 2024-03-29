// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/*#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}*/

mod auth;

use auth::User;

use std::io;

// Function to prompt the user for input
fn prompt(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    /*tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
*/

    // Get username and password from the user
     // Get username from the user
     let username = prompt("Enter your username:");

     // Get password from the user
     let password = prompt("Enter your password:");
 
     // Create a new user
     let user = User::new("example_user", "password123");
 
     // Attempt to log in
     if user.authenticate(&username, &password) {
         println!("Login successful!");
     } else {
         println!("Incorrect username or password.");
     }
}