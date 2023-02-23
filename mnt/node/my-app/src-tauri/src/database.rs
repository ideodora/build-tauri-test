use std::{collections::BTreeMap, path::Path, path::PathBuf, result, str::FromStr};

use futures::TryStreamExt;
use sqlx::{
    sqlite::{
        SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteRow, SqliteSynchronous,
    },
    Execute, QueryBuilder, Row, Sqlite, SqlitePool,
};
use std::fs;
use tauri::AppHandle;

use crate::handlers::{
    create_watershed::CreateBody, curves::CurveId, get_curves::CurveKey,
    update_watershed::UpdateWatershedRequest, update_watershed_item::UpdateWatershedItemRequest,
    watersheds::DbResultWatershed, Bounds, Curve, Point, RiverCandidate,
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

    let ppp = columns.into_iter().map(|(_k, v)| v).collect();
    Ok(ppp)

    // results.contains(&false)
}

pub async fn create_watershed(pool: &SqlitePool, body: &CreateBody) -> DbResult<()> {
    println!("database::create_watershed");

    let mut query_results: [bool; 2] = [false, true];

    const SQL1: &str = "
      INSERT INTO watersheds (name)
      VALUES (?)
      ;
    ";

    let row = sqlx::query(SQL1).bind(&body.name).execute(pool).await;

    let watershed_id = match row {
        Ok(row) => {
            println!("row: {:?}", row);
            let insert_id = u32::try_from(row.last_insert_rowid()).unwrap();
            println!("last_insert_rowid: {}", insert_id);
            query_results[0] = true;
            insert_id
        }
        Err(e) => {
            println!("{}", e);
            panic!("failed insert watershed")
        }
    };

    let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO watershed_items (watershed_id, `key`, data) ");

    query_builder.push_values(body.data.iter(), |mut b, source| {
        b.push_bind(&watershed_id)
            .push_bind(&source.0)
            .push_bind(&source.1);
    });

    let query = query_builder.build();

    let rows = query.execute(pool).await;

    match rows {
        Ok(rows) => {
            println!("rows: {:#?}", rows);
            query_results[1] = true;
        }
        Err(e) => {
            println!("{}", e);
        }
    };

    println!("done inserts");
    Ok(())
}

pub async fn watersheds(pool: &SqlitePool) -> DbResult<Vec<DbResultWatershed>> {
    println!("database::watershed");

    const SQL1: &str = "
      -- SQLite
      SELECT
        watersheds.*,
        item.id AS item__id,
        item.`key` AS item__key,
        item.data AS item__data,
        item.name AS item__name
      FROM
        watersheds
        LEFT JOIN watershed_items AS item ON item.watershed_id = watersheds.id
      ;
    ";

    let rows = sqlx::query(SQL1).fetch_all(pool).await;

    let rows2 = match rows {
        Ok(a) => {
            println!("rows {:?}", a[0].columns());
            a
        }
        Err(e) => {
            println!("{}", e);
            panic!("failed to query")
        }
    };

    let mut columns = BTreeMap::new();

    let hoge = rows2.iter();
    let piyo = hoge.enumerate();

    for (i, row) in piyo {
        let id: u32 = row.try_get("id").unwrap();
        let name: &str = row.try_get("name").unwrap();
        // let bounds: &str = row.try_get("bounds").unwrap();
        let item_id: u32 = row.try_get("item__id").unwrap();
        let item_key: &str = row.try_get("item__key").unwrap();
        let item_data: &str = row.try_get("item__data").unwrap();
        let item_name: Option<&str> = row.try_get("item__name").ok();
        columns.insert(
            i,
            DbResultWatershed::new(id, &name, item_id, &item_key, &item_data, item_name),
        );
    }

    let results = columns.into_iter().map(|(_, v)| v).collect();

    Ok(results)
}

pub async fn delete_watershed(pool: &SqlitePool, id: &u32) {
    println!("database::delete_watershed");

    const SQL1: &str = "
      -- SQLite
      pragma foreign_keys = ON;

      DELETE FROM watersheds
      WHERE
        id = ?
      ;
    ";

    let row = sqlx::query(SQL1).bind(&id).execute(pool).await;

    match row {
        Ok(row) => {
            println!("delete success");
        }
        Err(e) => {
            println!("{:#?}", e);
            panic!("delete error");
        }
    }
}

pub async fn update_watershed(pool: &SqlitePool, payload: &UpdateWatershedRequest) {
    println!("database::update_watershed");

    const SQL1: &str = "
    -- SQLite
    UPDATE watersheds
    SET name = ?
    WHERE
      id = ?
    ;
  ";

    let row = sqlx::query(SQL1)
        .bind(&payload.name)
        .bind(&payload.id)
        .execute(pool)
        .await;

    match row {
        Ok(_row) => {
            println!("update success");
        }
        Err(e) => {
            println!("{:#?}", e);
            panic!("update error");
        }
    }
}

pub async fn update_watershed_item(pool: &SqlitePool, payload: &UpdateWatershedItemRequest) {
    println!("database::update_watershed_item");

    const SQL1: &str = "
    -- SQLite
    UPDATE watershed_items
    SET name = ?
    WHERE
      id = ?
    ;
  ";

    let row = sqlx::query(SQL1)
        .bind(&payload.name)
        .bind(&payload.id)
        .execute(pool)
        .await;

    match row {
        Ok(_row) => {
            println!("update success");
        }
        Err(e) => {
            println!("{:#?}", e);
            panic!("update error");
        }
    }
}
