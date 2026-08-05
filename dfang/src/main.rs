use std::env;
use std::io::{self, Read, IsTerminal};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const LOCAL_PART_SYMBOLS: &[u8] = b"!#$%&'*+/=?^_{|.}~-";

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
    if has_ipv4(input) {
        return defang_ipv4(input);
    } else if has_ipv6(input) {
        return defang_ipv6(input);
    } else if has_email(input) {
        return defang_email(input)
    } else {
        return defang_url(input);
    }
}

fn defang_url(input: &str) -> String {
    let result = input.replace('.', "[.]");
    let result = replace_ignore_ascii_case(&result, "http", "hxxp");

    return result.replace("://", "[://]");
}

fn defang_ipv4(input: &str) -> String {
    return input.replace('.', "[.]");
}

fn defang_ipv6(input: &str) -> String {
    return input.replace(':', "[:]");
}

fn defang_email(input: &str) -> String {
    return input.replace('.', "[.]").replace('@', "[@]");
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

/// True if any substring parses as four dot-separated octets.
fn has_ipv4(input: &str) -> bool {
    let bytes = input.as_bytes();

    return (0..bytes.len()).any(|i| ipv4_at(&bytes[i..], 0));
}

fn ipv4_at(bytes: &[u8], octets: usize) -> bool {
    // Longest octet first, falling back to shorter ones so a leading digit can
    // be skipped when that is what lines the separator up (e.g. "2550.1.1.1").
    for len in (1..=3).rev() {
        if !is_octet(bytes, len) {
            continue;
        }
        if octets == 3 {
            return true;
        }
        if bytes.get(len) == Some(&b'.') && ipv4_at(&bytes[len + 1..], octets + 1) {
            return true;
        }
    }

    return false;
}

fn is_octet(bytes: &[u8], len: usize) -> bool {
    if bytes.len() < len || !bytes[..len].iter().all(u8::is_ascii_digit) {
        return false;
    }
    if len < 3 {
        return true;
    }

    return bytes[..3].iter().fold(0u32, |v, d| v * 10 + (d - b'0') as u32) <= 255;
}

/// True if the input looks like an IPv6 address. Only ever called once
/// `has_ipv4` has ruled the input out, so the forms embedding a dotted quad
/// (`::ffff:1.2.3.4`) can't reach here: what's left is either a `::` run or
/// eight uncompressed hex groups.
fn has_ipv6(input: &str) -> bool {
    let bytes = input.as_bytes();

    return bytes.windows(2).any(|w| w == b"::")
        || (0..bytes.len()).any(|i| eight_hex_groups_at(&bytes[i..]));
}

fn eight_hex_groups_at(bytes: &[u8]) -> bool {
    let mut i = 0;

    for group in 0..8 {
        let len = hex_group_len(&bytes[i..]);
        if len == 0 {
            return false;
        }
        i += len;
        if group < 7 {
            if bytes.get(i) != Some(&b':') {
                return false;
            }
            i += 1;
        }
    }

    return true;
}

fn hex_group_len(bytes: &[u8]) -> usize {
    return bytes.iter().take(4).take_while(|c| c.is_ascii_hexdigit()).count();
}

/// True if the input contains `local@domain.tld`.
fn has_email(input: &str) -> bool {
    let bytes = input.as_bytes();

    return bytes.iter().enumerate().any(|(i, &c)| {
        c == b'@'
            && i > 0
            && (bytes[i - 1].is_ascii_alphanumeric() || LOCAL_PART_SYMBOLS.contains(&bytes[i - 1]))
            && is_dotted_domain(&bytes[i + 1..])
    });
}

fn is_dotted_domain(bytes: &[u8]) -> bool {
    let mut i = 0;
    let mut labels = 0;

    loop {
        let len = domain_label_len(&bytes[i..]);
        if len == 0 {
            return false;
        }
        labels += 1;
        if labels == 2 {
            return true;
        }
        i += len;
        if bytes.get(i) != Some(&b'.') {
            return false;
        }
        i += 1;
    }
}

/// Length of a leading `[a-z0-9]([a-z0-9-]*[a-z0-9])?` label, 0 if there isn't one.
fn domain_label_len(bytes: &[u8]) -> usize {
    if !bytes.first().is_some_and(u8::is_ascii_alphanumeric) {
        return 0;
    }

    let mut end = 1;
    for (i, c) in bytes.iter().enumerate().skip(1) {
        if c.is_ascii_alphanumeric() {
            end = i + 1;
        } else if *c != b'-' {
            break;
        }
    }

    return end;
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

    #[test]
    fn test_defang_is_case_insensitive() {
        assert_eq!(defang("HTTP://EXAMPLE.COM"), "hxxp[://]EXAMPLE[.]COM");
        assert_eq!(defang("Http://Example.Com"), "hxxp[://]Example[.]Com");
    }

    #[test]
    fn test_ipv4_matching() {
        assert!(has_ipv4("192.168.1.1"));
        assert!(has_ipv4("visit 192.168.1.1 now"));
        assert!(has_ipv4("255.255.255.255"));
        assert!(has_ipv4("01.02.03.04"));
        // Unanchored, so an address can start part way through a run of digits:
        // "999.1.1.1" matches because "99.1.1.1" does.
        assert!(has_ipv4("2550.1.1.1"));
        assert!(has_ipv4("999.1.1.1"));
        assert!(!has_ipv4("1.2550.1.1"));
        assert!(!has_ipv4("1.2.3"));
        assert!(!has_ipv4("example.com"));
        // No such rescue when every group is over 255 and three digits wide,
        // since a shorter octet would no longer end at a separator.
        assert!(!has_ipv4("256.256.256.256"));
        assert!(!has_ipv4("300.400.500.600"));
        assert!(!has_ipv4("999.999.999.999"));
    }

    #[test]
    fn test_ipv6_matching() {
        assert!(has_ipv6("::"));
        assert!(has_ipv6("::1"));
        assert!(has_ipv6("fe80::1%eth0"));
        assert!(has_ipv6("1:2:3:4:5:6:7:8"));
        assert!(has_ipv6("1:2:3:4:5:6:7:8:9"));
        assert!(has_ipv6("12345::"));
        assert!(!has_ipv6("key: value"));
        assert!(!has_ipv6("C:/Users/test/file.txt"));
        assert!(!has_ipv6("1:2:3:4:5:6:7"));
    }

    #[test]
    fn test_email_matching() {
        assert!(has_email("example@example.com"));
        assert!(has_email("mailto:user@example.com"));
        assert!(has_email("Example.User+tag@Sub.Example.CO.UK"));
        assert!(!has_email("user@localhost"));
        assert!(!has_email("@example.com"));
        // Labels may not start or end with a hyphen.
        assert!(!has_email("user@b-.com"));
        assert!(!has_email("user@-b.com"));
    }

    #[test]
    fn test_defang_leaves_multibyte_input_intact() {
        assert_eq!(defang("Ünïcödé.example.com"), "Ünïcödé[.]example[.]com");
        assert_eq!(defang("日本.example.com"), "日本[.]example[.]com");
    }

    /// The first matching branch wins and handles the whole line. `has_ipv6`
    /// relies on IPv4 being tested first, so reordering these must fail loudly.
    #[test]
    fn test_defang_branch_precedence() {
        // A dotted quad anywhere outranks IPv6, so the colons are left alone.
        assert_eq!(defang("::ffff:192.168.1.1"), "::ffff:192[.]168[.]1[.]1");
        assert_eq!(defang("a:b:c:d:e:f:1.2.3.4"), "a:b:c:d:e:f:1[.]2[.]3[.]4");
        // ...and outranks the URL and email branches, which is why the scheme
        // and the "@" survive here.
        assert_eq!(defang("http://192.168.1.1/path"), "http://192[.]168[.]1[.]1/path");
        assert_eq!(defang("user@192.168.1.1"), "user@192[.]168[.]1[.]1");
        // IPv6 outranks email.
        assert_eq!(defang("foo::bar@example.com"), "foo[:][:]bar@example.com");
        assert_eq!(defang("1:2:3:4:5:6:7:8@example.com"), "1[:]2[:]3[:]4[:]5[:]6[:]7[:]8@example.com");
    }

    #[test]
    fn test_defang_replaces_every_occurrence() {
        assert_eq!(defang("httphttp://a.com"), "hxxphxxp[://]a[.]com");
        assert_eq!(defang("http://a.com/redir?u=http://b.com"), "hxxp[://]a[.]com/redir?u=hxxp[://]b[.]com");
        assert_eq!(defang("HtTpS://MiXeD.CoM"), "hxxpS[://]MiXeD[.]CoM");
    }

    #[test]
    fn test_defang_passes_through_uneventful_input() {
        assert_eq!(defang(""), "");
        assert_eq!(defang("nothing to defang"), "nothing to defang");
    }

    #[test]
    fn test_replace_ignore_ascii_case() {
        assert_eq!(replace_ignore_ascii_case("http", "http", "hxxp"), "hxxp");
        assert_eq!(replace_ignore_ascii_case("HTTP", "http", "hxxp"), "hxxp");
        assert_eq!(replace_ignore_ascii_case("a http b HTTP c", "http", "hxxp"), "a hxxp b hxxp c");
        assert_eq!(replace_ignore_ascii_case("", "http", "hxxp"), "");
        assert_eq!(replace_ignore_ascii_case("htt", "http", "hxxp"), "htt");
        // The copy offsets are byte indices, so they have to land on char
        // boundaries when a match sits between multibyte characters.
        assert_eq!(replace_ignore_ascii_case("é!http!é", "http", "hxxp"), "é!hxxp!é");
    }
}
