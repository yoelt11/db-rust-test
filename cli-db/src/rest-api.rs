use actix_web::body::MessageBody;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};
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
    let response = get_entries::get_room(RoomInput::Json(parsed_string.objects
                                                                        .iter()
                                                                        .map(|obj | obj.name.clone())
                                                                        .collect()))
                                                                        .unwrap_or(vec!["None".to_string()]);
    HttpResponse::Ok().body(ResponseWrapper(response))
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
    let response = get_entries::get_tier1(input).unwrap_or(vec!["None".to_string()]);
    HttpResponse::Ok().body(ResponseWrapper(response))
}

#[post("/get_tier2")]
async fn get_tier2(json_string: String) -> impl Responder {

    // parse json to object
    let parsed_string: Tier2Message = serde_json::from_str(&json_string).expect("Failed to decode Json string");

    // build tier2 input data type 

    let pose = parsed_string.pose;
    let tier1 = parsed_string.tier1;
    let kph = parsed_string.kph;
    
    let input = Tier2Input::Json(pose, tier1, kph);

    // get response
    let response = get_entries::get_tier2(input)
                                            .unwrap_or(vec!["None".to_string()]);
    HttpResponse::Ok().body(ResponseWrapper(response))
}

#[post("/get_activity")]
async fn get_activity(json_string: String) -> impl Responder {
    
    let message: ActivityMessage = serde_json::from_str(&json_string).expect("Failed to decode Json string");

    // get items from json and convert to string
    let pose_class = message.pose_class;
    let global_ctx = message.global_ctx.iter().map(|obj | obj.name.clone()).collect();
    let local_ctx = message.local_ctx.iter().map(|obj | obj.name.clone()).collect();
    let kph = message.kph;
    // build input
    let input = ActivityInput::Json(global_ctx, local_ctx, pose_class, kph);
    // get activity
    let response = get_entries::get_activity(input);

    HttpResponse::Ok().body(ResponseWrapper(response))
}

struct ResponseWrapper(Vec<String>);

impl MessageBody for ResponseWrapper {
    type Error = actix_web::Error;

    fn size(&self) -> actix_web::body::BodySize {
        actix_web::body::BodySize::Sized(self.0.iter().fold(0, |acc, s| acc + s.len() as u64))
    }

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<actix_web::web::Bytes, Self::Error>>> {
        let next = self.0.pop().map(|s| Ok(web::Bytes::from(s)));
        match next {
            Some(res) => std::task::Poll::Ready(Some(res)),
            None => std::task::Poll::Ready(None),
        }
    }
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