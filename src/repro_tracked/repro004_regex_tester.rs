use clap::{Arg, ArgAction, ArgMatches, Command, command};
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegexCaptureOnceError {
    #[error("No match for regex {1:?} and string {0:?}")]
    NoMatch(String, String),

    #[error("Failed to capture for regex {1:?} and string {0:?}")]
    CaptureError(String, String),

    #[error("Failed to collect capture groups for regex {1:?} and string {0:?}")]
    CaptureCollectError(String, String),
}

pub fn regex_capture_once(s: &str, re: &Regex) -> Result<Vec<String>, RegexCaptureOnceError> {
    type FnErr = RegexCaptureOnceError;

    if !re.is_match(s) {
        return Err(FnErr::NoMatch(s.to_owned(), re.as_str().to_owned()));
    }

    let matches = re
        .captures(&s)
        .ok_or(FnErr::CaptureError(
            s.to_owned(),
            re.as_str().to_owned(),
        ))?
        .iter().collect::<Option<Vec<_>>>()
        .ok_or(FnErr::CaptureCollectError(
            s.to_owned(),
            re.as_str().to_owned(),
        ))?;

    Ok(matches
        .into_iter()
        .map(|m| m.as_str().to_owned())
        .collect::<Vec<_>>())
}

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

    let regex = matches.get_one::<String>("regex").unwrap().to_string();

    let str = matches.get_one::<String>("str").unwrap().to_string();

    let re = Regex::new(&regex).expect("Invalid regex");

    let matches = regex_capture_once(&str, &re).expect("Could not parse regex");

    println!("matches: {matches:?}");
}
