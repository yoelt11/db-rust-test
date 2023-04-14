use crate::{establish_connection, input_types::{ActivityInput, RoomInput, Tier1Input, Tier2Input}};
use diesel::prelude::*;
use std::collections::HashMap;
use std::error::Error;

/// Infers a room based on a vector of objects.
/// 
/// # Example
/// 
/// Usage:
/// ```rust
///                // for cli-app. Where level3_m is &ArgMatches 
///                Some(("room", level3_m)) => {
///                   let room = get_room(RoomInput::Cli(level3_m.clone()));
///               } 
///               // for json-app.
///                  let room: Vec<String> = get_room(RoomInput::Json(vec!["apple","orange"]));
/// ```
pub fn get_room(input: RoomInput) -> Result<Vec<String>, Box<dyn Error>> {
    use crate::schema::room_object;
    use crate::schema::objects;
    use crate::schema::rooms;

    // get variables from cli or Json
    let object_names = match input {
        RoomInput::Cli(arg_matches) => {
            arg_matches.get_many::<String>("objects")
                .unwrap_or_default()
                .map(|v| v.to_owned())
                .collect()
        }
        RoomInput::Json(objects) => objects,
    };

    // generate connection to db 
    let connection =  &mut establish_connection();

    // perform query
    let query: Vec<String> = room_object::table
        .inner_join(rooms::table)
        .inner_join(objects::table)
        .filter(objects::name.eq_any(object_names))
        .select(rooms::name)
        .load::<String>(connection)?;

    let most_common = n_max(3, &query);

    most_common 
}

pub fn get_tier1(input: Tier1Input) -> Result<Vec<String>, Box<dyn Error>>{
    use crate::schema::tier1_activity_objects;
    use crate::schema::tier1_activity_poses;
    use crate::schema::tier1_activity_rooms;
    use crate::schema::objects;
    use crate::schema::rooms;
    use crate::schema::poses;
    use crate::schema::tier1activities;

    // get variables from cli or Json
    let (pose_name, room_names, object_names) = match input {
        Tier1Input::Cli(matches) => {
            // get variables from cli
            let pose_name = matches.get_one::<String>("pose").unwrap().clone();   
            let room_names = matches.get_many::<String>("rooms").unwrap_or_default().map(|v| v.to_owned()).collect::<Vec<_>>();
            let object_names = matches.get_many::<String>("objects").unwrap_or_default().map(|v| v.to_owned()).collect::<Vec<_>>();
            (pose_name, room_names, object_names)
        }
        Tier1Input::Json(pose, objects, rooms) => {
            (pose, rooms, objects)
        },
    };

    // generate connection to db 
    let connection =  &mut establish_connection();

    // perform query
    let tier1_pose_query: Vec<String> = tier1_activity_poses::table
        .inner_join(tier1activities::table)
        .inner_join(poses::table)
        .filter(poses::name.eq(pose_name))
        .select(tier1activities::tier1)
        .load::<String>(connection)?;
    
    let tier1_object_query: Vec<String> = tier1_activity_objects::table
        .inner_join(tier1activities::table)
        .inner_join(objects::table)
        .filter(objects::name.eq_any(object_names))
        .select(tier1activities::tier1)
        .load::<String>(connection)?;
    
    let tier1_room_query: Vec<String> = tier1_activity_rooms::table
        .inner_join(tier1activities::table)
        .inner_join(rooms::table)
        .filter(rooms::name.eq_any(room_names))
        .select(tier1activities::tier1)
        .load::<String>(connection)?;

    let mut merged_query = vec![];

    if tier1_object_query.is_empty() {
        let most_common = n_max(3, &merged_query);
        return most_common
    }

    merged_query.extend(tier1_object_query);
    merged_query.extend(tier1_pose_query);
    merged_query.extend(tier1_room_query);

    let most_common = n_max(3, &merged_query);
    
    most_common 
}

