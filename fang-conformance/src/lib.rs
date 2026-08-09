//! Tests that `dfang` and `rfang` agree on the tokens they exchange.
//!
//! Neither crate depends on the other, so neither can check that the brackets
//! one writes are the brackets the other reads. This crate is a sibling of
//! both, holds one corpus used in each direction, and is never published.

#[cfg(test)]
mod tests {
    struct Case {
        plain: &'static str,
        defanged: &'static str,
        /// What `refang` gives back, when it isn't `plain`. Defanging is not
        /// injective: an input already carrying `hxxp` or a bracket token
        /// comes out the far side as something else.
        recovered: Option<&'static str>,
    }

    impl Case {
        fn recovered(&self) -> &str {
            return self.recovered.unwrap_or(self.plain);
        }
    }

    const CASES: &[Case] = &[
        Case { plain: "http://example.com", defanged: "hxxp[://]example[.]com", recovered: None },
        Case { plain: "https://example.com", defanged: "hxxps[://]example[.]com", recovered: None },
        Case { plain: "HTTP://EXAMPLE.COM", defanged: "HXXP[://]EXAMPLE[.]COM", recovered: None },
        Case { plain: "HtTpS://MiXeD.CoM", defanged: "HxXpS[://]MiXeD[.]CoM", recovered: None },
        Case { plain: "example@example.com", defanged: "example[@]example[.]com", recovered: None },
        Case { plain: "192.168.1.1", defanged: "192[.]168[.]1[.]1", recovered: None },
        Case {
            plain: "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
            defanged: "2001[:]0db8[:]85a3[:]0000[:]0000[:]8a2e[:]0370[:]7334",
            recovered: None,
        },
        Case { plain: "2001:db8::1", defanged: "2001[:]db8[:][:]1", recovered: None },
        Case { plain: "::ffff:192.168.1.1", defanged: "[:][:]ffff[:]192[.]168[.]1[.]1", recovered: None },
        Case {
            plain: "http://192.168.1.1/malware.exe",
            defanged: "hxxp[://]192[.]168[.]1[.]1/malware[.]exe",
            recovered: None,
        },
        Case {
            plain: "Contact: abuse@corp.com, C2: 5.5.5.5",
            defanged: "Contact: abuse[@]corp[.]com, C2: 5[.]5[.]5[.]5",
            recovered: None,
        },
        Case { plain: "Ünïcödé.example.com", defanged: "Ünïcödé[.]example[.]com", recovered: None },
        // Colons only get bracketed when the line holds an IPv6 address, so
        // these survive defanging untouched and come back unchanged.
        Case { plain: "key: value", defanged: "key: value", recovered: None },
        Case { plain: "C:/Users/test/file.txt", defanged: "C:/Users/test/file[.]txt", recovered: None },
        Case { plain: "seen 10.0.0.5 at 12:30:45", defanged: "seen 10[.]0[.]0[.]5 at 12:30:45", recovered: None },
        Case { plain: "nothing to defang", defanged: "nothing to defang", recovered: None },
        Case { plain: "", defanged: "", recovered: None },
        // Input already carrying a bracket token nests one layer deeper on the
        // way out, and the brackets unwrap back to where they started. The
        // "hxxp" doesn't, for the same reason as the case below.
        Case {
            plain: "hxxp[://]already[.]defanged",
            defanged: "hxxp[[://]]already[[.]]defanged",
            recovered: Some("http[://]already[.]defanged"),
        },
        // One-way. Nothing marks the "hxxp" as having been literal, so refang
        // reads it as a defanged scheme and hands back "http".
        Case {
            plain: "hxxp://evil.com",
            defanged: "hxxp[://]evil[.]com",
            recovered: Some("http://evil.com"),
        },
        // One-way for the same reason: a bare "[:]" in the input is
        // indistinguishable from one dfang wrote.
        Case { plain: "a[:]b", defanged: "a[:]b", recovered: Some("a:b") },
    ];

    #[test]
    fn defang_writes_the_tokens_the_corpus_expects() {
        for case in CASES {
            assert_eq!(dfang::defang(case.plain), case.defanged, "defanging {:?}", case.plain);
        }
    }

    #[test]
    fn refang_reads_back_every_token_defang_writes() {
        for case in CASES {
            assert_eq!(rfang::refang(case.defanged), case.recovered(), "refanging {:?}", case.defanged);
        }
    }

    /// The asymmetry is narrow enough to be worth stating outright: everything
    /// that doesn't already look defanged makes the round trip intact.
    #[test]
    fn the_round_trip_is_lossless_except_where_the_corpus_says_otherwise() {
        for case in CASES.iter().filter(|c| c.recovered.is_none()) {
            assert_eq!(rfang::refang(&dfang::defang(case.plain)), case.plain);
        }
    }
}
