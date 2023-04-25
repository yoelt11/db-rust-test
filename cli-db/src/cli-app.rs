use clap::{Command, Arg, ArgAction };
use lib::add_entries::{add_room, add_object, add_keypoint, add_poses, add_tier1, add_tier2};
use lib::get_entries::{get_room, get_tier1, get_tier2};
use lib::input_types::*;

fn main() {

    let mut app_m = Command::new("cli-db")
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
                .arg(Arg::new("object"))
                .arg(Arg::new("rooms")
                    .num_args(0..)
                    .short('r')
                    .long("rooms")
                    .value_name("A room or rooms where object can be found"))
                )
            .subcommand(Command::new("tier1")
                .arg(Arg::new("tier1"))
                .arg(Arg::new("object")
                    .num_args(0..)
                    .short('o')
                    .long("object")
                    .value_name("The local objects related to the tier1 activity"))
                .arg(Arg::new("rooms")
                    .num_args(0..)
                    .short('r')
                    .long("rooms")
                    .value_name("The room related to the tier1 activity"))
                .arg(Arg::new("poses")
                    .num_args(0..)
                    .short('p')
                    .long("pose")
                    .value_name("The pose related to the tier1 activity"))
                )
            .subcommand(Command::new("tier2")
                .arg(Arg::new("tier2"))
                .arg(Arg::new("tier1")
                    .short('t')
                    .long("tier1")
                    .value_name("The tier1 activity related to the tier2 activity"))
                .arg(Arg::new("poses")
                    .num_args(0..)
                    .short('p')
                    .long("pose")
                    .value_name("The pose related to the tier1 activity"))
                .arg(Arg::new("keypoint-hits")
                    .action(ArgAction::Append)
                    .num_args(2)
                    .short('k')
                    .long("kph")
                    .value_name("The keypoint hits related to the tier2 activity"))
                ))
        .subcommand(Command::new("show_entries")
            .arg(Arg::new("table")))
        .subcommand(Command::new("get_entries")
            .subcommand(Command::new("room")
                .arg(Arg::new("objects")
                    .short('o')
                    .num_args(0..)
                    .long("objects") 
                    .value_name("A list of objects")
                ))
            .subcommand(Command::new("tier1")
                .arg(Arg::new("objects")
                    .num_args(0..)
                    .short('o')
                    .long("objects")
                    .value_name("The local objects related to the tier1 activity"))
                .arg(Arg::new("rooms")
                    .num_args(0..)
                    .short('r')
                    .long("rooms")
                    .value_name("The room related to the tier1 activity"))
                .arg(Arg::new("pose")
                    .num_args(0..)
                    .short('p')
                    .long("pose")
                    .value_name("The pose related to the tier1 activity"))
        )
            .subcommand(Command::new("tier2")
                .arg(Arg::new("tier1")
                    .short('t')
                    .long("tier1")
                    .value_name("The tier1 activity related to the tier2 activity"))
                .arg(Arg::new("pose")
                    .num_args(0..)
                    .short('p')
                    .long("pose")
                    .value_name("The pose related to the tier1 activity"))
                .arg(Arg::new("keypoint-hits")
                    .action(ArgAction::Append)
                    .num_args(2)
                    .short('k')
                    .long("kph")
                    .value_name("The keypoint hits related to the tier2 activity. e.g -k '<keypoint 1> <object 1> -k <keypoint n> <object n>' " ))
            )
        );

    match app_m.clone().get_matches().subcommand() {
        Some(("add_entry",  sub_m)) => {
            match sub_m.subcommand(){
                Some(("rooms", level3_m)) => {println!("{:?}",add_room(level3_m));},
                Some(("keypoints", level3_m)) => {println!("{:?}", add_keypoint(level3_m));},
                Some(("poses", level3_m)) => {println!("{:?}", add_poses(level3_m));},
                Some(("objects", level3_m)) => {println!("{:?}", add_object(level3_m));},
                Some(("tier1", level3_m)) => {println!(" {:?}", add_tier1(level3_m));},
                Some(("tier2", level3_m)) => {println!(" {:?}", add_tier2(level3_m));},
                _ => {
                    let subcmd = app_m.find_subcommand_mut("add_entry").unwrap();
                    subcmd.print_help().unwrap();
                }
            }
        },
        Some(("show_entry",  _sub_m)) => {println!("push was used")}, 
        Some(("get_entries",  sub_m)) => {
            match sub_m.subcommand(){
                Some(("room", level3_m)) => {
                    let room = get_room(RoomInput::Cli(level3_m.clone()));
                    println!("voted room: {:?}", room);
                },
                Some(("tier1", level3_m)) => {
                    let tier1 = get_tier1(Tier1Input::Cli(level3_m.clone()));
                    println!("voted activity: {:?}", tier1);
                },
                Some(("tier2", level3_m)) => {
                    let tier2 = get_tier2(Tier2Input::Cli(level3_m.clone()));
                    println!("voted activity: {:?}", tier2);
                },
                _  => {
                    let subcmd = app_m.find_subcommand_mut("add_entry").unwrap();
                    subcmd.print_help().unwrap();
                } 
            }
        },
       _ => {app_m.clone().print_help().unwrap()} 
    }
}

