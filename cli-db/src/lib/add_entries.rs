use clap::{ArgMatches};
use crate::models::*;
use crate::establish_connection;
use diesel::prelude::*;
use std::error::Error;

/// Adds a room to database.
/// Currently it only available fot cli-app
pub fn add_room(matches:&ArgMatches) -> Result<(), Box<dyn Error>>{
    use crate::schema::rooms;

    // gets keypoint from cli
    let room = matches.get_one::<String>("room")
                                        .ok_or("Missing required argument: room")?;                                     
    println!("adding room: {:?}", room);   

    // generate connection to db 
    let connection =  &mut establish_connection();
    // create structure compatible with databae
    let new_room = NewRoom { name:room };

    // inserts to database
    diesel::insert_or_ignore_into(rooms::table)
        .values(&new_room)
        .execute(connection)
        .map_err(|err| format!("Error saving room: {:?}", err))?;

    Ok(())
}

/// Adds a object to database.
/// Currently it only available fot cli-app
pub fn add_object(matches:&ArgMatches) -> Result<(), Box<dyn Error>>{
    use crate::schema::objects;
    use crate::schema::room_object;
    use crate::schema::rooms;

    // get inputs from cli
    let object_name = matches.get_one::<String>("object").
                                    ok_or("Missing argument object_name")?;
    let room_names = matches.get_many::<String>("rooms")
                                       .unwrap_or_default()
                                       .map(|v| v.as_str()).collect::<Vec<_>>();
    println!("adding object: {:?}", object_name);   
    println!("adding rooms: {:?}", room_names);   
    
    // generate connection to db 
    let connection =  &mut establish_connection();
    
    //  create the object if it doesnt exists
    let new_object = NewObject { name: object_name};
    diesel::insert_or_ignore_into(objects::table)
        .values(&new_object)
        .execute(connection)?; //  unwrap unsafe must pass error
                                       //
    // for each room create if it doesnt exists
    let new_rooms = room_names
        .iter()
        .map(|room_name| NewRoom { name: room_name })
        .collect::<Vec<NewRoom>>();

    diesel::insert_or_ignore_into(rooms::table)
        .values(&new_rooms)
        .execute(connection)?;
    
    // load rooms and object once
    let rooms = rooms::table
       .filter(rooms::name.eq_any(room_names))
       .load::<Room>(connection)?;
    let object = objects::table
       .filter(objects::name.eq(object_name))
       .first::<Object>(connection)?;

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
       .execute(connection)?;

    Ok(())
}    

/// Adds a keypoint to database.
/// Currently it only available fot cli-app
pub fn add_keypoint(matches: &ArgMatches) -> Result<(), Box<dyn Error>>{
    use crate::schema::keypoints;

    // gets keypoint from cli
    let keypoint = matches.get_one::<String>("keypoint")
                                                      .ok_or("Missing argument keypoint")?;   
    println!("adding keypoint: {:?}", keypoint);   

    // generate connection to db 
    let connection =  &mut establish_connection();
    // create structure compatible with databae
    let new_keypoint = NewKeypoint { name:keypoint };

    // inserts to database
    diesel::insert_or_ignore_into(keypoints::table)
        .values(&new_keypoint)
        .execute(connection)
        .map_err(|err| format!("Error saving keypoint: {}", err))?;

    Ok(())
}    

/// Adds a poses to database.
/// Currently it only available fot cli-app
pub fn add_poses(matches:&ArgMatches) -> Result<(), Box<dyn Error>>{
    use crate::schema::poses;

    // gets keypoint from cli
    let pose = matches.get_one::<String>("pose")
                                .ok_or("Missing argument pose")?;   
    println!("adding pose: {:?}", pose);   

    // generate connection to db 
    let connection =  &mut establish_connection();
    // create structure compatible with databae
    let new_pose = NewPose { name:pose };

    // inserts to database
    diesel::insert_or_ignore_into(poses::table)
        .values(&new_pose)
        .execute(connection)
        .map_err(|err| format!("Error saving keypoint: {}", err))?;

    Ok(())
}    

