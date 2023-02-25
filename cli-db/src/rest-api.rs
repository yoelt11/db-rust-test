use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use lib::json_data_structs::*;
use lib::input_types::*;
use lib::get_entries;
use serde_json;


#[get("/")]
async fn readme() -> impl Responder {
    let response = "READ ME".to_string();
    HttpResponse::Ok().body(response)
}

#[post("/get_room")]
async fn get_room(json_string: String) -> impl Responder {
    let parsed_string: RoomMessage = serde_json::from_str(&json_string).expect("Failed to decode Json string");
    let response = get_entries::get_room(RoomInput::Json(parsed_string.objects.iter().map(|obj | obj.name.clone()).collect()));
    HttpResponse::Ok().body(response)
}

#[post("/get_tier1")]
async fn get_tier1(json_string: String) -> impl Responder {
    // parse json to object
    let parsed_string: Tier1Message = serde_json::from_str(&json_string).expect("Fialed to decode Json string");

    // build tier1 input data type 
    let pose = parsed_string.pose;
    let objects = parsed_string.objects.iter().map(|obj | obj.name.clone()).collect();
    let rooms = parsed_string.rooms.iter().map(|room | room.name.clone()).collect();

    let input = Tier1Input::Json(pose, objects, rooms);

    // get response
    let response = get_entries::get_tier1(input);
    HttpResponse::Ok().body(response)
}

#[post("/get_tier2")]
async fn get_tier2(json_string: String) -> impl Responder {

    // parse json to object
    let parsed_string: Tier2Message = serde_json::from_str(&json_string).expect("Fialed to decode Json string");

    // build tier2 input data type 

    let tier1 = parsed_string.tier1;
    let kph = parsed_string.kph;
    
    let input = Tier2Input::Json(tier1, kph);

    // get response
    let response = get_entries::get_tier2(input);
    HttpResponse::Ok().body(response)
}

#[post("/get_activity")]
async fn get_activity(json_string: String) -> impl Responder {
let response = json_string;
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(readme)
            .service(get_room)
            .service(get_tier1)
            .service(get_tier2)
            .service(get_activity)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}