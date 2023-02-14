use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::models::{NewKeypoint};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_keypoint(conn: &mut SqliteConnection, name: &str) {

    use crate::schema::keypoints;

    let new_keypoint = NewKeypoint {name};
    
    diesel::insert_or_ignore_into(keypoints::table)
        .values(&new_keypoint)
        .execute(conn)
        .expect("Error saving keypoint");
}