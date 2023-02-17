use std::{error::Error, io::Write};

use serde::{Deserialize, Serialize};
use tauri::{self, AppHandle, Manager, State};
use zip::{result::ZipError, write::FileOptions};

fn zip(_path: &String, _list: &Vec<(String, String)>) -> zip::result::ZipResult<()> {
    let path = std::path::Path::new(_path);
    let file = std::fs::File::create(&path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    for (i, content) in _list.iter() {
        let prefixed = format!("files/{}", i);
        zip.start_file(prefixed, FileOptions::default())?;
        zip.write_all(content.as_bytes())?;
    }
    zip.finish()?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveBody {
    pub path: String,
    pub data: Vec<(String, String)>,
}

#[tauri::command]
pub(crate) async fn save_export(
    app_handle: tauri::AppHandle,
    body: SaveBody,
) -> Result<String, String> {
    println!("body path: {}", body.path);

    for (filename, content) in &body.data {
        println!("filename: {}", filename);
        println!("content {}:", content);
    }

    match zip(&body.path, &body.data) {
        Ok(_) => {
            println!("zipping done");
            Ok("done".to_string())
        }
        Err(err) => {
            println!("{}", err);
            Err(format!("{}", err))
        }
    }
}
