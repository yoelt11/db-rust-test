use self::models::*;
use diesel::prelude::*;
use dbtest::*;
use std::env;

fn main() {
    use self::schema::rooms::dsl::*;

    let connection = establish_connection;

    let args: Vec<String> = env::args().collect();

    let results = rooms.filter(room_id.eq(args[1].parse::<i32>().unwrap()))
        .load::<Room>(&mut connection())
        .expect("Error loading rooms");

    println!("Rooms {:?}", results);    
}