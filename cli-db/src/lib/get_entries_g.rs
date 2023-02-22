use std::fmt::Debug;

pub trait RoomInput {
    fn getRoom(&self);
}

impl RoomInput for JsonGetRoomInput {
    fn getRoom(&self) {
        println!("{:?}", self.input);
    }
}
impl RoomInput for CliGetRoomInput {
    fn getRoom(&self) {
        println!("{}", self.input);
    }
}

#[derive(Debug)]
pub struct JsonGetRoomInput {
    pub input: Vec<String>,
}

#[derive(Debug)]
pub struct CliGetRoomInput {
    pub input: String,   
}
