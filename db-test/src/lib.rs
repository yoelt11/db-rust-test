use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::models::{NewKeypoint, NewRoom, NewObject, NewPose, NewRoomObject};

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

pub fn create_object(conn: &mut SqliteConnection, object_name: &str, room_list: Vec<&str>) -> Result<(), diesel::result::Error>{
    // sqlite interface does not support returns when adding an entry
    use crate::schema::objects;
    use crate::schema::rooms;
    use crate::schema::room_object;
    use crate::models::Room;
    use crate::models::Object;
    
    // for each room create if it doesnt exists
    let new_rooms = room_list.iter().map(|room_name| NewRoom {
        name: room_name,
    }).collect::<Vec<NewRoom>>();

    diesel::insert_or_ignore_into(rooms::table)
        .values(&new_rooms)
        .execute(conn)?;

    // insert new object to table
    let new_object = NewObject { name: object_name };
    diesel::insert_or_ignore_into(objects::table)
        .values(&new_object)
        .execute(conn)?;

    // load rooms and object once
    let rooms = rooms::table.filter(rooms::name.eq_any(room_list))
        .load::<Room>(conn)?;
    let object = objects::table.filter(objects::name.eq(object_name))
        .first::<Object>(conn)?;

    // associate objects with rooms
    let room_objects = rooms.iter().map(|room| {
        NewRoomObject {
            room_id: room.room_id,
            object_id: object.object_id,
        }
    }).collect::<Vec<NewRoomObject>>();
    // insert to table
    diesel::insert_or_ignore_into(room_object::table)
        .values(room_objects)
        .execute(conn)?;

    Ok(())
}

pub fn create_pose(conn: &mut SqliteConnection, name: &str) {

    use crate::schema::poses;

    let new_pose = NewPose {name};
    
    diesel::insert_or_ignore_into(poses::table)
        .values(&new_pose)
        .execute(conn)
        .expect("Error saving keypoint");
}