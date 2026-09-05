// Explicit `return` is deliberate here; the lint would rewrite it.
#![allow(clippy::needless_return)]

use dfang::defang;
use std::env;
use std::io::{self, BufWriter, IsTerminal, Read, Write};
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        if io::stdin().is_terminal() {
            help();
        } else if let Err(err) = defang_stdin() {
            // The reader hanging up (`dfang < big.txt | head -1`) is not an error.
            if err.kind() == io::ErrorKind::BrokenPipe {
                return;
            }
            eprintln!("dfang: {err}");
            process::exit(1);
        }
    } else {
        for i in 1..args.len() {
            println!("{}", defang(&args[i]));
        }
    }
}

/// Buffered so stdout is flushed once per block rather than once per line;
/// on big inputs the per-line flushes were most of the runtime.
fn defang_stdin() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut out = BufWriter::new(io::stdout().lock());
    for line in input.lines() {
        writeln!(out, "{}", defang(line))?;
    }

    return out.flush();
}

fn help() {
    println!("dfang v{}", VERSION);
    println!("usage: dfang <string>");
}
