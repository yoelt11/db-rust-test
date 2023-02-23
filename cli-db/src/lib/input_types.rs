use serde::{Deserialize};
use serde_json;

use clap::{ArgMatches, Arg};

pub enum RoomInput {
    Cli(ArgMatches),
    Json(Vec<String>)
}

pub enum Tier1Input {
    Cli(ArgMatches),
    Json(String, Vec<String>, Vec<String>)
}
pub enum Tier2Input {
    Cli(ArgMatches),
    Json(String, Vec<(String,String)>)
}