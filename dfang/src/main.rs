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

/// Dots and URL scheme markers are always safe to escape. The "@" and the
/// bare colons are not, so those stay behind a check: escaping them
/// unconditionally would chew through prose like "Contact: me" or "C:/tmp".
fn defang(input: &str) -> String {
    let result = input.replace('.', "[.]");
    let result = replace_preserving_ascii_case(&result, "http", "hxxp");
    let mut result = result.replace("://", "[://]");

    if has_email(input) {
        result = result.replace('@', "[@]");
    }
    if has_ipv6(input) {
        result = bracket_bare_colons(&result);
    }

    return result;
}

/// Escapes colons, skipping the ones inside a "[://]" we just wrote.
fn bracket_bare_colons(input: &str) -> String {
    const KEEP: &[u8] = b"[://]";
    let bytes = input.as_bytes();
    let mut out = String::with_capacity(input.len());
    let mut copied = 0;
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'[' && bytes[i..].starts_with(KEEP) {
            i += KEEP.len();
        } else if bytes[i] == b':' {
            out.push_str(&input[copied..i]);
            out.push_str("[:]");
            i += 1;
            copied = i;
        } else {
            i += 1;
        }
    }
    out.push_str(&input[copied..]);

    return out;
}

/// Non-overlapping, leftmost replacement of an ASCII `needle` of the same
/// length as `to`. Matching ignores case and each character written keeps the
/// case of the one it replaced, so "HTTP" becomes "HXXP" and a later refang
/// can hand back exactly what it was given.
fn replace_preserving_ascii_case(haystack: &str, needle: &str, to: &str) -> String {
    let (bytes, pat) = (haystack.as_bytes(), needle.as_bytes());
    let mut out = String::with_capacity(haystack.len());
    let mut copied = 0;
    let mut i = 0;

    while i + pat.len() <= bytes.len() {
        if bytes[i..i + pat.len()].eq_ignore_ascii_case(pat) {
            out.push_str(&haystack[copied..i]);
            for (&from, &to) in bytes[i..i + pat.len()].iter().zip(to.as_bytes()) {
                out.push(if from.is_ascii_uppercase() { to.to_ascii_uppercase() } else { to } as char);
            }
            i += pat.len();
            copied = i;
        } else {
            i += 1;
        }
    }
    out.push_str(&haystack[copied..]);

    return out;
}

/// True if the input looks like an IPv6 address. Every compressed form
/// contains a "::", so only the uncompressed ones need spelling out: eight
/// hex groups, or six followed by an embedded IPv4 address.
fn has_ipv6(input: &str) -> bool {
    let bytes = input.as_bytes();

    return bytes.windows(2).any(|w| w == b"::")
        || (0..bytes.len()).any(|i| ipv6_groups_at(&bytes[i..]));
}

