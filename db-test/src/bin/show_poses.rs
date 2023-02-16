use self::models::*;
use diesel::prelude::*;
use dbtest::*;
use std::env;

fn main() {
    use self::schema::poses::dsl::*;

    let connection = establish_connection;

    let args: Vec<String> = env::args().collect();

    let results = poses.filter(pose_id.eq(args[1].parse::<i32>().unwrap()))
        .load::<Pose>(&mut connection())
        .expect("Error loading pose");

    println!("pose {:?}", results);    
}