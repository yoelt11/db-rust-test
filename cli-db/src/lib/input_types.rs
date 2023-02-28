use clap::{ArgMatches};

pub enum RoomInput {
    Cli(ArgMatches),
    Json(Vec<String>)
}

#[derive(Debug)]
pub enum Tier1Input {
    Cli(ArgMatches),
    Json(String, Vec<String>, Vec<String>)
}

#[derive(Debug)]
pub enum Tier2Input {
    Cli(ArgMatches),
    Json(Vec<String>, Vec<(String,String)>)
}

pub enum ActivityInput {
    Json(Vec<String>, Vec<String>, String, Vec<(String, String)>)
}