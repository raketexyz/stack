mod ast;
mod value;
mod parser;
mod interpreter;

pub use ast::*;
pub use value::*;
pub use parser::*;
pub use interpreter::*;

use std::{io::{stdin, stdout, Write}, process::exit};

use nom::error::convert_error;

pub fn run_repl(verbose: bool) {
    let mut interpreter = Interpreter::new(verbose);
    let mut input = String::new();

    println!(
        "{} v{}  Copyright (C) 2024  rakete
This program is free software: you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );
    print!(">>> ");

    loop {
        interpreter.statements.clear();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("Couldn't read stdin");

        if verbose {
            println!("Input: {input:?}")
        }

        let program = match program(&input) {
            Ok((_, program)) => program,
            Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
                eprintln!("Parser error: {}", convert_error(input.as_str(), e));
                input.clear();
                print!(">>> ");
                continue;
            }
            Err(nom::Err::Incomplete(_)) => {
                print!("... ");
                continue;
            }
        };

        match interpreter.run_program(program) {
            Ok(None) => {}
            Ok(Some(v)) => println!("{v}"),
            Err(e) => eprintln!("{e}"),
        }

        input.clear();
        print!(">>> ");
    }
}

pub fn run_program(input: &str, verbose: bool) {
    let res = Interpreter::new(verbose).run_program(parse_program(input));

    if let Err(e) = res {
        eprintln!("Interpreter error: {e}");
    }
}

/// Tries to parse a program and exits on failure.
fn parse_program(input: &str) -> Program {
    match program(input) {
        Ok((_, program)) => program,
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
            eprintln!("Parser error: {}", convert_error(input, e));
            exit(-1)
        }
        Err(nom::Err::Incomplete(_)) => {
            eprintln!("Input seems incomplete. (This should be unreachable. \
                       What are you doing?)");
            unreachable!()
        }
    }
}
