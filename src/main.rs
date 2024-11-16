use std::{io::{self}, result::Result};
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Error, Row, Sqlite, SqlitePool};
use rust_sqlite::Settings;

async fn create_schema() -> Result<SqliteQueryResult,sqlx::Error> {
    let pool = SqlitePool::connect(&DB_URL).await?;
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

const DB_URL: &str = "sqlite://sqlite.db";

#[async_std::main]
async fn main() {
    if !Sqlite::database_exists(&DB_URL).await.unwrap_or(false) {
        Sqlite::create_database(&DB_URL).await.unwrap();
        match create_schema().await {
            Ok(_) => println!("Database created sucessfully"),
            Err(e) => panic!("{}",e)
        }
    }
    //LOOP menu to ask what is required to be done
    menu().await;
}

async fn menu() {
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
            1 => _ = add_details().await,
            2 => _ = read_details().await,
            3 => _ = update_details().await,
            4 => _ = delete_setting().await,
            _ => {println!("Exiting!"); break;}
        }
    }
}

fn create_setting() -> Settings {
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
async fn add_details() -> Result<SqliteQueryResult,sqlx::Error>{  
    println!("Add");
    let setting = create_setting();
    let qry = "INSERT INTO settings (settings_id, description,done) VALUES(?,?,?)";
    let pool = SqlitePool::connect(&DB_URL).await?;
    let result = sqlx::query(qry)
                                                            .bind(setting.id)
                                                            .bind(setting.description)
                                                            .bind(setting.done)
                                                            .execute(&pool)
                                                            .await;
    result

}


async fn read_details() -> Result<Settings, Error> {
    println!("read");
    let pool = SqlitePool::connect(&DB_URL).await?;
    let mut id = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line");
    let id:i32 = id.trim().parse().expect("Expected Number");
    let mut setting = Settings::new(0, String::from("value"));
    // SQL query to select all rows
    let qry = "SELECT settings_id, description, done FROM settings WHERE settings_id = ?";
    let rows = sqlx::query(qry).bind(id).fetch_all(&pool).await;
    let result = match rows {
        Ok(val) => val,
        Err(e) => panic!("{}",e)
    };
    for row in result {
        let description: String = row.get("description");
        let id: i32 = row.get("settings_id");
        println!("The description of the setting with {}",id);
        println!("is {}",description);
        setting = Settings::new(id, description);
    }
    Ok(setting)
}

async fn update_details() -> Result<(),Error> {
    println!("update");
    println!("Enter the id of the setting that you would like to update");
    let mut id = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Expected an id");
    let id: i32 = id.trim().parse().expect("expected a number");
    let mut new_desc = String::new();
    io::stdin()
        .read_line(&mut new_desc)
        .expect("Failed to read line");
    let qry = "UPDATE settings SET description = ? WHERE settings_id = ?";
    let pool = SqlitePool::connect(DB_URL).await?;
    let result = sqlx::query(qry)
                                                        .bind(new_desc)
                                                        .bind(id)
                                                        .execute(&pool)
                                                        .await?;
    if result.rows_affected() > 0 {
        println!("Updated");
    } else {
        println!("No Setting with that ID");
    }
    return Ok(())
}

async fn delete_setting() -> Result<(),Error> {
    println!("Delete");
    println!("Enter the id of the setting that you would like to Delete");
    let mut id = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Expected an id");
    let id: i32 = id.trim().parse().expect("expected a number");
    let pool = SqlitePool::connect(DB_URL).await?;
    let qry = "DELETE FROM settings where settings_id = ?";
    let result = sqlx::query(qry)
                                                        .bind(id)
                                                        .execute(&pool)
                                                        .await?;
    if result.rows_affected() > 0 {
        println!("Deleted!");
    } else {
        println!("No rows with the id you shared");
    }
    return Ok(())

}
