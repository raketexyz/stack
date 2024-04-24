use std::{fs, io::{stdin, IsTerminal, Read}, path::PathBuf};

use clap::Parser;
use stack::{run_program, run_repl};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// File to read or `-` for stdin.
    file: Option<PathBuf>,
    /// Verbose mode. (good for debugging)
    #[arg(short)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.file {
        Some(f) if f != PathBuf::from("-") => {
            let input = fs::read_to_string(f).expect("Couldn't read file");

            run_program(&input, cli.verbose)
        },
        _ if stdin().is_terminal() => run_repl(cli.verbose),
        _ => {
            let mut input = String::new();

            stdin().read_to_string(&mut input).expect("Couldn't read stdin");

            run_program(&input, cli.verbose)
        }
    }
}