fn ipv6_groups_at(bytes: &[u8]) -> bool {
    let mut i = 0;

    for group in 0..8 {
        // A dotted quad stands in for the last two groups.
        if group == 6 && dotted_quad_len(&bytes[i..]) > 0 {
            return true;
        }
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

/// Length of a leading dotted quad, 0 if there isn't one.
fn dotted_quad_len(bytes: &[u8]) -> usize {
    let mut i = 0;

    for octet in 0..4 {
        if octet > 0 {
            if bytes.get(i) != Some(&b'.') {
                return 0;
            }
            i += 1;
        }
        let len = octet_len(&bytes[i..]);
        if len == 0 {
            return 0;
        }
        i += len;
    }

    return i;
}

/// Length of a leading 0-255 octet, 0 if there isn't one.
fn octet_len(bytes: &[u8]) -> usize {
    let digits = bytes.iter().take(3).take_while(|c| c.is_ascii_digit()).count();

    if digits == 3 && bytes[..3].iter().fold(0u32, |v, d| v * 10 + (d - b'0') as u32) > 255 {
        return 0;
    }

    return digits;
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

    /// The scheme is matched regardless of case and rewritten in the case it
    /// arrived in, so nothing is lost on the way back through rfang.
    #[test]
    fn test_defang_is_case_insensitive() {
        assert_eq!(defang("HTTP://EXAMPLE.COM"), "HXXP[://]EXAMPLE[.]COM");
        assert_eq!(defang("Http://Example.Com"), "Hxxp[://]Example[.]Com");
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
        // Six groups plus an embedded IPv4 address.
        assert!(has_ipv6("a:b:c:d:e:f:1.2.3.4"));
        assert!(!has_ipv6("a:b:c:d:e:f:1.2.3"));
        assert!(!has_ipv6("a:b:c:d:e:f:256.1.1.1"));
    }

    #[test]
    fn test_dotted_quad_length() {
        assert_eq!(dotted_quad_len(b"1.2.3.4"), 7);
        assert_eq!(dotted_quad_len(b"255.255.255.255"), 15);
        assert_eq!(dotted_quad_len(b"192.168.1.1/rest"), 11);
        assert_eq!(dotted_quad_len(b"1.2.3"), 0);
        assert_eq!(dotted_quad_len(b"256.1.1.1"), 0);
        assert_eq!(dotted_quad_len(b"1000.1.1.1"), 0);
        assert_eq!(dotted_quad_len(b"example.com"), 0);
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

    /// Every applicable rule fires, so a line carrying more than one kind of
    /// IOC comes out with all of them defanged rather than just the first.
    #[test]
    fn test_defang_applies_every_applicable_rule() {
        assert_eq!(defang("http://192.168.1.1/malware.exe"), "hxxp[://]192[.]168[.]1[.]1/malware[.]exe");
        assert_eq!(defang("user@192.168.1.1"), "user[@]192[.]168[.]1[.]1");
        assert_eq!(defang("2001:db8::1 and http://evil.com"), "2001[:]db8[:][:]1 and hxxp[://]evil[.]com");
        assert_eq!(defang("foo::bar@example.com"), "foo[:][:]bar[@]example[.]com");
        assert_eq!(defang("1:2:3:4:5:6:7:8@example.com"), "1[:]2[:]3[:]4[:]5[:]6[:]7[:]8[@]example[.]com");
        assert_eq!(defang("::ffff:192.168.1.1"), "[:][:]ffff[:]192[.]168[.]1[.]1");
        assert_eq!(defang("a:b:c:d:e:f:1.2.3.4"), "a[:]b[:]c[:]d[:]e[:]f[:]1[.]2[.]3[.]4");
        assert_eq!(defang("Contact: abuse@corp.com, C2: 5.5.5.5"), "Contact: abuse[@]corp[.]com, C2: 5[.]5[.]5[.]5");
    }

    /// Colons only get escaped when the line actually holds an IPv6 address,
    /// so ordinary prose and Windows paths survive intact.
    #[test]
    fn test_defang_leaves_incidental_colons_alone() {
        assert_eq!(defang("key: value"), "key: value");
        assert_eq!(defang("C:/Users/test/file.txt"), "C:/Users/test/file[.]txt");
        assert_eq!(defang("mailto:user@example.com"), "mailto:user[@]example[.]com");
        assert_eq!(defang("seen 10.0.0.5 at 12:30:45"), "seen 10[.]0[.]0[.]5 at 12:30:45");
    }

    #[test]
    fn test_defang_does_not_double_escape_the_scheme_separator() {
        assert_eq!(defang("http://[2001:db8::1]/x"), "hxxp[://][2001[:]db8[:][:]1]/x");
        assert_eq!(bracket_bare_colons("a[://]b:c"), "a[://]b[:]c");
    }

    #[test]
    fn test_defang_replaces_every_occurrence() {
        assert_eq!(defang("httphttp://a.com"), "hxxphxxp[://]a[.]com");
        assert_eq!(defang("http://a.com/redir?u=http://b.com"), "hxxp[://]a[.]com/redir?u=hxxp[://]b[.]com");
        assert_eq!(defang("HtTpS://MiXeD.CoM"), "HxXpS[://]MiXeD[.]CoM");
    }

    #[test]
    fn test_defang_passes_through_uneventful_input() {
        assert_eq!(defang(""), "");
        assert_eq!(defang("nothing to defang"), "nothing to defang");
    }

    #[test]
    fn test_replace_preserving_ascii_case() {
        assert_eq!(replace_preserving_ascii_case("http", "http", "hxxp"), "hxxp");
        assert_eq!(replace_preserving_ascii_case("HTTP", "http", "hxxp"), "HXXP");
        assert_eq!(replace_preserving_ascii_case("Http", "http", "hxxp"), "Hxxp");
        assert_eq!(replace_preserving_ascii_case("HtTp", "http", "hxxp"), "HxXp");
        assert_eq!(replace_preserving_ascii_case("a http b HTTP c", "http", "hxxp"), "a hxxp b HXXP c");
        assert_eq!(replace_preserving_ascii_case("", "http", "hxxp"), "");
        assert_eq!(replace_preserving_ascii_case("htt", "http", "hxxp"), "htt");
        // The copy offsets are byte indices, so they have to land on char
        // boundaries when a match sits between multibyte characters.
        assert_eq!(replace_preserving_ascii_case("é!http!é", "http", "hxxp"), "é!hxxp!é");
    }
}
