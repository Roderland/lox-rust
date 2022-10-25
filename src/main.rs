mod error;
mod scanner;
mod token;
mod expr;
mod parser;
mod object;

use crate::error::{LoxError, SyntaxError};
use crate::scanner::Scanner;
use std::env::args;
use std::fs::File;
use std::io::{self, stdout, BufRead, BufReader, Read, Write};
use crate::LoxError::{Runtime, Syntax};
use crate::parser::Parser;

pub fn main() {
    let args: Vec<String> = args().collect();
    println!("args: {:?}", args);
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("Could not run file"),
        _ => {
            println!("Usage: lox-rust [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &String) -> io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    let _ = reader.read_to_end(&mut buffer);
    match run(&buffer) {
        Ok(_) => {}
        Err(e) => {
            // m.report("".to_string());
            println!("{}", e);
            std::process::exit(65);
        }
    }
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    let _ = stdout().flush();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line.as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    // m.report("".to_string());
                    println!("{}", e);
                }
            }
            print!("> ");
            let _ = stdout().flush();
        } else {
            break;
        }
    }
}

fn run(source: &[u8]) -> Result<(), LoxError> {
    let source = String::from_utf8(source.to_vec())
        .expect("from utf8 error")
        .chars()
        .collect::<Vec<char>>();

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().map_err(Syntax)?;

    let mut parser = Parser::new(tokens.clone());
    let expr = parser.parse().map_err(Syntax)?;
    let result = expr.eval().map_err(Runtime)?;
    println!("{}", result);
    Ok(())
}
