use dfang::defang;
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
