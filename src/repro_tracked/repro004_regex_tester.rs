use clap::{Arg, ArgAction, ArgMatches, Command, command};
use lan_rs_common::util::regex as comm_regex;
use regex::Regex;

fn parse_args() -> ArgMatches {
    command!()
        .arg(Arg::new("regex").required(true))
        .arg(Arg::new("str").required(true))
        .arg(
            Arg::new("verbose")
                .help("1: info, 2: debug, 3: trace")
                .short('v')
                .action(ArgAction::Count),
        )
        .get_matches()
}

pub fn main() {
    let matches = parse_args();

    let regex = {
        matches //_
            .get_one::<String>("regex")
            .unwrap()
            .to_string()
    };

    let str = {
        matches //_
            .get_one::<String>("str")
            .unwrap()
            .to_string()
    };

    let re = Regex::new(&regex).expect("Invalid regex");

    println!("regex: {regex}");
    println!("str: {str}");

    let matches = //_
        comm_regex::regex_capture_once(&str, &re)
            .expect("Could not parse regex");

    println!("matches: {matches:?}");
}
