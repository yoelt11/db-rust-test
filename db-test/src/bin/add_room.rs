use dbtest::*;
use std::env;

fn main(){
    let connection =  &mut establish_connection();
    let args: Vec<String> = env::args().collect();

    let room = create_room(connection, &args[1].to_string());
    println!("\nSaved room {:?}", room);
}