mod error;
mod token;
mod scanner;

use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use crate::error::LoxError;
use crate::scanner::Scanner;

pub fn main() {
    let args: Vec<String> = args().collect();
    println!("args: {:?}", args);

    if args.len() > 2 {
        println!("Usage: lox-rust [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) -> io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    let _ = reader.read_to_end(&mut buffer);
    match run(&buffer) {
        Ok(_) => {}
        Err(m) => {
            m.report("".to_string());
            std::process::exit(65);
        }
    }
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line.as_bytes()) {
                Ok(_) => {}
                Err(m) => {
                    m.report("".to_string());
                }
            }
        } else {
            break;
        }
    }
}

fn run(source: &[u8]) -> Result<(), LoxError> {
    let source = String::from_utf8(source.repeat(1)).expect("from utf8 error");
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
