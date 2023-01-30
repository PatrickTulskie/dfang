use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::io;
use std::io::Read;
extern crate atty;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    println!("rfang v{}", VERSION);
    println!("usage: rfang <string>");
}

fn refang(input: &str) -> String {
    let mut result = input.to_string();

    result = DOTS_REGEX.replace_all(&result, ".").to_string();
    result = HXXP_REGEX.replace_all(&result, "http").to_string();
    result = SLASHES_REGEX.replace_all(&result, "://").to_string();
    result = AT_REGEX.replace_all(&result, "@").to_string();
    result = COLONS_REGEX.replace_all(&result, ":").to_string();

    return result;
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
