use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

use crate::{database, emit_import_prgress};
use tauri::{self, AppHandle, Manager, State};

use super::{CurvesResponse, LatLng};

#[derive(Debug, Serialize, Deserialize)]
pub struct CurveKey {
    pub name: String,
    pub river_code: String,
}

#[tauri::command]
pub(crate) async fn get_curves(
    app_handle: tauri::AppHandle,
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    curve_key: CurveKey,
) -> Result<CurvesResponse, String> {
    println!("get_curves");

    println!("{:#?}", curve_key);

    let results = database::get_curves(&*sqlite_pool, &curve_key)
        .await
        .unwrap();

    Ok(CurvesResponse { curves: results })
}
