use diesel::prelude::*;
use crate::schema::*;

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

// Room-Object table
#[derive(Insertable)]
#[diesel(table_name = room_object)]
pub struct NewRoomObject {
    pub room_id: i32,
    pub object_id: i32,
}

// Models: Pose
#[derive(Debug, Queryable)]
pub struct Pose {
    pub pose_id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = poses)]
pub struct NewPose<'a> {
    pub name: &'a str,
}

// Models: Tier1Activities
#[derive(Debug, Queryable)]
pub struct Tier1Activities {
    pub tier1_id: i32,
    pub tier1: String,
}

#[derive(Insertable)]
#[diesel(table_name = tier1activities)]
pub struct NewTier1Activity<'a> {
    pub tier1: &'a str,
}

// Models: Tier1Activities relations
#[derive(Insertable)]
#[diesel(table_name = tier1_activity_objects)]
pub struct NewTier1Object {
    pub object_id: i32,
    pub tier1_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tier1_activity_rooms)]
pub struct NewTier1Room {
    pub room_id: i32,
    pub tier1_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tier1_activity_poses)]
pub struct NewTier1Pose {
    pub pose_id: i32,
    pub tier1_id: i32,
}

// Models: Tier2Activities
#[derive(Debug, Queryable)]
pub struct Tier2Activities {
    pub tier2_id: i32,
    pub tier2: String,
}

#[derive(Insertable)]
#[diesel(table_name = tier2activities)]
pub struct NewTier2Activity<'a> {
    pub tier2: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = tier2_activity_poses)]
pub struct NewTier2Poses {
    pub pose_id: i32,
    pub tier2_id: i32,
}

// Tier2 relations
#[derive(Insertable)]
#[diesel(table_name = tier2_tier1_kph)]
pub struct NewTier2Tier1Kph {
    pub tier2_id: i32,
    pub tier1_id: i32,
    pub kph_id: i32,
}

// Models: KPH
#[derive(Debug, Queryable)]
pub struct KeypointHits {
    pub kph_id: i32,
    pub object_id: i32,
    pub keypoint_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = keypoint_hits)]
pub struct NewKPH {
    pub object_id: i32,
    pub keypoint_id: i32
}
