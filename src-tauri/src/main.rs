// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
use auth::User;
//use tauri::http::Response;
//use std::io;
//use serde::Serialize;


// Get username and password from the user
#[tauri::command(rename_all = "snake_case")]
fn user_input(employeeid: String, pswd: String) -> Result<String, String> {
    let username = employeeid.clone();
    let password = pswd;

    // Create a new test user
    let user = User::new("example_user", "password123");

    // Attempt to log in against the created test user above
    if user.authenticate(&username, &password) {
        println!("Login successful!");//for testing only, comment out to disable printing to the console
        Ok(username)
    } else {
        println!("Incorrect username or password!");//for testing only, comment out to disable printing to the console
        //return an error if credentials do not match
        Err(String::from("Incorrect username or password"))
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![user_input])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}