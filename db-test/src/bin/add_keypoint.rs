use dbtest::*;
use std::env;

fn main(){
    let connection =  &mut establish_connection();
    let args: Vec<String> = env::args().collect();

    let keypoint = create_keypoint(connection, &args[1].to_string());
    println!("\nSaved keypoint{:?}", keypoint);
}