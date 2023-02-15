use diesel::prelude::*;
use crate::schema::{keypoints, rooms, objects};

// Models: Keypoint
#[derive(Debug, Queryable)]
pub struct Keypoint {
    pub keypoint_id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = keypoints)]
pub struct NewKeypoint<'a> {
    pub name: &'a str,
}

// Models: Rooms
#[derive(Debug, Queryable)]
pub struct Room {
    pub room_id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = rooms)]
pub struct NewRoom<'a> {
    pub name: &'a str,
}

// Models: Objects
#[derive(Debug, Queryable)]
pub struct Object {
    pub object_id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = objects)]
pub struct NewObject<'a> {
    pub name: &'a str,
}