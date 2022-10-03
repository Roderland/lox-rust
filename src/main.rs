mod error;
mod scanner;
mod token;

use crate::error::LoxError;
use crate::scanner::Scanner;
use std::env::args;
use std::fs::File;
use std::io::{self, stdout, BufRead, BufReader, Read, Write};

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
        Err(_) => {
            // m.report("".to_string());
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
            if run(line.as_bytes()).is_ok() {}
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
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
