use clap::{Command, Arg};

fn main() {
    let cmd = Command::new("cli-db")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(Arg::new("add_entries")
                .short('a')
                .long("add_entries")
                .value_name("Entry"))
        .arg(Arg::new("show_entries")
                .short('l')
                .long("show_entries")
                .value_name("Entry"))
        .get_matches();

    println!("{:?}", cmd);
}