/// Adds a tier1 to database.
/// Currently it only available fot cli-app
pub fn add_tier1(matches:&ArgMatches) -> Result<(), Box<dyn Error>>{
    // schema.rs
    use crate::schema::objects;
    use crate::schema::tier1_activity_objects;
    use crate::schema::rooms;
    use crate::schema::tier1_activity_rooms;
    use crate::schema::poses;
    use crate::schema::tier1_activity_poses;
    use crate::schema::tier1activities;

    // get variables from cli
    let tier1_name = matches.get_one::<String>("tier1")
                                             .ok_or("Missing argument tier1_name")?;   
    let pose_names = matches.get_many::<String>("poses").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();   
    let room_names = matches.get_many::<String>("rooms").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
    let object_names = matches.get_many::<String>("object").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();

    println!("adding tier1: {:?}", tier1_name);   
    println!("adding pose: {:?}", pose_names);   
    println!("adding object: {:?}", object_names);   
    println!("adding rooms: {:?}", room_names);   

    // generate connection to db 
    let connection =  &mut establish_connection();

    // create tier 1 activity if not exists
    let new_activity = NewTier1Activity {tier1: tier1_name};
    diesel::insert_or_ignore_into(tier1activities::table)
        .values(&new_activity)
        .execute(connection)?; //  unwrap unsafe must pass error

    // create pose if not exists
    let new_poses = pose_names
        .iter()
        .map(|pose_name| NewPose { name: pose_name })
        .collect::<Vec<NewPose>>();

    diesel::insert_or_ignore_into(poses::table)
        .values(&new_poses)
        .execute(connection)?; //  unwrap unsafe must pass error

    // create rooms if not exists
    let new_rooms = room_names
        .iter()
        .map(|room_name| NewRoom { name: room_name })
        .collect::<Vec<NewRoom>>();

    diesel::insert_or_ignore_into(rooms::table)
        .values(&new_rooms)
        .execute(connection)?;

    // create objects if not exists
    let new_objects = object_names
        .iter()
        .map(|object_name| NewObject { name: object_name })
        .collect::<Vec<NewObject>>();

    diesel::insert_or_ignore_into(objects::table)
        .values(&new_objects)
        .execute(connection)?;

    // load model items 
    let rooms = rooms::table
       .filter(rooms::name.eq_any(room_names))
       .load::<Room>(connection)?;
    
    let objects = objects::table
       .filter(objects::name.eq_any(object_names))
       .load::<Object>(connection)?;

    let tier1 = tier1activities::table
       .filter(tier1activities::tier1.eq(tier1_name))
       .first::<Tier1Activities>(connection)?;

    let poses = poses::table
       .filter(poses::name.eq_any(pose_names))
       .load::<Pose>(connection)?;
    
    // create relationship tables: Rooms-> Tier1  
    let room_tier1 = rooms
       .iter()
       .map(|room| NewTier1Room {
           room_id: room.room_id,
           tier1_id: tier1.tier1_id,
       })
       .collect::<Vec<NewTier1Room>>();

    diesel::insert_or_ignore_into(tier1_activity_rooms::table)
       .values(room_tier1)
       .execute(connection)?;
    
    // create relationship tables: Objects-> Tier1  
    let object_tier1 = objects
       .iter()
       .map(|object| NewTier1Object {
           object_id: object.object_id,
           tier1_id: tier1.tier1_id,
       })
       .collect::<Vec<NewTier1Object>>();

    diesel::insert_or_ignore_into(tier1_activity_objects::table)
       .values(object_tier1)
       .execute(connection)?;

    // create relationship tables: Pose-> Tier1  
    let pose_tier1 = poses
       .iter()
       .map(|pose| NewTier1Pose {
           pose_id: pose.pose_id,
           tier1_id: tier1.tier1_id,
       })
       .collect::<Vec<NewTier1Pose>>();

    diesel::insert_or_ignore_into(tier1_activity_poses::table)
       .values(pose_tier1)
       .execute(connection)?;

       Ok(())

}    

