use std::{io::{self}, result::Result};
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
    let setting = create_task();
    setting.print_details();
    //LOOP menu to ask what is required to be done
    menu();
}

fn menu() {
    loop {
        //MAIN MENU
        println!("1. Add a new setting to DB");
        println!("2. Read a setting from DB");
        println!("3. Update a setting in DB");
        println!("4. Delete a setting from DB");
        println!("Anything else to Exit");
        let mut input = String::from("");
        io::stdin()
            .read_line(&mut input)
            .expect("Couldnt read line");
        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            }
        };

        match input {
            1 => add_details(1),
            2 => read_details(1),
            3 => update_details(1),
            4 => delete_setting(1),
            _ => {println!("Exiting!"); break;}
        }
    }
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

//CRUD functionality
fn add_details(id: usize) {  
    println!("Add");
}

fn read_details(id: usize) {
    println!("read");
}

fn update_details(id: usize) {
    println!("update");
}

fn delete_setting(id:usize) {
    println!("Delete");
}
