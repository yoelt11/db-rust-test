pub mod add_entries;
pub mod get_entries;
pub mod show_entries;
pub mod input_types;
pub mod json_data_structs;
use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use std::env;
pub mod models;
pub mod schema;
//use self::models::{NewKeypoint, NewObject, NewPose, NewRoom, NewRoomObject};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}