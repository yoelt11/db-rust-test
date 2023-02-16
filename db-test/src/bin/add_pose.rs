use dbtest::*;
use std::env;

fn main(){
    let connection =  &mut establish_connection();
    let args: Vec<String> = env::args().collect();

    let pose = create_pose(connection, &args[1].to_string());
    println!("\nSaved pose {:?}", pose);
}