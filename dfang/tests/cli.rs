// Explicit `return` is deliberate here; the lint would rewrite it.
#![allow(clippy::needless_return)]

use std::io::Write;
use std::process::{Command, Stdio};

fn run(input: &[u8]) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_dfang"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();

    return child.wait_with_output().unwrap();
}

#[test]
fn defangs_every_piped_line() {
    let output = run(b"http://a.com\nuser@b.org\n\n10.0.0.1");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "hxxp[://]a[.]com\nuser[@]b[.]org\n\n10[.]0[.]0[.]1\n"
    );
}

/// A downstream reader that hangs up early (`dfang < big.txt | head -1`)
/// should end the run quietly, not with a panic on stderr.
#[test]
fn exits_quietly_when_the_reader_hangs_up() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_dfang"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdout.take());
    child
        .stdin
        .take()
        .unwrap()
        .write_all("http://example.com\n".repeat(100_000).as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
}
