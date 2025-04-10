// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Local, Utc};
use regex::Regex;
use reqwest::header::USER_AGENT;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

fn main() {
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_rssfeeds,
            create_database,
            get_rss_feed_channel_from_database,
            delete_rss_feed_channel_from_database,
            insert_rssfeed_into_databese,
            set_rssfeed_activity
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct RssFeed {
    pub id: String,
    pub feed_name: String,
    pub header: String,
    pub description: String,
    pub url: String,
    pub image: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RssFeedChannel {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub active: bool,
}

#[tauri::command]
async fn load_rssfeeds() -> Vec<RssFeed> {
    let mut temp = get_all_rss_items().await;

    sort_rssfeed_vec(&mut temp);

    temp.truncate(100);

    temp.iter_mut().for_each(|rssfeed| {
        rssfeed.description = add_a_tag_blank_in_discription(&rssfeed.description);
        rssfeed.description = replace_img_tag_in_discription(&rssfeed.description);
    });

    temp
}

fn replace_img_tag_in_discription(discription: &str) -> String {
    let re = Regex::new(r#"<img\s.*?>"#).unwrap();
    let result = re.replace_all(&discription, "<br /> *PICTURE* <br />");
    result.to_string()
}

fn add_a_tag_blank_in_discription(discription: &str) -> String {
    let re = Regex::new(r#"<a"#).unwrap();
    let result = re.replace_all(&discription, "<a target=\"_blank\"");
    result.to_string()
}

fn sort_rssfeed_vec(rssfeed_vec: &mut [RssFeed]) {
    rssfeed_vec.sort_by(|a, b| b.date.cmp(&a.date));
}

async fn get_all_rss_items() -> Vec<RssFeed> {
    let rss_feed_urls = get_active_rssfeed_url_from_database();
    let mut rss_feed_items = Vec::new();

    let client = reqwest::Client::new();

    for rss_feed_url in rss_feed_urls {
        if let Ok(response) = client
            .get(&rss_feed_url)
            .header(USER_AGENT, "Rssviewer/0.0.0")
            .send()
            .await
        {
            if let Ok(body) = response.text().await {
                rss_feed_items.append(&mut get_items_form_feed(&body));
            } else {
                eprintln!(
                    "{} Error while fetching rss feed from: {}",
                    Local::now().format("%Y-%m-%d-%H-%M-%S"),
                    rss_feed_url
                );
            }
        } else {
            eprintln!(
                "{} Error while fetching rss feed from: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                rss_feed_url
            );
        }
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
            let mut rss_feed: RssFeed = Default::default();

            for child in node.children() {
                match child.tag_name().name() {
                    "guid" => rss_feed.id = child.text().unwrap().to_string(),
                    "title" => rss_feed.header = child.text().unwrap().to_string(),
                    "description" => rss_feed.description = child.text().unwrap().to_string(),
                    "link" => rss_feed.url = child.text().unwrap().to_string(),
                    "pubDate" => {
                        let utc_dt: DateTime<Utc> =
                            DateTime::parse_from_rfc2822(child.text().unwrap())
                                .unwrap()
                                .with_timezone(&Utc);
                        let date = utc_dt.with_timezone(&Local);
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

fn get_active_rssfeed_url_from_database() -> Vec<String> {
    let mut file_path = tauri::api::path::data_dir().expect("Can't get data dir");
    file_path.push("me.pagnany.de");
    file_path.push("rssdb.sqlite");

    let conn = match Connection::open(file_path) {
        Ok(conn) => conn,
        Err(e) => panic!("Error opening database: {:?}", e),
    };

    let sql_select = String::from("SELECT * FROM rssfeed where active = 'true'");

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

#[tauri::command]
fn get_rss_feed_channel_from_database() -> Vec<RssFeedChannel> {
    let mut file_path = tauri::api::path::data_dir().expect("Can't get data dir");
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

    let rssfeed_channel_iter = match stmt.query_map([], |row| {
        let active_str: String = row.get(3).unwrap_or(String::from("false"));
        let active_bool = matches!(active_str.to_lowercase().as_str(), "true" | "1");

        Ok(RssFeedChannel {
            id: row.get(0).unwrap_or(0),
            name: row.get(1).unwrap_or(String::from("Empty")),
            url: row.get(2).unwrap_or(String::from("Empty")),
            active: active_bool,
        })
    }) {
        Ok(rssfeed_channel_iter) => rssfeed_channel_iter,
        Err(e) => panic!("Error querying database: {:?}", e),
    };

    rssfeed_channel_iter
        .map(|rssfeed_channel| rssfeed_channel.unwrap())
        .collect()
}

#[tauri::command]
async fn insert_rssfeed_into_databese(name: String, url: String, active: bool) {
    let mut file_path = tauri::api::path::data_dir().expect("Can't get data dir");
    file_path.push("me.pagnany.de");
    file_path.push("rssdb.sqlite");

    let conn = match Connection::open(file_path) {
        Ok(conn) => conn,
        Err(e) => panic!("Error opening database: {:?}", e),
    };

    let sql_insert = String::from("INSERT INTO rssfeed (name, url, active) VALUES (?, ?, ?)");

    let str_active = if active {
        String::from("true")
    } else {
        String::from("false")
    };

    match conn.execute(&sql_insert, [&name, &url, &str_active]) {
        Ok(_) => (),
        Err(e) => panic!("Error inserting into table: {:?}", e),
    }
}

#[tauri::command]
async fn delete_rss_feed_channel_from_database(id: i32) {
    let mut file_path = tauri::api::path::data_dir().expect("Can't get data dir");
    file_path.push("me.pagnany.de");
    file_path.push("rssdb.sqlite");

    let conn = match Connection::open(file_path) {
        Ok(conn) => conn,
        Err(e) => panic!("Error opening database: {:?}", e),
    };

    let sql_delete = String::from("DELETE FROM rssfeed WHERE id = ?");

    match conn.execute(&sql_delete, [&id]) {
        Ok(_) => (),
        Err(e) => panic!("Error deleting from table: {:?}", e),
    }
}

#[tauri::command]
fn create_database() {
    let mut file_path = tauri::api::path::data_dir().expect("Can't get data dir");
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
        "CREATE TABLE IF NOT EXISTS rssfeed (id INTEGER PRIMARY KEY, name TEXT, url TEXT, active BOOLEAN)",
    );

    match conn.execute(&sql_table_create, ()) {
        Ok(_) => (),
        Err(e) => panic!("Error creating table: {:?}", e),
    }
}

#[tauri::command]
async fn set_rssfeed_activity(id: i32, active: bool) {
    let active_string = if active { "true" } else { "false" };

    let mut file_path = tauri::api::path::data_dir().expect("Can't get data dir");
    file_path.push("me.pagnany.de");
    file_path.push("rssdb.sqlite");

    let conn = match Connection::open(file_path) {
        Ok(conn) => conn,
        Err(e) => panic!("Error opening database: {:?}", e),
    };

    let sql_delete = String::from("UPDATE rssfeed SET active = ?1 WHERE id = ?2");

    match conn.execute(&sql_delete, params![active_string, id]) {
        Ok(_) => (),
        Err(e) => panic!("Error deleting from table: {:?}", e),
    }
}
