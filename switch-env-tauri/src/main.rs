#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

use commands::change_enow_env;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![change_enow_env])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
