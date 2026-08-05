use std::env;
use std::io::{self, Read, IsTerminal};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let mut input = String::new();
        if !io::stdin().is_terminal() {
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
    let result = input.replace("[.]", ".");
    let result = replace_ignore_ascii_case(&result, "hxxp", "http");
    let result = result.replace("[://]", "://");
    let result = result.replace("[@]", "@");

    return result.replace("[:]", ":");
}

/// Non-overlapping, leftmost replacement of an ASCII `needle`, ignoring case.
fn replace_ignore_ascii_case(haystack: &str, needle: &str, to: &str) -> String {
    let (bytes, pat) = (haystack.as_bytes(), needle.as_bytes());
    let mut out = String::with_capacity(haystack.len());
    let mut copied = 0;
    let mut i = 0;

    while i + pat.len() <= bytes.len() {
        if bytes[i..i + pat.len()].eq_ignore_ascii_case(pat) {
            out.push_str(&haystack[copied..i]);
            out.push_str(to);
            i += pat.len();
            copied = i;
        } else {
            i += 1;
        }
    }
    out.push_str(&haystack[copied..]);

    return out;
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

    #[test]
    fn test_refang_is_case_insensitive() {
        assert_eq!(refang("HXXP[://]EXAMPLE[.]COM"), "http://EXAMPLE.COM");
        assert_eq!(refang("Hxxps[://]Example[.]Com"), "https://Example.Com");
    }

    #[test]
    fn test_refang_leaves_multibyte_input_intact() {
        assert_eq!(refang("Ünïcödé[.]example[.]com"), "Ünïcödé.example.com");
    }

    #[test]
    fn test_refang_replaces_every_occurrence() {
        assert_eq!(refang("hxxp[://]a[.]b hxxp[://]c[.]d"), "http://a.b http://c.d");
        assert_eq!(refang("[.][:][@][://]"), ".:@://");
    }

    /// Defanging twice nests the brackets; refanging peels off one layer.
    #[test]
    fn test_refang_unwraps_a_single_layer() {
        assert_eq!(refang("hxxp[[://]]already[[.]]defanged"), "http[://]already[.]defanged");
    }

    #[test]
    fn test_refang_passes_through_uneventful_input() {
        assert_eq!(refang(""), "");
        assert_eq!(refang("nothing to refang"), "nothing to refang");
    }

    #[test]
    fn test_replace_ignore_ascii_case() {
        assert_eq!(replace_ignore_ascii_case("hxxp", "hxxp", "http"), "http");
        assert_eq!(replace_ignore_ascii_case("HXXP", "hxxp", "http"), "http");
        assert_eq!(replace_ignore_ascii_case("a hxxp b HXXP c", "hxxp", "http"), "a http b http c");
        assert_eq!(replace_ignore_ascii_case("", "hxxp", "http"), "");
        assert_eq!(replace_ignore_ascii_case("hxx", "hxxp", "http"), "hxx");
        // The copy offsets are byte indices, so they have to land on char
        // boundaries when a match sits between multibyte characters.
        assert_eq!(replace_ignore_ascii_case("é!hxxp!é", "hxxp", "http"), "é!http!é");
    }
}
