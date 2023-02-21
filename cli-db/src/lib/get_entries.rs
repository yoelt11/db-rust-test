use crate::{establish_connection};
use clap::{ArgMatches};
use diesel::prelude::*;

pub fn get_room(matches:&ArgMatches) -> String {
    use crate::schema::room_object;
    use crate::schema::objects;
    use crate::schema::rooms;

    // get variables from cli
    let object_names = matches.get_many::<String>("objects").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
    println!("querying by using object: {:?}", object_names);   
    
    // perform query
    // generate connection to db 
    let connection =  &mut establish_connection();

    // perform query
    let query = room_object::table
        .inner_join(rooms::table)
        .inner_join(objects::table)
        .filter(objects::name.eq_any(object_names))
        .select(rooms::name)
        .load::<String>(connection)
        .unwrap();

    let most_common = query.iter()
        .max_by_key(|&element| query.iter().filter(|&e| e == element).count())
        .unwrap(); // if tie vote for default room

    println!("returned by query: {:?}", query);
    most_common.to_string()
}

pub fn get_tier1(matches:&ArgMatches) -> String{
    use crate::schema::tier1_activity_objects;
    use crate::schema::tier1_activity_poses;
    use crate::schema::tier1_activity_rooms;
    use crate::schema::objects;
    use crate::schema::rooms;
    use crate::schema::poses;
    use crate::schema::tier1activities;

    // get variables from cli
    let pose_name = matches.get_one::<String>("pose").unwrap();   
    let room_names = matches.get_many::<String>("rooms").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
    let object_names = matches.get_many::<String>("objects").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();

    // generate connection to db 
    let connection =  &mut establish_connection();

    // perform query
    let tier1_pose_query = tier1_activity_poses::table
        .inner_join(tier1activities::table)
        .inner_join(poses::table)
        .filter(poses::name.eq(pose_name))
        .select(tier1activities::tier1)
        .load::<String>(connection)
        .unwrap();
    
    let tier1_object_query = tier1_activity_objects::table
        .inner_join(tier1activities::table)
        .inner_join(objects::table)
        .filter(objects::name.eq_any(object_names))
        .select(tier1activities::tier1)
        .load::<String>(connection)
        .unwrap();
    
    let tier1_room_query = tier1_activity_rooms::table
        .inner_join(tier1activities::table)
        .inner_join(rooms::table)
        .filter(rooms::name.eq_any(room_names))
        .select(tier1activities::tier1)
        .load::<String>(connection)
        .unwrap();

    println!("rooms: {:?}", tier1_room_query);
    println!("object: {:?}", tier1_object_query);
    println!("pose: {:?}", tier1_pose_query);

    let mut merged_query = vec![];
    merged_query.extend(tier1_object_query);
    merged_query.extend(tier1_pose_query);
    merged_query.extend(tier1_room_query);

    let most_common = merged_query.iter()
        .max_by_key(|&element| merged_query.iter().filter(|&e| e == element).count())
        .unwrap(); // if tie vote for default room

    most_common.to_string()
}
pub fn get_tier2(matches:&ArgMatches){
    use crate::schema::tier1activities; 
    use crate::schema::tier2activities;
    use crate::schema::objects;
    use crate::schema::keypoints;
    use crate::schema::tier2_tier1_kph;
    use crate::schema::keypoint_hits;

    // Get arguments from cli 
    let tier1_name = matches.get_one::<String>("tier1").unwrap();   
    // Kp-hits are obtained as a flattened list e.g: {-k l_ear cellphone -k r_hand cellphone} -> [l_ear, cellphone, r_hand cellphone]
    let kph = matches.get_many::<String>("keypoint-hits").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
    // here we convert them into pairs -> [(l_ear, cellphone), (r_hand, cellphone)]
    let kph_pairs: Vec<_> = kph.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();

    // See what is being appended
    println!("tier1  {:?}", tier1_name);
    println!("keypoint hits {:?}", kph_pairs);
    
    // generate connection to db 
    let connection =  &mut establish_connection();

    // perform query
    // query from kph to get kph_id according to object_id and keypoint_id
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
    
    let tier2_tier1_query = tier2_tier1_kph::table
        .inner_join(tier2activities::table)
        .inner_join(tier1activities::table)
        .inner_join(keypoint_hits::table)
        .filter(tier1activities::tier1.eq(tier1_name)
                .and(keypoint_hits::kph_id.eq_any(kph)))
        .select(tier2activities::tier2)
        .load::<String>(connection)
        .unwrap();

    println!("{:?}", tier2_tier1_query);
}