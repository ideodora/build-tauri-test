use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

use crate::{database, emit_import_prgress};
use tauri::{self, AppHandle, Manager, State};

// use super::{CurvesResponse, LatLng};

#[derive(Debug, Serialize, Deserialize)]
pub struct WatershedResponse {
    watersheds: Vec<TheParent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbResultWatershed {
    pub id: u32,
    pub name: String,
    // let bounds: &str = row.try_get("bounds").unwrap();
    pub item_id: u32,
    pub item_key: String,
    pub item_data: String,
}
impl DbResultWatershed {
    pub fn new(id: u32, name: &str, item_id: u32, item_key: &str, item_data: &str) -> Self {
        DbResultWatershed {
            id,
            name: name.to_string(),
            item_id,
            item_key: item_key.to_string(),
            item_data: item_data.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TheParent {
    pub id: u32,
    pub name: String,
    pub children: Vec<TheChild>,
}
impl TheParent {
    pub fn new(id: u32, name: &String) -> Self {
        TheParent {
            id,
            name: name.clone(),
            children: vec![],
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct TheChild {
    pub id: u32,
    pub key: String,
    pub data: String,
}
impl TheChild {
    pub fn new(id: u32, key: &String, data: &String) -> Self {
        TheChild {
            id,
            key: key.clone(),
            data: data.clone(),
        }
    }
}

#[tauri::command]
pub(crate) async fn watersheds(
    _app_handle: tauri::AppHandle,
    sqlite_pool: State<'_, sqlx::SqlitePool>,
) -> Result<WatershedResponse, String> {
    println!("watersheds");

    let _results = database::watersheds(&*sqlite_pool).await.unwrap();

    let folded = _results.iter().fold(BTreeMap::new(), |mut acc, v| {
        let parent = acc.entry(v.id).or_insert(TheParent::new(v.id, &v.name));
        let child = TheChild::new(v.item_id, &v.item_key, &v.item_data);
        parent.children.push(child);
        acc
    });

    Ok(WatershedResponse {
        watersheds: folded.into_iter().map(|(_, v)| v).collect(),
    })
}
