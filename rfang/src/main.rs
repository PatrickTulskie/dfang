// Explicit `return` is deliberate here; the lint would rewrite it.
#![allow(clippy::needless_return)]

use rfang::refang;
use std::env;
use std::io::{self, BufRead, BufWriter, IsTerminal, Write};
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let line_buffered = args.iter().any(|a| a == "--line-buffered");
    args.retain(|a| a != "--line-buffered");

    if args.is_empty() {
        if io::stdin().is_terminal() {
            help();
        } else if let Err(err) = refang_stdin(line_buffered) {
            // The reader hanging up (`rfang < big.txt | head -1`) is not an error.
            if err.kind() == io::ErrorKind::BrokenPipe {
                return;
            }
            eprintln!("rfang: {err}");
            process::exit(1);
        }
    } else {
        for arg in &args {
            println!("{}", refang(arg));
        }
    }
}

/// Lines are handled as they arrive, so a `tail -f` can be piped in and memory
/// is bounded by the longest line. Output is flushed once per block, or once
/// per line when asked, since a block can sit unseen while the input is slow.
fn refang_stdin(line_buffered: bool) -> io::Result<()> {
    let mut input = io::stdin().lock();
    let mut out = BufWriter::new(io::stdout().lock());
    let mut line = String::new();

    while input.read_line(&mut line)? > 0 {
        // Same terminator handling as `str::lines`, so output is unchanged.
        writeln!(out, "{}", refang(line.lines().next().unwrap_or("")))?;
        if line_buffered {
            out.flush()?;
        }
        line.clear();
    }

    return out.flush();
}

fn help() {
    println!("rfang v{}", VERSION);
    println!("usage: rfang <string>...");
    println!("       rfang [--line-buffered] < input");
    println!();
    println!("  --line-buffered  flush after every line instead of every block");
}
