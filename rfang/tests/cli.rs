// Explicit `return` is deliberate here; the lint would rewrite it.
#![allow(clippy::needless_return)]

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn spawn(args: &[&str]) -> Child {
    return Command::new(env!("CARGO_BIN_EXE_rfang"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
}

#[test]
fn refangs_every_piped_line() {
    let mut child = spawn(&[]);
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"hxxp[://]a[.]com\r\nuser[@]b[.]org\n\n10[.]0[.]0[.]1")
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "http://a.com\nuser@b.org\n\n10.0.0.1\n"
    );
}

/// Input is handled as it arrives, so a line comes back before stdin closes.
/// Without `--line-buffered` it would sit in the output buffer instead.
#[test]
fn streams_each_line_as_it_arrives() {
    let mut child = spawn(&["--line-buffered"]);
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    stdin.write_all(b"hxxp[://]a[.]com\n").unwrap();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut line = String::new();
        stdout.read_line(&mut line).unwrap();
        tx.send(line).unwrap();
    });
    let line = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("no output before stdin was closed");
    assert_eq!(line, "http://a.com\n");

    drop(stdin);
    assert!(child.wait().unwrap().success());
}

/// A downstream reader that hangs up early (`rfang < big.txt | head -1`)
/// should end the run quietly, not with a panic on stderr.
#[test]
fn exits_quietly_when_the_reader_hangs_up() {
    let mut child = spawn(&[]);
    drop(child.stdout.take());
    // The binary exits partway through this write, so it may fail too.
    let _ = child
        .stdin
        .take()
        .unwrap()
        .write_all("hxxp[://]example[.]com\n".repeat(100_000).as_bytes());

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
}
