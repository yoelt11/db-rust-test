use serde::{Deserialize};

#[derive(Deserialize)]
#[serde(tag = "topic")]
pub enum Topic {
    #[serde(rename = "getRooms")]
    GetRooms {message: RoomMessage},
    
    #[serde(rename = "getTier1")]
    GetTier1 {message: Tier1Message},

    #[serde(rename = "getTier2")]
    GetTier2 {message: Tier2Message},
    
    #[serde(rename = "getActivity")]
    GetActivity {message: ActivityMessage},
}

// getRooms Message
#[derive(Deserialize)]
pub struct RoomMessage{
    pub objects:Vec<Objects>,
}

// getTier1 Message
#[derive(Deserialize)]
pub struct Tier1Message{
    pub objects:Vec<Objects>,
    pub rooms:Vec<Rooms>,
    pub pose: String,
}

// getTier2 Message
#[derive(Deserialize)]
pub struct Tier2Message{
    pub tier1: Vec<String>,
    pub kph: Vec<(String, String)>
}

// getTier2 Message
#[derive(Deserialize)]
pub struct ActivityMessage{
    pub global_ctx: Vec<Objects>,
    pub local_ctx: Vec<Objects>,
    pub pose_class: String,
    pub kph: Vec<(String, String)>
}

// child components
#[derive(Deserialize)]
pub struct Rooms{
    pub name: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Objects{
    pub name: String,
}