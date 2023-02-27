use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::database;
use tauri::{self, State};

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
    pub item_name: Option<String>,

    pub pref_id: u32,
    pub pref_short_name: String,
    pub pref_long_name: String,
    pub pref_furi_kana: String,
}
impl DbResultWatershed {
    pub fn new(
        id: u32,
        name: &str,
        item_id: u32,
        item_key: &str,
        item_data: &str,
        item_name: Option<&str>,
        pref_id: u32,
        pref_short_name: &str,
        pref_long_name: &str,
        pref_furi_kana: &str,
    ) -> Self {
        DbResultWatershed {
            id,
            name: name.to_string(),
            item_id,
            item_key: item_key.to_string(),
            item_data: item_data.to_string(),
            item_name: item_name.map(str::to_string),
            pref_id,
            pref_short_name: pref_short_name.to_string(),
            pref_long_name: pref_long_name.to_string(),
            pref_furi_kana: pref_furi_kana.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TheParent {
    pub id: u32,
    pub name: String,
    pub children: Vec<TheChild>,
    pub prefectures: Vec<ThePref>,
}
impl TheParent {
    pub fn new(id: u32, name: &String) -> Self {
        TheParent {
            id,
            name: name.clone(),
            children: vec![],
            prefectures: vec![],
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct TheChild {
    pub id: u32,
    pub watershed_id: u32,
    pub key: String,
    pub data: String,
    pub name: Option<String>,
}
impl TheChild {
    pub fn new(
        id: u32,
        watershed_id: u32,
        key: &String,
        data: &String,
        name: &Option<String>,
    ) -> Self {
        TheChild {
            id,
            watershed_id,
            key: key.clone(),
            data: data.clone(),
            name: name.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ThePref {
    pub id: u32,
    pub short_name: String,
    pub long_name: String,
    pub furi_kana: String,
}
impl ThePref {
    pub fn new(id: u32, short_name: &String, long_name: &String, furi_kana: &String) -> Self {
        ThePref {
            id,
            short_name: short_name.clone(),
            long_name: long_name.clone(),
            furi_kana: furi_kana.clone(),
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
        let child = TheChild::new(v.item_id, v.id, &v.item_key, &v.item_data, &v.item_name);
        parent.children.push(child);
        let pref = ThePref::new(
            v.pref_id,
            &v.pref_short_name,
            &v.pref_long_name,
            &v.pref_furi_kana,
        );
        parent.prefectures.push(pref);
        acc
    });

    Ok(WatershedResponse {
        watersheds: folded
            .into_iter()
            .map(|(_, mut v)| {
                let mut columns = BTreeMap::new();
                for child in v.children {
                    if child.id == 0 {
                        continue;
                    }
                    columns.insert(child.id, child);
                }
                let children: Vec<TheChild> = columns.into_iter().map(|(_, v)| v).collect();
                v.children = children;

                let mut columns = BTreeMap::new();
                for pref in v.prefectures {
                    if pref.id == 0 {
                        continue;
                    }
                    columns.insert(pref.id, pref);
                }
                let prefs: Vec<ThePref> = columns.into_iter().map(|(_, v)| v).collect();
                v.prefectures = prefs;
                v
            })
            .collect(),
    })
}
