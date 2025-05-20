// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Initialize dotenv to load environment variables from .env file
    dotenv::from_filename(".env").ok();
    assistant_lib::run()
}
