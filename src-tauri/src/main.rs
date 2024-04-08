// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
use auth::User;
use std::io;
//use serde::Serialize;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn user_input() {
  println!("I was invoked from JS!");
}



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
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![user_input])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


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