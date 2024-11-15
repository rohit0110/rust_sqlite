use std::{io, result::Result};
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};
use rust_sqlite::Settings;

async fn create_schema(db_url: &str) -> Result<SqliteQueryResult,sqlx::Error> {
    let pool = SqlitePool::connect(&db_url).await?;
    let qry = 
    "PRAGMA foreign_keys = ON;
    CREATE TABLE IF NOT EXISTS settings
    (
        settings_id     INTEGER PRIMARY KEY NOT NULL,
        description     TEXT                NOT NULL,
        created_on      DATETIME DEFAULT    (datetime('now','localtime')),
        updated_on      DATETIME DEFAULT    (datetime('now','localtime')),
        done            BOOLEAN             NOT NULL DEFAULT 0
    );
    ";
    let result = sqlx::query(&qry).execute(&pool).await;
    pool.close().await;
    return result;
}

#[async_std::main]
async fn main() {
    let db_url = String::from("sqlite://sqlite.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await {
            Ok(_) => println!("Database created sucessfully"),
            Err(e) => panic!("{}",e)
        }
    }

    let instances = SqlitePool::connect(&db_url).await.unwrap();
    let qry = "INSERT INTO settings (description) VALUES($1)";
    let result = sqlx::query(&qry).bind("testing").execute(&instances).await;
    instances.close().await;
    let setting = create_task();
    setting.print_details();
    println!("{:?}",result);

}

fn create_task() -> Settings {
    let mut id = String::new();
    let mut description = String::new();

    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line");
    let id = id.trim().parse().expect("Expected Number");
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
    let setting = Settings::new(id,description);
    setting
}
