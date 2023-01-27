use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::io;
use std::io::Read;
extern crate atty;

lazy_static! {
    // Replacers
    static ref DOTS_REGEX: Regex = Regex::new(r"\[\.\]").unwrap();
    static ref COLONS_REGEX: Regex = Regex::new(r"\[:\]").unwrap();
    static ref AT_REGEX: Regex = Regex::new(r"\[@\]").unwrap();
    static ref HXXP_REGEX: Regex = Regex::new(r"(?i)hxxp").unwrap();
    static ref SLASHES_REGEX: Regex = Regex::new(r"\[://\]").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let mut input = String::new();
        if !atty::is(atty::Stream::Stdin) {
            // read input from pipe
            io::stdin().read_to_string(&mut input).unwrap();
            for line in input.lines() {
                println!("{}", refang(line));
            }
        } else {
            help();
        }
    } else {
        for i in 1..args.len() {
            println!("{}", refang(&args[i]));
        }
    }
}

fn help() {
    println!("usage: dfang <string>")
}

fn refang(input: &str) -> String {
    let fanged_dots = DOTS_REGEX.replace_all(input, ".");
    let fanged_http = HXXP_REGEX.replace_all(&fanged_dots, "http");
    let fanged_slashes = SLASHES_REGEX.replace_all(&fanged_http, "://");
    let fanged_ats = AT_REGEX.replace_all(&fanged_slashes, "@");
    let fanged_colons = COLONS_REGEX.replace_all(&fanged_ats, ":");

    return fanged_colons.to_string();
}

