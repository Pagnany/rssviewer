// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

fn main() {
    tauri::Builder::default()
        //.setup(_setup_handler)
        .invoke_handler(tauri::generate_handler![example_feed, test1])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn example_feed() -> String {
    let rss_url = "https://www.heise.de/rss/heise.rdf";
    //let rss_url = "https://www.tagesschau.de/inland/index~rss2.xml";
    //let rss_url = "https://www.spiegel.de/schlagzeilen/index.rss";
    let mut my_string = String::new();
    if let Ok(content) = reqwest::get(rss_url).await {
        if let Ok(text) = content.text().await {
            my_string = text;
        }
    }
    my_string
}

#[tauri::command]
fn test1() -> Vec<RssFeed> {
    let mut test_vec = Vec::new();
    test_vec.push(RssFeed {
        id: String::from("1"),
        header: String::from("Test"),
        description: String::from("Test"),
        url: String::from("Test"),
        image: String::from("Test"),
        date: String::from("Test"),
    });
    test_vec.push(RssFeed {
        id: String::from("2"),
        header: String::from("Test"),
        description: String::from("Test"),
        url: String::from("Test"),
        image: String::from("Test"),
        date: String::from("Test"),
    });
    test_vec
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RssFeed {
    pub id: String,
    pub header: String,
    pub description: String,
    pub url: String,
    pub image: String,
    pub date: String,
}

fn _insert_rssfeed_into_databese(name: String, url: String) {
    let mut file_path = tauri::api::path::data_dir().unwrap_or(std::path::PathBuf::new());
    file_path.push("me.pagnany.de");
    file_path.push("rssdb.sqlite");

    let conn = match Connection::open(file_path) {
        Ok(conn) => conn,
        Err(e) => panic!("Error opening database: {:?}", e),
    };

    let sql_insert = String::from("INSERT INTO rssfeed (name, url) VALUES (?, ?)");

    match conn.execute(&sql_insert, &[&name, &url]) {
        Ok(_) => (),
        Err(e) => panic!("Error inserting into table: {:?}", e),
    }
}

fn _create_database() {
    let mut file_path = tauri::api::path::data_dir().unwrap_or(std::path::PathBuf::new());
    file_path.push("me.pagnany.de");
    file_path.push("rssdb.sqlite");

    // create path if not exists
    if !file_path.parent().unwrap().exists() {
        std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
    }

    let conn = match Connection::open(file_path) {
        Ok(conn) => conn,
        Err(e) => panic!("Error opening database: {:?}", e),
    };

    let sql_table_create = String::from(
        "CREATE TABLE IF NOT EXISTS rssfeed (id INTEGER PRIMARY KEY, name TEXT, url TEXT)",
    );

    match conn.execute(&sql_table_create, ()) {
        Ok(_) => (),
        Err(e) => panic!("Error creating table: {:?}", e),
    }

    /*
    let tx = conn.transaction().unwrap();
    let mut stmt = tx.prepare("INSERT INTO test (name) VALUES (?1)").unwrap();
    for geb in gebinde {
        stmt.execute([geb]).unwrap();
    }
    stmt.finalize().unwrap();
    tx.commit().unwrap();
    */
}

fn _setup_handler(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let app_handle = app.handle();

    println!(
        "{}",
        app_handle
            .path_resolver()
            .resource_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        app_handle
            .path_resolver()
            .app_config_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        app_handle
            .path_resolver()
            .app_data_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        app_handle
            .path_resolver()
            .app_local_data_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        app_handle
            .path_resolver()
            .app_cache_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        app_handle
            .path_resolver()
            .app_log_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "Data dir: {}",
        tauri::api::path::data_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::local_data_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::cache_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::config_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::executable_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::public_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::runtime_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::template_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::font_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::home_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::audio_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::desktop_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::document_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::download_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::picture_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    Ok(())
}
