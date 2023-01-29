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
    println!("usage: rfang <string>")
}

fn refang(input: &str) -> String {
    let fanged_dots = DOTS_REGEX.replace_all(input, ".");
    let fanged_http = HXXP_REGEX.replace_all(&fanged_dots, "http");
    let fanged_slashes = SLASHES_REGEX.replace_all(&fanged_http, "://");
    let fanged_ats = AT_REGEX.replace_all(&fanged_slashes, "@");
    let fanged_colons = COLONS_REGEX.replace_all(&fanged_ats, ":");

    return fanged_colons.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refang() {
        assert_eq!(refang("hxxp[://]example[.]com"), "http://example.com");
        assert_eq!(refang("hxxps[://]example[.]com"), "https://example.com");
        assert_eq!(refang("example[@]example[.]com"), "example@example.com");
        assert_eq!(refang("2001[:]0db8[:]85a3[:]0000[:]0000[:]8a2e[:]0370[:]7334"), "2001:0db8:85a3:0000:0000:8a2e:0370:7334");
        assert_eq!(refang("192[.]168[.]1[.]1"), "192.168.1.1")
    }
}
