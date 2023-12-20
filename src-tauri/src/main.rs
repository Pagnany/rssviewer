// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::prelude::*;
use regex::Regex;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

fn main() {
    tauri::Builder::default()
        //.setup(_setup_handler)
        .invoke_handler(tauri::generate_handler![example_feed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn example_feed() -> Vec<RssFeed> {
    let mut temp = get_all_rss_items().await;
    sort_rssfeed_vec(&mut temp);
    temp
}

fn sort_rssfeed_vec(rssfeed_vec: &mut Vec<RssFeed>) {
    rssfeed_vec.sort_by(|a, b| b.date.cmp(&a.date));
}

async fn get_all_rss_items() -> Vec<RssFeed> {
    let rss_feed_urls = get_rssfeed_url_from_database();
    let mut rss_feed_items = Vec::new();

    for rss_feed_url in rss_feed_urls {
        let response = reqwest::get(&rss_feed_url).await.unwrap();
        let body = response.text().await.unwrap();
        rss_feed_items.append(&mut get_items_form_feed(&body));
    }
    rss_feed_items
}

fn get_items_form_feed(feed: &str) -> Vec<RssFeed> {
    let mut rss_feed_vec = Vec::new();

    let doc = roxmltree::Document::parse(feed).unwrap();
    let mut rss_feed_name = String::from("");
    let mut rss_feed_name_set = false;
    for node in doc.descendants() {
        if node.tag_name().name() == "item" {
            let mut rss_feed = RssFeed {
                id: String::from(""),
                feed_name: String::from(""),
                header: String::from(""),
                description: String::from(""),
                url: String::from(""),
                image: String::from(""),
                date: String::from(""),
            };

            for child in node.children() {
                //println!("{}", child.tag_name().name());
                match child.tag_name().name() {
                    "guid" => rss_feed.id = child.text().unwrap().to_string(),
                    "title" => rss_feed.header = child.text().unwrap().to_string(),
                    "description" => rss_feed.description = child.text().unwrap().to_string(),
                    "link" => rss_feed.url = child.text().unwrap().to_string(),
                    "pubDate" => {
                        let date = DateTime::parse_from_rfc2822(child.text().unwrap())
                            .unwrap()
                            .with_timezone(&FixedOffset::east_opt(3600).unwrap());
                        rss_feed.date = date.format("%Y-%m-%d %H:%M").to_string();
                    }
                    "enclosure" => {
                        if child.attribute("type").unwrap() == "image/jpeg" {
                            rss_feed.image = child.attribute("url").unwrap().to_string();
                        }
                    }
                    "encoded" => {
                        // find <img> tag in a non xml string and get the link in src=""
                        let content_encoded = child.text().unwrap().to_string();
                        let re = Regex::new(r#"<img src="([^"]*)""#).unwrap();
                        let caps = re.captures(&content_encoded);

                        if let Some(caps) = caps {
                            let img_src = &caps[1];
                            rss_feed.image = img_src.to_string();
                        }
                    }
                    _ => (),
                }
            }
            rss_feed.feed_name = rss_feed_name.clone();
            rss_feed_vec.push(rss_feed);
        } else if node.tag_name().name() == "title" && !rss_feed_name_set {
            rss_feed_name = node.text().unwrap().to_string();
            rss_feed_name_set = true;
        }
    }

    rss_feed_vec
}

fn get_rssfeed_url_from_database() -> Vec<String> {
    let mut file_path = tauri::api::path::data_dir().unwrap_or(std::path::PathBuf::new());
    file_path.push("me.pagnany.de");
    file_path.push("rssdb.sqlite");

    let conn = match Connection::open(file_path) {
        Ok(conn) => conn,
        Err(e) => panic!("Error opening database: {:?}", e),
    };

    let sql_select = String::from("SELECT * FROM rssfeed");

    let mut stmt = match conn.prepare(&sql_select) {
        Ok(stmt) => stmt,
        Err(e) => panic!("Error preparing statement: {:?}", e),
    };

    let mut rssfeed_vec = Vec::new();

    let rssfeed_iter =
        match stmt.query_map([], |row| Ok(row.get(2).unwrap_or(String::from("Empty")))) {
            Ok(rssfeed_iter) => rssfeed_iter,
            Err(e) => panic!("Error querying database: {:?}", e),
        };

    for rssfeed in rssfeed_iter {
        rssfeed_vec.push(rssfeed.unwrap());
    }

    rssfeed_vec
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RssFeed {
    pub id: String,
    pub feed_name: String,
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
