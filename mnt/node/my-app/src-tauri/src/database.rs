use std::{collections::BTreeMap, path::Path, path::PathBuf, result, str::FromStr};

use futures::TryStreamExt;
use sqlx::{
    sqlite::{
        SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteRow, SqliteSynchronous,
    },
    Row, SqlitePool,
};
use std::fs;
use tauri::AppHandle;

use crate::handlers::{
    curves::CurveId, get_curves::CurveKey, Bounds, Curve, Point, RiverCandidate,
};

/// このモジュール内の関数の戻り値型
type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

/// SQLiteのコネクションプールを作成して返す
pub async fn create_sqlite_pool(database_url: &str) -> DbResult<SqlitePool> {
    // コネクションの設定
    let connection_options = SqliteConnectOptions::from_str(database_url)?
        // DBが存在しないなら作成する
        .create_if_missing(true)
        // トランザクション使用時の性能向上のため、WALを使用する
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    // 上の設定を使ってコネクションプールを作成する
    let sqlite_pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;

    Ok(sqlite_pool)
}

/// マイグレーションを行う
pub async fn migrate_database(pool: &SqlitePool) -> DbResult<()> {
    sqlx::migrate!().run(pool).await?;
    Ok(())
}

// just exec query
pub async fn rutime_migrate_database(
    pool: &SqlitePool,
    migrate_path: PathBuf,
    app: &AppHandle,
) -> DbResult<()> {
    let mut progress: u8 = 17;

    match fs::read_dir(&migrate_path) {
        Ok(_) => println!("read dir success"),
        Err(error) => println!("read dir failed {}", error),
    };

    for entry in fs::read_dir(&migrate_path)? {
        let entry = entry?;
        let path = entry.path();

        let path_str = match path.to_str() {
            Some(s) => s,
            None => continue,
        };

        println!("found file: {}", path_str);

        if !path_str.ends_with("sql") {
            continue;
        }

        println!("its not gzip. readfile");
        progress += 1;
        crate::emit_import_prgress(&app, &progress, 28, "start seeding a file");

        let query = fs::read_to_string(path)?;

        match &query.as_str().get(0..=100) {
            Some(str) => println!("head sql: {}", str),
            None => println!("failed to read sql"),
        };

        let statement = sqlx::query(&query);

        let result = statement.execute(pool).await?;

        println!("rows affected: {}", result.rows_affected());
        progress += 1;
        crate::emit_import_prgress(&app, &progress, 28, "complete seeding a file");
    }
    // let m = sqlx::migrate::Migrator::new(Path::new(&migrate_path)).await?;
    // m.run(*&pool).await?;
    Ok(())
}

/// fetch qandidates
pub async fn query_candidates(pool: &SqlitePool, query: &str) -> DbResult<Vec<RiverCandidate>> {
    println!("{}", query);
    let river_name = format!("%{}%", query);
    const SQL1: &str = "SELECT DISTINCT
      s.riverName,
      s.riverCode,
      s.prefectureName,
      s.prefectureCode
    FROM Stream AS s
    WHERE s.riverName LIKE ?
    ORDER BY s.prefectureCode, s.riverName
    ;
  ";
    println!("{}", SQL1);
    let mut rows = sqlx::query(SQL1).bind(river_name).fetch(pool);

    let mut columns = BTreeMap::new();

    while let Some(row) = rows.try_next().await? {
        let river_name: &str = row.try_get("riverName")?;
        println!("{}", river_name.to_string());
        let river_code: i64 = row.try_get("riverCode")?;
        let prefecture_name: &str = row.try_get("prefectureName")?;
        println!("{}", river_code.to_string());
        columns.insert(
            river_code,
            RiverCandidate::new(river_name, river_code.to_string(), prefecture_name),
        );
    }

    Ok(columns.into_iter().map(|(_k, v)| v).collect())
}

/// check data
pub async fn check_data(pool: &SqlitePool) -> bool {
    println!("check data");

    const SQL1: &str = "SELECT * FROM Point LIMIT 1;";
    const SQL2: &str = "SELECT * FROM Stream LIMIT 1;";

    let mut results: [bool; 2] = [false, false];

    let rows = sqlx::query(SQL1).fetch_one(pool).await;
    match rows {
        Ok(row) => {
            println!("rows {:?}", row.columns());
            results[0] = true;
        }
        Err(e) => println!("{}", e),
    };

    let rows = sqlx::query(SQL2).fetch_one(pool).await;
    match rows {
        Ok(row) => {
            println!("rows {:?}", row.columns());
            results[1] = true;
        }
        Err(e) => println!("{}", e),
    };

    let result = if results.contains(&false) {
        false
    } else {
        true
    };

    result
}