/// 
/// Adds a tier2 to database.
/// Currently it only available fot cli-app
pub fn add_tier2(matches:&ArgMatches) -> Result<(), Box<dyn Error>>{
    // schema.rs
    use crate::schema::tier1activities; 
    use crate::schema::tier2activities;
    use crate::schema::objects;
    use crate::schema::keypoints;
    use crate::schema::tier2_tier1_kph;
    use crate::schema::keypoint_hits;

    // Get arguments from cli 
    let tier2_name = matches.get_one::<String>("tier2")
                                  .ok_or("Missing Argument tier2_name")?;   
    let tier1_name = matches.get_one::<String>("tier1")
                                                        .ok_or("Missing argument tier1_name")?;   
    // Kp-hits are obtained as a flattened list e.g: {-k l_ear cellphone -k r_hand cellphone} -> [l_ear, cellphone, r_hand cellphone]
    let kph = matches.get_many::<String>("keypoint-hits").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
    // here we convert them into pairs -> [(l_ear, cellphone), (r_hand, cellphone)]
    let kph_pairs: Vec<_> = kph.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();

    // See what is being appended
    println!("tier1  {:?}", tier1_name);
    println!("tier2  {:?}", tier2_name);
    println!("keypoint hits {:?}", kph_pairs);

    // generate connection to db 
    let connection =  &mut establish_connection();

    // create tier2 if not exists
    let new_tier2 = NewTier2Activity {tier2: tier2_name};
    diesel::insert_or_ignore_into(tier2activities::table)
        .values(&new_tier2)
        .execute(connection)?; //  unwrap unsafe must pass error


    // create tier1 if not exits
    let new_activity = NewTier1Activity {tier1: tier1_name};
    diesel::insert_or_ignore_into(tier1activities::table)
        .values(&new_activity)
        .execute(connection)?; //  unwrap unsafe must pass error

    for pair in &kph_pairs {
        // create kph if not exits
        let (keypoint_name, object_name) = pair;
            // get kp id (then it must be created if it doesnt exists)
        let new_keypoint = NewKeypoint { name: keypoint_name };

        diesel::insert_or_ignore_into(keypoints::table)
            .values(&new_keypoint)
            .execute(connection)
            .map_err(|err| format!("Error saving keypoint {}", err))?;
    
        let keypoint = keypoints::table
           .filter(keypoints::name.eq(keypoint_name))
           .first::<Keypoint>(connection)?;

            // get object id (then it must be created if not exists)
        let new_object = NewObject { name: object_name };

        diesel::insert_or_ignore_into(objects::table)
            .values(&new_object)
            .execute(connection)
            .map_err(|err| format!("Error saving object {}", err))?;
    
        let object = objects::table
           .filter(objects::name.eq(object_name))
           .first::<Object>(connection)?;

            // add kph to table
        let new_kph = NewKPH {
                object_id: object.object_id,
                keypoint_id: keypoint.keypoint_id,
            };

        diesel::insert_or_ignore_into(keypoint_hits::table)
             .values(&new_kph)
             .execute(connection)?;
    }
    // load database items 
        // get id from tier2 
    let tier2 = tier2activities::table
       .filter(tier2activities::tier2.eq(tier2_name))
       .first::<Tier2Activities>(connection)?;
        // get id from tier1
    let tier1 = tier1activities::table
       .filter(tier1activities::tier1.eq(tier1_name))
       .first::<Tier1Activities>(connection)?;
        // get ids from kph 
    // for only one pair
    let kph: Vec<i32> = kph_pairs
         .iter()
         .flat_map(|(keypoint_name, object_name)| {
             keypoint_hits::table
             .inner_join(keypoints::table)
             .inner_join(objects::table)
             .filter(keypoints::name.eq(keypoint_name)
                .and(objects::name.eq(object_name))) 
             .select(keypoint_hits::kph_id)
             .load::<i32>(connection)
             .unwrap_or_else(|_| Vec::new())
         }).collect();

    // create relationship between items (Create a tier2_tier1_kph object)
    let tier2_tier1_kph = kph
       .iter()
       .map(|&id| NewTier2Tier1Kph { 
           tier2_id: tier2.tier2_id,
           tier1_id: tier1.tier1_id,
           kph_id: id,
       })
       .collect::<Vec<NewTier2Tier1Kph>>();
    
    diesel::insert_or_ignore_into(tier2_tier1_kph::table)
       .values(tier2_tier1_kph)
       .execute(connection)?;
       
    Ok(())
}    