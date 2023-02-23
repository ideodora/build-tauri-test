use serde::{Deserialize, Serialize};

use crate::database;
use tauri::{self, State};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWatershedRequest {
    pub id: u32,
    pub name: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct WatershedResponse {
//     watersheds: Vec<TheParent>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct DbResultWatershed {
//     pub id: u32,
//     pub name: String,
//     // let bounds: &str = row.try_get("bounds").unwrap();
//     pub item_id: u32,
//     pub item_key: String,
//     pub item_data: String,
// }
// impl DbResultWatershed {
//     pub fn new(id: u32, name: &str, item_id: u32, item_key: &str, item_data: &str) -> Self {
//         DbResultWatershed {
//             id,
//             name: name.to_string(),
//             item_id,
//             item_key: item_key.to_string(),
//             item_data: item_data.to_string(),
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct TheParent {
//     pub id: u32,
//     pub name: String,
//     pub children: Vec<TheChild>,
// }
// impl TheParent {
//     pub fn new(id: u32, name: &String) -> Self {
//         TheParent {
//             id,
//             name: name.clone(),
//             children: vec![],
//         }
//     }
// }
// #[derive(Debug, Serialize, Deserialize)]
// struct TheChild {
//     pub id: u32,
//     pub key: String,
//     pub data: String,
// }
// impl TheChild {
//     pub fn new(id: u32, key: &String, data: &String) -> Self {
//         TheChild {
//             id,
//             key: key.clone(),
//             data: data.clone(),
//         }
//     }
// }

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
