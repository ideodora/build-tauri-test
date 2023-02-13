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
pub struct CurveId {
    pub pid: String,
    pub pref_code: u32,
}

#[tauri::command]
pub(crate) async fn curves(
    app_handle: tauri::AppHandle,
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    curve_id: CurveId,
) -> Result<CurvesResponse, String> {
    println!("curves");

    println!("{:#?}", curve_id);

    let results = database::curves(&*sqlite_pool, &curve_id).await.unwrap();

    Ok(CurvesResponse { curves: results })
}
