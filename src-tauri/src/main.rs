// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, example_feed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn example_feed() -> String {
    let mut my_string = String::new();
    if let Ok(content) = reqwest::get("https://www.heise.de/rss/heise.rdf").await {
        if let Ok(text) = content.bytes().await {
            // bytes to string
            my_string = String::from_utf8(text.to_vec()).unwrap();
        }
    }
    my_string
}
