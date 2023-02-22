use lib::get_entries_g::*;
fn main(){
   let input = CliGetRoomInput { input: "some input".to_string() };
   input.getRoom();
   

   let input = JsonGetRoomInput { input: vec!["one".to_string(), "two".to_string(), "three".to_string()] };
   input.getRoom();
}

