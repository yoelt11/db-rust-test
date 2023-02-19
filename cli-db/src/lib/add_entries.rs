use clap::{ArgMatches, builder::Str};
use crate::models::{NewKeypoint, NewObject, NewPose, NewRoom, NewRoomObject};
use crate::establish_connection;
use diesel::prelude::*;

pub fn add_room(matches:&ArgMatches){
    use crate::schema::rooms;

    // gets keypoint from cli
    let room = matches.get_one::<String>("room").unwrap();   
    println!("adding room: {:?}", room);   

    // generate connection to db 
    let connection =  &mut establish_connection();
    // create structure compatible with databae
    let new_room = NewRoom { name:room };

    // inserts to database
    diesel::insert_or_ignore_into(rooms::table)
        .values(&new_room)
        .execute(connection)
        .expect("Error saving keypoint");
}

//TODO
pub fn add_object(matches:&ArgMatches){
    use crate::schema::objects;
    use crate::schema::room_object;
    use crate::schema::rooms;
    use crate::models::Object;
    use crate::models::Room;

    // get inputs from cli
    let object = matches.get_one::<String>("object").unwrap();
    let rooms = matches.get_many::<String>("rooms").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
    println!("adding object: {:?}", object);   
    println!("adding rooms: {:?}", rooms);   
    
    // generate connection to db 
    let connection =  &mut establish_connection();
    
    // generate connection to db
    let new_object = NewObject { name: object};
    diesel::insert_or_ignore_into(objects::table)
        .values(&new_object)
        .execute(connection).unwrap(); //  unwrap unsafe must pass error
                                       //
    // for each room create if it doesnt exists
    let new_rooms = rooms
        .iter()
        .map(|room_name| NewRoom { name: room_name })
        .collect::<Vec<NewRoom>>();

    diesel::insert_or_ignore_into(rooms::table)
        .values(&new_rooms)
        .execute(connection).unwrap();
    
       // load rooms and object once
       let rooms = rooms::table
       .filter(rooms::name.eq_any(rooms))
       .load::<Room>(connection).unwrap();
   let object = objects::table
       .filter(objects::name.eq(object))
       .first::<Object>(connection).unwrap();

   // associate objects with rooms
   let room_objects = rooms
       .iter()
       .map(|room| NewRoomObject {
           room_id: room.room_id,
           object_id: object.object_id,
       })
       .collect::<Vec<NewRoomObject>>();
   // insert to table
   diesel::insert_or_ignore_into(room_object::table)
       .values(room_objects)
       .execute(connection).unwrap();
}    

pub fn add_keypoint(matches:&ArgMatches){
    use crate::schema::keypoints;

    // gets keypoint from cli
    let keypoint = matches.get_one::<String>("keypoint").unwrap();   
    println!("adding keypoint: {:?}", keypoint);   

    // generate connection to db 
    let connection =  &mut establish_connection();
    // create structure compatible with databae
    let new_keypoint = NewKeypoint { name:keypoint };

    // inserts to database
    diesel::insert_or_ignore_into(keypoints::table)
        .values(&new_keypoint)
        .execute(connection)
        .expect("Error saving keypoint");
}    

pub fn add_poses(matches:&ArgMatches){
    let pose = matches.get_one::<String>("pose").unwrap();   
    println!("adding pose: {:?}", pose);   
    use crate::schema::poses;

    // gets keypoint from cli
    let pose = matches.get_one::<String>("pose").unwrap();   
    println!("adding pose: {:?}", pose);   

    // generate connection to db 
    let connection =  &mut establish_connection();
    // create structure compatible with databae
    let new_pose = NewPose { name:pose };

    // inserts to database
    diesel::insert_or_ignore_into(poses::table)
        .values(&new_pose)
        .execute(connection)
        .expect("Error saving keypoint");
}    

//TODO
pub fn add_tier1(matches:&ArgMatches){
    let tier1 = matches.get_one::<String>("tier1").unwrap();   
    let pose = matches.get_one::<String>("pose").unwrap();   
    let rooms = matches.get_many::<String>("rooms").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
    let objects = matches.get_many::<String>("object").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();

    println!("adding tier1: {:?}", tier1);   
    println!("adding pose: {:?}", pose);   
    println!("adding object: {:?}", objects);   
    println!("adding rooms: {:?}", rooms);   
}    

//TODO
pub fn add_tier2(matches:&ArgMatches){
    let tier2 = matches.get_one::<String>("tier2").unwrap();   
    let tier1 = matches.get_one::<String>("tier1").unwrap();   
    let kph = matches.get_many::<String>("keypoint-hits").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
    let pairs: Vec<_> = kph.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();

    println!("tier1  {:?}", tier1);
    println!("tier2  {:?}", tier2);
    println!("keypoint hits {:?}", pairs);
}    