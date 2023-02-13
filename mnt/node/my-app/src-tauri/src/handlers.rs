use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

use crate::{database, emit_import_prgress};
use tauri::{self, AppHandle, Manager, State};

pub mod curves;
pub mod get_curves;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    candidates: Vec<RiverCandidate>,
}

/// 検索候補
#[derive(Debug, Serialize, Deserialize)]
pub struct RiverCandidate {
    river_name: String,
    prefecture_name: String,
    river_code: String,
}

impl RiverCandidate {
    pub fn new(river_name: &str, river_code: String, prefecture_name: &str) -> Self {
        RiverCandidate {
            river_name: river_name.to_string(),
            prefecture_name: prefecture_name.to_string(),
            river_code,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ImportProgressPayload {
    current: u8,
    remains: u8,
    total: u8,
}

#[tauri::command]
pub(crate) async fn greet(
    app_handle: tauri::AppHandle,
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    name: &str,
) -> Result<String, String> {
    crate::emit_import_prgress(&app_handle, &1, 28, "init");

    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or("no app data dir")?;

    crate::emit_import_prgress(&app_handle, &2, 28, "check app data dir");

    extract_files(&name, &app_data_dir, &app_handle);

    println!("Done extract files. Next");
    let mut next_part = 16;
    crate::emit_import_prgress(&app_handle, &next_part, 28, "extracted files");

    let xy = &app_data_dir.join("migration");

    match &xy.to_str() {
        Some(str) => println!("migration files at: {}", &str),
        None => println!("failed path to str"),
    }

    next_part += 1;
    crate::emit_import_prgress(&app_handle, &next_part, 28, "start seeding");

    database::rutime_migrate_database(&sqlite_pool, xy.to_path_buf(), &app_handle)
        .await
        .map_err(|e| e.to_string())?;

    let last_part = 28;
    crate::emit_import_prgress(&app_handle, &last_part, 28, "Done all seed");

    println!("Hello, {}!", name);
    Ok(format!("Hello, {}!", name))
}

#[tauri::command]
pub(crate) async fn candidates(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    query: &str,
) -> Result<Board, String> {
    println!("Hello, {}!", query);
    let candidates = database::query_candidates(&*sqlite_pool, &query)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Board { candidates })
}

#[tauri::command]
pub(crate) async fn check_initiation(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
) -> Result<bool, String> {
    println!("data checking");
    let result = database::check_data(&*sqlite_pool).await;
    Ok(result)
}

fn un_zip(path: &PathBuf) -> std::io::Result<()> {
    // let file = File::open("./01_Point.sql.gz")?;
    println!("{}", path.to_str().unwrap());
    let out_path = path.to_str().unwrap().trim_end_matches(".gz");
    println!("{}", out_path);

    let file = File::open(&path)?;
    let mut decoder = GzDecoder::new(file);
    let mut sql = Vec::new();
    decoder.read_to_end(&mut sql)?;

    println!("gzip read {} DONE", out_path);

    let mut output_file = File::create(out_path)?;
    output_file.write_all(&sql)?;

    println!("write file {} DONE", out_path);
    Ok(())
}

fn extract_files(
    file_path: &str,
    app_data_dir: &PathBuf,
    app_handle: &AppHandle,
) -> std::io::Result<()> {
    let mut current: u8 = 2;

    let file = File::open(&file_path)?;
    let mut archive = ZipArchive::new(file)?;

    let outdir = app_data_dir.join("migration");

    current += 1;
    emit_import_prgress(&app_handle, &current, 28, "create temp folder");
    std::fs::create_dir_all(&outdir)?;
    current += 1;
    emit_import_prgress(&app_handle, &current, 28, "complete create temp folder");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = outdir.join(file.mangled_name());

        if outpath.to_str().unwrap().contains("__MACOSX") {
            continue;
        }

        current += 1;
        emit_import_prgress(&app_handle, &current, 28, "start copy and unzip a file");

        let mut outfile = File::create(&outpath)?;
        std::io::copy(&mut file, &mut outfile)?;

        let _ = un_zip(&outpath);

        current += 1;
        emit_import_prgress(&app_handle, &current, 28, "complete copy and unzip a file");
    }

    println!("all source file Done");
    current += 1;
    emit_import_prgress(
        &app_handle,
        &current,
        28,
        "complete extracting all source files",
    );

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatLng {
    pub lat: f32,
    pub lng: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bounds {
    pub min: LatLng,
    pub max: LatLng,
}

#[tauri::command]
pub(crate) async fn search_points(
    app_handle: tauri::AppHandle,
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    bounds: Bounds,
) -> Result<SearchPointsResponse, String> {
    println!("search_points");

    println!("{:#?}", bounds);

    let results = database::search_points(&*sqlite_pool, &bounds)
        .await
        .unwrap();

    Ok(SearchPointsResponse { points: results })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPointsResponse {
    pub points: Vec<Point>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub id: String,
    pub sid: String,
    pub nid: String,
    pub lat: f32,
    pub lng: f32,
    pub prefecture_code: u32, // i64?
    pub river_name: String,
    pub river_code: i64,
}
impl Point {
    pub fn new(
        id: &str,
        sid: &str,
        nid: &str,
        lat: &f32,
        lng: &f32,
        prefecture_code: &u32,
        river_name: &str,
        river_code: &i64,
    ) -> Point {
        Point {
            id: id.to_string(),
            sid: sid.to_string(),
            nid: nid.to_string(),
            lat: *lat,
            lng: *lng,
            prefecture_code: *prefecture_code,
            river_name: river_name.to_string(),
            river_code: *river_code,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurvesResponse {
    pub curves: Vec<Curve>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Curve {
    pub id: String,
    pub segments: String,
}
impl Curve {
    pub fn new(id: &str, segments: &str) -> Curve {
        Curve {
            id: id.to_string(),
            segments: segments.to_string(),
        }
    }
}
