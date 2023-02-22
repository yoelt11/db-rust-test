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
#[derive(Debug, Deserialize)]
struct Objects{
    name: String,
}


fn main() {
    let json_string = std::env::args().nth(1).expect("No JSON string provided");

    let topic: Topic = serde_json::from_str(&json_string).expect("Fialed to decode Json string");

    match topic {
        Topic:: GetRooms{message} => {
            for object in message.objects {
                println!("{:?}", object.name);
            }
        }
        Topic::GetTier1{message} => {
            for object in message.objects {
                println!("{:?}", object.name);
            }

            for room in message.rooms {
                println!("{:?}", room.name);
            }

            println!("{:?}", message.pose);
        }
        Topic::GetTier2{message} => {
            println!("{:?}", message.tier1);
            
            for kph in message.kph {
                println!("{:?}", kph);
            }
        }
    }

}

// input example for get Room
//{"topic": "getRooms",
 //"message":{
			//"objects":[
						//{"name":"test"},
						//{"name":"test2"}
					   //]
		  //}
//}'

// input example for getTier1

//{"topic": "getTier1",
// "message":{
//			"objects":[
//						{"name":"test"},
//						{"name":"test2"}
//					   ],
//			"rooms": [
//						{"name":"kitchen"},
//						{"name": "bedroom"}
//					],
//			"pose": "standing"
//		  }
//}

// input example fo "getTier2"
//{"topic": "getTier2",
// "message":{
//			"kph":[
//						["nose","object1"],
//						["r_eye","object2"]
//					   ],
//			"tier1": "eating"
//		  }
//}

