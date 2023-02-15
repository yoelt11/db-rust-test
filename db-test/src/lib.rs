use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::models::{NewKeypoint, NewRoom, NewObject};

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

pub fn create_room(conn: &mut SqliteConnection, name: &str) {

    use crate::schema::rooms;

    let new_room = NewRoom {name};
    
    diesel::insert_or_ignore_into(rooms::table)
        .values(&new_room)
        .execute(conn)
        .expect("Error saving room");
}

pub fn create_object(conn: &mut SqliteConnection, object_name: &str, room_list: Vec<&str>){
    // sqlite interface does not support returns when adding an entry
    use crate::schema::objects;
    use crate::schema::rooms;
    use crate::schema::room_object;
    use crate::models::Room;
    use crate::models::Object;
    //use self::schema::rooms::dsl::*;
    //use self::schema::objects::dsl::*;


    // for each room create if it doesnt exists
    let new_rooms = room_list.iter().map(|room_name| NewRoom{
        name:room_name
    }).collect::<Vec<NewRoom>>();
    
    diesel::insert_or_ignore_into(rooms::table)
        .values(&new_rooms)
        .execute(conn)
        .expect("Error saving room");
    
    // insert new object to table
    let new_object = NewObject {name: object_name};
    diesel::insert_or_ignore_into(objects::table)
        .values(&new_object)
        .execute(conn)
        .expect("Error saving object");

    // associate objects with rooms
        // get the ids of the rooms as vector
    let room_ids = room_list.iter().map(|room_name| {rooms::table.filter(rooms::name.eq(room_name).id)
        .load::<Room>(&mut conn)
        .expect("Error loading rooms")}).collect::<Vec<i32>>();
        // get the ids of the objects as int
    let object_id = objects::table.filter(objects::name.eq(object_name))
        .load::<Object>(conn)
        .expect("Error loading rooms");
        // create (objec_id, room_id) pairs
    let room_objects = room_ids.iter().map(|room_id| {
        (object_id, room_id)
    });
        // insert to table
    diesel::insert_or_ignore_into(room_object::table)
        .values(&room_objects)
        .execute(conn)
        .expect("Error saving room-objects");

}