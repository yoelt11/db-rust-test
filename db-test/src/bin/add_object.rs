use dbtest::*;
use std::env;
// must add arg options --o apple -r bedroom kitchen
fn main(){
    let connection =  &mut establish_connection();
    let args: Vec<String> = env::args().collect();
    let obj = args[1].as_str();

    let result = create_object(connection, obj,args.get(2..)
                                    .unwrap().iter().map(|s| s.as_str()).collect::<Vec<&str>>());
    println!("\nSaved object {:?}", result);
}