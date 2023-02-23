use clap::ArgMatches;

pub enum RoomInput {
    Cli(ArgMatches),
    Json(Vec<String>)
}

pub fn get_room(input: RoomInput) {
    let objects = match input {
        RoomInput::Cli(arg_matches) => {
            arg_matches.get_many::<String>("objects")
                .unwrap_or_default()
                .map(|v| v.to_owned())
                .collect()
        }
        RoomInput::Json(objects) => objects,
    };

    println!("{:?}", objects);
}

pub fn test() {
    let objects = vec!["item".to_string(),"item".to_string()];
    let input = RoomInput::Json(objects);

    get_room(input);
}