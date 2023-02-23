use lib::input_types::*;
use lib::get_entries::*;
use serde::{Deserialize};
use serde_json;


#[derive(Deserialize)]
#[serde(tag = "topic")]
enum Topic {
    #[serde(rename = "getRooms")]
    GetRooms {message: RoomMessage},
    
    #[serde(rename = "getTier1")]
    GetTier1 {message: Tier1Message},

    #[serde(rename = "getTier2")]
    GetTier2 {message: Tier2Message},
}

// getRooms Message
#[derive(Deserialize)]
struct RoomMessage{
    objects:Vec<Objects>,
}

// getTier1 Message
#[derive(Deserialize)]
struct Tier1Message{
    objects:Vec<Objects>,
    rooms:Vec<Rooms>,
    pose: String,
}

// getTier2 Message
#[derive(Deserialize)]
struct Tier2Message{
    tier1:String,
    kph: Vec<(String, String)>
}

// child components
#[derive(Deserialize)]
struct Rooms{
    name: String,
}
#[derive(Debug, Deserialize, Clone)]
struct Objects{
    name: String,
}


fn main() {
    let json_string = std::env::args().nth(1).expect("No JSON string provided");

    let topic: Topic = serde_json::from_str(&json_string).expect("Fialed to decode Json string");

    match topic {
        Topic:: GetRooms{message} => {
         let room = get_room(RoomInput::Json(message.objects.iter().map(|obj | obj.name.clone()).collect()));
         println!("{:?}", room);
        }
        Topic::GetTier1{message} => {
            let pose = message.pose;
            let objects = message.objects.iter().map(|obj | obj.name.clone()).collect();
            let rooms = message.rooms.iter().map(|room | room.name.clone()).collect();
            
            let input = Tier1Input::Json(pose, objects, rooms);
            
            let tier1 = get_tier1(input);
            
            println!("{:?}", tier1);
        }
        Topic::GetTier2{message} => {
            let tier1 = message.tier1;
            let kph = message.kph;
            
            let input = Tier2Input::Json(tier1, kph);

            let tier2 = get_tier2(input);
            
            println!("{:?}", tier2);
        }
    }

}
