use clap::{Command, Arg};
use lib::add_entries::{add_room, add_object, add_keypoint, add_poses, add_tier1, add_tier2};

fn main() {

    let app_m = Command::new("cli-db")
         .version(env!("CARGO_PKG_VERSION"))
         .author(env!("CARGO_PKG_AUTHORS"))
         .about("Multi-level menu example")
        .subcommand(Command::new("add_entry")
            .subcommand(Command::new("rooms")
                .arg(Arg::new("room")))
            .subcommand(Command::new("keypoints")
                .arg(Arg::new("keypoint")))
            .subcommand(Command::new("poses")
                .arg(Arg::new("pose")))
            .subcommand(Command::new("objects")
                .arg(Arg::new("object")
                    .short('o')
                    .long("object")
                    .value_name("The object to be added"))
                .arg(Arg::new("rooms")
                    .short('r')
                    .long("rooms")
                    .value_name("A room or rooms where object can be found"))
                )
            .subcommand(Command::new("tier1")
                .arg(Arg::new("object")
                    .short('o')
                    .long("object")
                    .value_name("The local objects related to the tier1 activity"))
                .arg(Arg::new("rooms")
                    .short('r')
                    .long("rooms")
                    .value_name("The room related to the tier1 activity"))
                .arg(Arg::new("pose")
                    .short('p')
                    .long("pose")
                    .value_name("The pose related to the tier1 activity"))
                )
            .subcommand(Command::new("tier2")
                .arg(Arg::new("tier1")
                    .short('t')
                    .long("tier1")
                    .value_name("The tier1 activity related to the tier2 activity"))
                .arg(Arg::new("keypoint-hits")
                    .short('k')
                    .long("kph")
                    .value_name("The keypoint hits related to the tier2 activity"))
                ))
        .subcommand(Command::new("show_entries")
            .arg(Arg::new("table")))
        .get_matches();

    match app_m.subcommand() {
        Some(("add_entry",  sub_m)) => {
            match sub_m.subcommand(){
                Some(("rooms", level3_m)) => {add_room(level3_m)},
                Some(("keypoints", level3_m)) => {add_keypoint(level3_m)},
                Some(("poses", level3_m)) => {add_poses(level3_m)},
                Some(("objects", level3_m)) => {add_object(level3_m)},
                Some(("tier1", level3_m)) => {add_tier1(level3_m)},
                Some(("tier2", level3_m)) => {add_tier2(level3_m)},
                _  => println!("Error matching options")
            }
        },
        Some(("show_entry",   sub_m)) => {println!("push was used")}, 
        _                       => {}, 
    }
}

