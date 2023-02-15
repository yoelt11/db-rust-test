use self::models::*;
use diesel::prelude::*;
use dbtest::*;
use std::env;

fn main() {
    use self::schema::keypoints::dsl::*;

    let connection = establish_connection;

    let args: Vec<String> = env::args().collect();

    let results = keypoints.filter(keypoint_id.eq(args[1].parse::<i32>().unwrap()))
        .load::<Keypoint>(&mut connection())
        .expect("Error loading keypoint");

    println!("Keypoint {:?}", results);    
}