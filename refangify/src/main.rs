use lazy_static::lazy_static;
use regex::Regex;
use std::env;

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
    let refangged = refangify(&args[1]);
    println!("{}", refangged);
}

fn refangify(input: &str) -> String {
    let fanged_dots = DOTS_REGEX.replace_all(input, ".");
    let fanged_http = HXXP_REGEX.replace_all(&fanged_dots, "http");
    let fanged_slashes = SLASHES_REGEX.replace_all(&fanged_http, "://");
    let fanged_ats = AT_REGEX.replace_all(&fanged_slashes, "@");
    let fanged_colons = COLONS_REGEX.replace_all(&fanged_ats, ":");

    return fanged_colons.to_string();
}

