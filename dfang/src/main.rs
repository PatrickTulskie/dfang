use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::io::{self, Read, IsTerminal};

const VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    // Replacers
    static ref DOTS_REGEX: Regex = Regex::new(r"\.").unwrap();
    static ref COLONS_REGEX: Regex = Regex::new(r":").unwrap();
    static ref AT_REGEX: Regex = Regex::new(r"@").unwrap();
    static ref HTTP_REGEX: Regex = Regex::new(r"(?i)http").unwrap();
    static ref SLASHES_REGEX: Regex = Regex::new(r"://").unwrap();

    // Matchers
    static ref EMAIL_REGEX: Regex = Regex::new(r"(?i)([A-Za-z0-9!#$%&'*+/=?^_{|.}~-]+@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?)").unwrap();
    static ref IPV4_REGEX: Regex = Regex::new(r"(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)").unwrap();
    static ref IPV6_REGEX: Regex = Regex::new(r"(?:(?:(?:[0-9A-Fa-f]{1,4}:){7}(?:[0-9A-Fa-f]{1,4}|:))|(?:(?:[0-9A-Fa-f]{1,4}:){6}(?::[0-9A-Fa-f]{1,4}|(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(?:(?:[0-9A-Fa-f]{1,4}:){5}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,2})|:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(?:(?:[0-9A-Fa-f]{1,4}:){4}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,3})|(?:(?::[0-9A-Fa-f]{1,4})?:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(?:(?:[0-9A-Fa-f]{1,4}:){3}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,4})|(?:(?::[0-9A-Fa-f]{1,4}){0,2}:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(?:(?:[0-9A-Fa-f]{1,4}:){2}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,5})|(?:(?::[0-9A-Fa-f]{1,4}){0,3}:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(?:(?:[0-9A-Fa-f]{1,4}:){1}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,6})|(?:(?::[0-9A-Fa-f]{1,4}){0,4}:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(?::(?:(?:(?::[0-9A-Fa-f]{1,4}){1,7})|(?:(?::[0-9A-Fa-f]{1,4}){0,5}:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:)))(?:%.+)?\s*").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let mut input = String::new();
        if !io::stdin().is_terminal() {
            // read input from pipe
            io::stdin().read_to_string(&mut input).unwrap();
            for line in input.lines() {
                println!("{}", defang(line));
            }
        } else {
            help();
        }
    } else {
        for i in 1..args.len() {
            println!("{}", defang(&args[i]));
        }
    }
}

fn help() {
    println!("dfang v{}", VERSION);
    println!("usage: dfang <string>");
}

fn defang(input: &str) -> String {
    if IPV4_REGEX.is_match(input) {
        return defang_ipv4(input);
    } else if IPV6_REGEX.is_match(input) {
        return defang_ipv6(input);
    } else if EMAIL_REGEX.is_match(input) {
        return defang_email(input)
    } else {
        return defang_url(input);
    }
}

fn defang_url(input: &str) -> String {
    let mut result = input.to_string();

    result = DOTS_REGEX.replace_all(&result, "[.]").to_string();
    result = HTTP_REGEX.replace_all(&result, "hxxp").to_string();
    result = SLASHES_REGEX.replace_all(&result, "[://]").to_string();

    return result;
}

fn defang_ipv4(input: &str) -> String {
    return DOTS_REGEX.replace_all(input, "[.]").to_string();
}

fn defang_ipv6(input: &str) -> String {
    return COLONS_REGEX.replace_all(input, "[:]").to_string();
}

fn defang_email(input: &str) -> String {
    let mut result = input.to_string();

    result = DOTS_REGEX.replace_all(&result, "[.]").to_string();
    result = AT_REGEX.replace_all(&result, "[@]").to_string();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defang() {
        assert_eq!(defang("http://example.com"), "hxxp[://]example[.]com");
        assert_eq!(defang("https://example.com"), "hxxps[://]example[.]com");
        assert_eq!(defang("example@example.com"), "example[@]example[.]com");
        assert_eq!(defang("2001:0db8:85a3:0000:0000:8a2e:0370:7334"), "2001[:]0db8[:]85a3[:]0000[:]0000[:]8a2e[:]0370[:]7334");
        assert_eq!(defang("192.168.1.1"), "192[.]168[.]1[.]1")
    }
}
