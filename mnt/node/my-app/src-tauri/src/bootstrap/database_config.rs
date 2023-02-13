use std::fs;
use std::io;
use std::path::{self, Path};

use crate::bootstrap::dirs;

pub struct DatabaseConfig {
    dir: String,
    file: String,
}

impl DatabaseConfig {
    pub fn new(dir: &str, file: &str) -> DatabaseConfig {
        DatabaseConfig {
            dir: dir.to_string(),
            file: file.to_string(),
        }
    }

    pub fn is_db_file_exists(&self) -> Result<bool, io::Error> {
        let database_dir = self.database_dir_path()?;
        let database_file = database_dir.join(&self.file);

        // データベースファイルが存在するかチェックする
        let db_exists = Path::new(&database_file).try_exists()?;
        Ok(db_exists)
    }

    pub fn database_url(&self) -> Result<String, io::Error> {
        let database_dir = self.database_dir_path()?;

        let database_dir_str = dunce::canonicalize(&database_dir)?;
        let s = database_dir_str.to_string_lossy().replace('\\', "/");

        let database_url = format!("sqlite://{}/{}", s, self.file);

        Ok(database_url)
    }

    fn database_dir_path(&self) -> Result<path::PathBuf, io::Error> {
        let home_dir = dirs::make_home_dir()?;
        let database_dir = home_dir.join(&self.dir);

        // データベース用フォルダが存在するかチェックする
        let dir_exists = Path::new(&database_dir).try_exists()?;

        // 存在しないなら、ファイルを格納するためのディレクトリを作成する
        if !dir_exists {
            fs::create_dir(&database_dir)?;
        }

        Ok(database_dir)
    }
}
