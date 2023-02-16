use dbtest::*;
use std::env;
use diesel::prelude::*;
use crate::schema::{rooms, objects, room_object};
use crate::models::{Room};

fn main(){
    
    let args: Vec<String> = env::args().collect();
    let object_name = args[1].as_str();

    let connection =  &mut establish_connection();

    let rooms = rooms::table
        .inner_join(room_object::table.on(room_object::room_id.eq(rooms::room_id)))
        .inner_join(objects::table.on(room_object::object_id.eq(objects::object_id)))
        .filter(objects::name.eq(object_name))
        .select(rooms::all_columns)
        .load::<Room>(connection)
        .expect("Error getting rooms from objects");

    println!("Rooms matching to room: {:?}", rooms);
}