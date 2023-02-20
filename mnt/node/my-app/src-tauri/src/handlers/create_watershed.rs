use std::{error::Error, io::Write};

use serde::{Deserialize, Serialize};
use tauri::{self, AppHandle, Manager, State};

use crate::database;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBody {
    pub name: String,
    pub data: Vec<(String, String)>,
}

#[tauri::command]
pub(crate) async fn create_watershed(
    app_handle: tauri::AppHandle,
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    body: CreateBody,
) -> Result<String, String> {
    println!("name: {}", body.name);

    for (segment_id, content) in &body.data {
        println!("segment_id: {}", segment_id);
        println!("content {}:", content);
    }

    let _result = database::create_watershed(&*sqlite_pool, &body)
        .await
        .unwrap();

    Ok("ok".to_string())
}
