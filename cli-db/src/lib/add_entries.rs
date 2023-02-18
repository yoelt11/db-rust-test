use clap::ArgMatches;


pub fn add_room(matches:&ArgMatches){
    println!("adding room: {:?}", matches.get_one::<String>("room").unwrap());   
    
}

pub fn add_object(matches:&ArgMatches){
    let object = matches.get_one::<String>("object").unwrap();
    let rooms = matches.get_one::<String>("rooms").unwrap(); // must conver to list of strings
    println!("adding object: {:?}", object);   
    println!("adding rooms: {:?}", rooms);   
}    

pub fn add_keypoint(matches:&ArgMatches){
    println!("adding keypoint: {:?}", matches.get_one::<String>("kepoint").unwrap());   
}    

pub fn add_poses(matches:&ArgMatches){
    println!("adding pose: {:?}", matches.get_one::<String>("pose").unwrap());   
}    

pub fn add_tier1(matches:&ArgMatches){
    println!("adding tier1 activity: {:?}", matches.get_one::<String>("tier1").unwrap());   
}    
pub fn add_tier2(matches:&ArgMatches){
    println!("adding tier2 activity: {:?}", matches.get_one::<String>("tier2").unwrap());   
    // must parse keypoint hits
}    