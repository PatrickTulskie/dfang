use lazy_static::lazy_static;
use regex::Regex;
use std::env;

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
    let first_email = &args[1];
    let defangged = defangify(first_email);
    println!("{}", defangged);
}

fn defangify(input: &str) -> String {
    if IPV4_REGEX.is_match(input) {
        return defang_ipv4(input);
    } else if IPV6_REGEX.is_match(input) {
        return defang_ipv6(input);
    } else if EMAIL_REGEX.is_match(input) {
        return defangify_email(input)
    } else {
        return defangify_url(input);
    }
}

fn defangify_url(input: &str) -> String {
    let no_dots = DOTS_REGEX.replace_all(input, "[.]");
    let no_http = HTTP_REGEX.replace_all(&no_dots, "hxxp");
    let no_slashes = SLASHES_REGEX.replace_all(&no_http, "[://]");

    return no_slashes.to_string();
}

fn defang_ipv4(input: &str) -> String {
    let no_dots = DOTS_REGEX.replace_all(input, "[.]");

    return no_dots.to_string();
}

fn defang_ipv6(input: &str) -> String {
    let no_colons = COLONS_REGEX.replace_all(input, "[:]");

    return no_colons.to_string();
}

fn defangify_email(input: &str) -> String {
    let no_dots = DOTS_REGEX.replace_all(input, "[.]");
    let no_at = AT_REGEX.replace_all(&no_dots, "[@]");

    return no_at.to_string();
}