pub async fn search_points(pool: &SqlitePool, bounds: &Bounds) -> DbResult<Vec<Point>> {
    println!("database::search_points");

    const SQL1: &str = "
      SELECT DISTINCT
        coalesce(s.riverName, 'データ欠損') as riverName,
        coalesce(s.riverCode, 0) as riverCode,
        coalesce(s.prefectureName, 'データ欠損') as pref,
        n.id as nid,
        s.id as sid,
        p.*
      FROM Point as p
      LEFT JOIN RiverNode as n on n.position = p.id AND n.prefectureCode = p.prefectureCode
      LEFT JOIN Stream as s on s.startStreamNode = n.id AND p.prefectureCode = s.prefectureCode
      WHERE
        ? <= lat AND lat <= ?
        AND ? <= lng AND lng <= ?
      LIMIT 5000
    ";

    let mut results: [bool; 2] = [false, true];

    let rows = sqlx::query(SQL1)
        .bind(&bounds.min.lat)
        .bind(&bounds.max.lat)
        .bind(&bounds.min.lng)
        .bind(&bounds.max.lng)
        .fetch_all(pool)
        .await;

    let rows2: Vec<SqliteRow>;
    match rows {
        Ok(rows) => {
            println!("rows {:?}", rows[0].columns());
            results[0] = true;
            rows2 = rows;
        }
        Err(e) => {
            println!("{}", e);
            rows2 = vec![];
        }
    };

    let mut columns = BTreeMap::new();

    for (i, row) in rows2.iter().enumerate() {
        let id: &str = row.try_get("id").unwrap();
        let sid: &str = row.try_get("sid").unwrap();
        let nid: &str = row.try_get("nid").unwrap();
        let lat: f32 = row.try_get("lat").unwrap();
        let lng: f32 = row.try_get("lng").unwrap();
        let prefecture_code: u32 = row.try_get("prefectureCode").unwrap();
        let river_name: &str = row.try_get("riverName").unwrap();
        let river_code: i64 = row.try_get("riverCode").unwrap();
        columns.insert(
            i,
            Point::new(
                &id,
                &sid,
                &nid,
                &lat,
                &lng,
                &prefecture_code,
                &river_name,
                &river_code,
            ),
        );
    }

    Ok(columns.into_iter().map(|(_k, v)| v).collect())

    // results.contains(&false)
}

pub async fn curves(pool: &SqlitePool, curve_id: &CurveId) -> DbResult<Vec<Curve>> {
    println!("database::curves");

    // const SQL1: &str = "
    //   SELECT cv.* FROM Curve AS cv
    //   LEFT JOIN Stream AS s ON s.location = cv.id AND s.prefectureCode = cv.prefectureCode
    //   LEFT JOIN RiverNode AS rn ON rn.id = s.startStreamNode AND rn.prefectureCode = s.prefectureCode
    //   LEFT JOIN Point AS p ON p.id = rn.position AND p.prefectureCode = rn.prefectureCode
    //   WHERE p.id = ? AND p.prefectureCode = ?;
    // ";
    const SQL2: &str = "
      SELECT cv.* FROM Point AS p
      LEFT JOIN RiverNode AS rn ON p.id = rn.position AND p.prefectureCode = rn.prefectureCode
      LEFT JOIN Stream AS s ON rn.id = s.startStreamNode AND rn.prefectureCode = s.prefectureCode
      LEFT JOIN Curve AS cv ON s.location = cv.id AND s.prefectureCode = cv.prefectureCode
      WHERE p.id = ? AND p.prefectureCode = ?
      ;
    ";

    let mut results: [bool; 2] = [false, true];

    let rows = sqlx::query(SQL2)
        .bind(&curve_id.pid)
        .bind(&curve_id.pref_code)
        .fetch_all(pool)
        .await;

    let rows2: Vec<SqliteRow>;
    match rows {
        Ok(rows) => {
            println!("rows {:?}", rows[0].columns());
            results[0] = true;
            rows2 = rows;
        }
        Err(e) => {
            println!("{}", e);
            rows2 = vec![];
        }
    };

    let mut columns = BTreeMap::new();

    for (i, row) in rows2.iter().enumerate() {
        let id: &str = row.try_get("id").unwrap();
        let segments: &str = row.try_get("segments").unwrap();
        columns.insert(i, Curve::new(&id, &segments));
    }

    Ok(columns.into_iter().map(|(_k, v)| v).collect())

    // results.contains(&false)
}

pub async fn get_curves(pool: &SqlitePool, curve_key: &CurveKey) -> DbResult<Vec<Curve>> {
    println!("database::get_curves");

    const SQL1: &str = "
      SELECT cv.* FROM Curve AS cv
      LEFT JOIN Stream AS s ON s.location = cv.id AND s.prefectureCode = cv.prefectureCode
      WHERE s.riverCode = ?
        AND s.riverName = ?
      ;
    ";

    let mut results: [bool; 2] = [false, true];

    let rows = sqlx::query(SQL1)
        // .bind(&num_code)
        .bind(&curve_key.river_code)
        .bind(&curve_key.name)
        .fetch_all(pool)
        .await;

    let rows2: Vec<SqliteRow>;
    match rows {
        Ok(rows) => {
            println!("rows {:?}", rows[0].columns());
            results[0] = true;
            rows2 = rows;
        }
        Err(e) => {
            println!("{}", e);
            rows2 = vec![];
        }
    };

    let mut columns = BTreeMap::new();

    for (i, row) in rows2.iter().enumerate() {
        let id: &str = row.try_get("id").unwrap();
        let segments: &str = row.try_get("segments").unwrap();
        columns.insert(i, Curve::new(&id, &segments));
    }

    Ok(columns.into_iter().map(|(_k, v)| v).collect())

    // results.contains(&false)
}
