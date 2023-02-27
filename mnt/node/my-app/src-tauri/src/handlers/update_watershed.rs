use serde::{Deserialize, Serialize};

use crate::database;
use tauri::{self, State};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWatershedRequest {
    pub id: u32,
    pub name: String,
    pub pref_ids: Vec<u32>,
}

#[tauri::command]
pub(crate) async fn update_watershed(
    _app_handle: tauri::AppHandle,
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    payload: UpdateWatershedRequest,
) -> Result<(), String> {
    println!("update_watershed, {:#?}", payload);

    database::update_watershed(&*sqlite_pool, &payload).await;

    // let folded = _results.iter().fold(BTreeMap::new(), |mut acc, v| {
    //     let parent = acc.entry(v.id).or_insert(TheParent::new(v.id, &v.name));
    //     let child = TheChild::new(v.item_id, &v.item_key, &v.item_data);
    //     parent.children.push(child);
    //     acc
    // });

    // Ok(WatershedResponse {
    //     watersheds: folded.into_iter().map(|(_, v)| v).collect(),
    // })
    Ok(())
}
