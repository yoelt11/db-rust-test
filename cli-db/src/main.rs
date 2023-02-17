use clap::{Command, Arg};

fn main() {
    let cmd = Command::new("cli-db")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
            .about("Multi-level menu example")
        .arg(Arg::new("COMMAND"))
        .arg(Arg::new("options")
                .short('o')
                .long("options")
                .value_name("rooms, keypoints, poses, objects, tier1, tier2"))
        .get_matches();

        println!("{:?}", cmd.get_one::<String>("COMMAND"));
        println!("{:?}", cmd.get_one::<String>("options"));
}

