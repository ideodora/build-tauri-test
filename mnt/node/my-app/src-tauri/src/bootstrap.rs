use std::error::Error;
mod database_config;
mod dirs;

use crate::database;
use tauri::async_runtime;

use sqlx::SqlitePool;

pub use database_config::DatabaseConfig;

pub fn init(database_config: &DatabaseConfig) -> Result<SqlitePool, Box<dyn Error>> {
    // データベースファイルが存在するかチェックする
    let db_exists = database_config.is_db_file_exists()?;

    // データベース接続用URL
    let database_url = database_config.database_url()?;

    // SQLiteのコネクションプールを作成する
    let sqlite_pool = async_runtime::block_on(database::create_sqlite_pool(&database_url))?;

    // マイグレーションSQLを実行する
    async_runtime::block_on(database::migrate_database(&sqlite_pool))?;

    println!("{}, {}", database_url, db_exists);

    Ok(sqlite_pool)
}
