use lib::input_types::*;
use lib::get_entries::*;
use lib::json_data_structs::*;
use serde_json;

fn main() {
    let json_string = std::env::args().nth(1).expect("No JSON string provided");

    let topic: Topic = serde_json::from_str(&json_string).expect("Failed to decode Json string");

    match topic {
        Topic:: GetRooms{message} => {
         let room = get_room(RoomInput::Json(message.objects
                                                    .iter()
                                                    .map(|obj | obj.name.clone())
                                                    .collect())).unwrap_or(vec!["None".to_string()]);
         println!("{:?}", room);
        }
        Topic::GetTier1{message} => {
            let pose = message.pose;
            let objects = message.objects.iter().map(|obj | obj.name.clone()).collect();
            let rooms = message.rooms.iter().map(|room | room.name.clone()).collect();
            
            let input = Tier1Input::Json(pose, objects, rooms);
            
            let tier1 = get_tier1(input).unwrap_or(vec!["None".to_string()]);
            
            println!("{:?}", tier1);
        }
        Topic::GetTier2{message} => {
            let pose = message.pose;
            let tier1 = message.tier1;
            let kph = message.kph;
            
            let input = Tier2Input::Json(pose, tier1, kph);

            let tier2 = get_tier2(input).unwrap_or(vec!["None".to_string()]);
            
            println!("{:?}", tier2);
        }
        Topic::GetActivity{message} => {
            // get items from json and convert to string
            let pose_class = message.pose_class;
            let global_ctx = message.global_ctx.iter().map(|obj | obj.name.clone()).collect();
            let local_ctx = message.local_ctx.iter().map(|obj | obj.name.clone()).collect();
            let kph = message.kph;
            // build input
            let input = ActivityInput::Json(global_ctx, local_ctx, pose_class, kph);
            // get activity
            let inferred_activity = get_activity(input);
            println!("{:?}", inferred_activity);
        }
    }

}
