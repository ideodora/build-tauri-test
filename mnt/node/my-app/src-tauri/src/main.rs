// Option and Result
// ? returns
// Box<dyn Error>
// dont migrate just exe sql

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod bootstrap;
mod database;
mod handlers;

use bootstrap::DatabaseConfig;
use serde::Serialize;
use std::error::Error;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Clone)]
pub struct ImportProgressPayload {
    current: u8,
    total: u8,
    message: String,
}

pub fn emit_import_prgress(app: &AppHandle, current: &u8, total: u8, message: &str) {
    let _emittion = app
        .emit_all(
            "import-progress",
            ImportProgressPayload {
                current: *&current.clone(),
                total,
                message: message.to_string(),
            },
        )
        .unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    // データベースのファイルパス等を設定する
    const DATABASE_DIR: &str = "info-ideodora";
    const DATABASE_FILE: &str = "db.sqlite";

    let config = DatabaseConfig::new(DATABASE_DIR, DATABASE_FILE);
    let sqlite_pool = bootstrap::init(&config)?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handlers::greet,
            handlers::candidates,
            handlers::check_initiation,
            handlers::search_points,
            handlers::curves::curves,
            handlers::get_curves::get_curves,
            handlers::save_export::save_export,
            handlers::create_watershed::create_watershed,
            handlers::watersheds::watersheds
        ])
        // ハンドラからコネクションプールにアクセスできるよう、登録する
        .setup(|app| {
            app.manage(sqlite_pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