pub fn get_tier2(input: Tier2Input) -> Result<Vec<String>, Box<dyn Error>> {
    use crate::schema::tier1activities; 
    use crate::schema::tier2activities;
    use crate::schema::objects;
    use crate::schema::keypoints;
    use crate::schema::tier2_tier1_kph;
    use crate::schema::keypoint_hits;

    // get inputs depending on case CLI app or Json app
    let (tier1_name, kph_pairs) = match input {
        Tier2Input::Cli(matches) => {
            // Get arguments from cli 
            let tier1_name = matches.get_many::<String>("tier1").unwrap_or_default().map(|v| v.to_string()).collect::<Vec<_>>();
            // Kp-hits are obtained as a flattened list e.g: {-k l_ear cellphone -k r_hand cellphone} -> [l_ear, cellphone, r_hand cellphone]
            let kph = matches.get_many::<String>("keypoint-hits").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
            // here we convert them into pairs -> [(l_ear, cellphone), (r_hand, cellphone)]
            let kph_pairs: Vec<_> = kph.chunks(2).map(|chunk| (chunk[0].to_owned(), chunk[1].to_owned())).collect();
            (tier1_name, kph_pairs)
            }
        Tier2Input::Json(tier1, kph) => {
            (tier1, kph)
        }
    };
    
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
        .filter(tier1activities::tier1.eq_any(tier1_name)
                .and(keypoint_hits::kph_id.eq_any(kph)))
        .select(tier2activities::tier2)
        .load::<String>(connection)?;

    let most_common = n_max(3, &tier2_tier1_query);

    most_common
}

pub fn get_activity(input: ActivityInput) -> Vec<String> {
    let (room_input, local_ctx, pose_class, kph) = match input{
        //TODO: Implement Cli case
        ActivityInput::Json(global_ctx, local_ctx
            , pose_class, kph ) => {
                (RoomInput::Json(global_ctx), local_ctx, pose_class, kph)
            }
    };
    
    let result = match get_room(room_input) {
        Ok(rooms) => {
            if rooms.is_empty() {
                vec![pose_class.clone()]
            } else {
                match get_tier1(Tier1Input::Json(pose_class.clone(), local_ctx, rooms)) {
                    Ok(tier1activity) => {
                        if tier1activity.is_empty() {
                            vec![pose_class]
                        } else {
                            match get_tier2(Tier2Input::Json(tier1activity.clone(), kph)) {
                                Ok(tier2activity) => {
                                    if tier2activity.is_empty() {
                                        tier1activity
                                    } else {
                                        let v = format!("[{}] - {}",tier1activity[0], tier2activity[0]);
                                        vec![v]
                                    }
                                },
                                Err(e) => tier1activity
                            }
                        }
                    },
                    Err(e) => vec![pose_class]
                }
            }
        },
        Err(e) => vec![pose_class]
    };

    result
}

fn n_max<T>(n: usize, it: &Vec<T> ) -> Result<Vec<T>, Box<dyn Error>>
    where 
        T: std::fmt::Debug + std::hash::Hash + std::cmp::Eq + Clone  {
            
    // create a new has map
    let mut count_map = HashMap::new();
    
    // count the number of ocurrences of each item
    for item in it {
        let count = count_map.entry(item.clone()).or_insert(0);
        *count +=1;
    }

    // Sort items based on their frequency count
    let mut sorted_items: Vec<_> = count_map.into_iter().collect();
    sorted_items.sort_by(|a, b| b.1.cmp(&a.1));
    
    // if only one max value is found return otherwise return all ties 
    let top_occ: Vec<_> = sorted_items.iter()
                                      .filter(|(_, occr)| *occr == sorted_items[0].1)
                                      .map(|(key, _)| key.clone()).collect();

    // return only the string
    if top_occ.len() <= n {
        Ok(top_occ)
    } else {
        Ok(top_occ[..n].to_vec())
    }
}