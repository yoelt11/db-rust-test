use crate::establish_connection;
use clap::{ArgMatches};
use crate::models::*;
use diesel::prelude::*;

pub fn get_room(matches:&ArgMatches){
    // get variables from cli
    let object_names = matches.get_many::<String>("objects").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
    println!("adding object: {:?}", object_names);   
    
    // perform query
    // generate connection to db 
    let connection =  &mut establish_connection();
}

pub fn get_tier1(matches:&ArgMatches){
    // get variables from cli
    let pose_name = matches.get_one::<String>("pose").unwrap();   
    let room_names = matches.get_many::<String>("rooms").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
    let object_names = matches.get_many::<String>("objects").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();

    println!("adding pose: {:?}", pose_name);   
    println!("adding object: {:?}", object_names);   
    println!("adding rooms: {:?}", room_names);   
    
    // perform query
    // generate connection to db 
    let connection =  &mut establish_connection();
}
pub fn get_tier2(matches:&ArgMatches){

    // Get arguments from cli 
    let tier1_name = matches.get_one::<String>("tier1").unwrap();   
    // Kp-hits are obtained as a flattened list e.g: {-k l_ear cellphone -k r_hand cellphone} -> [l_ear, cellphone, r_hand cellphone]
    let kph = matches.get_many::<String>("keypoint-hits").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
    // here we convert them into pairs -> [(l_ear, cellphone), (r_hand, cellphone)]
    let kph_pairs: Vec<_> = kph.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();

    // See what is being appended
    println!("tier1  {:?}", tier1_name);
    println!("keypoint hits {:?}", kph_pairs);
    
    // perform query
    // generate connection to db 
    let connection =  &mut establish_connection();
}