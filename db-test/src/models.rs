use diesel::prelude::*;
use crate::schema::keypoints;

#[derive(Queryable)]
pub struct Keypoint {
    pub keypoint_id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = keypoints)]
pub struct NewKeypoint<'a> {
    pub name: &'a str,
}