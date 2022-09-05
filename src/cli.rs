use crate::{error::ISLError, interpreter::Interpreter, parser::Parser, scanner::Scanner};
use colored::*;
use std::{
    borrow::Borrow,
    io::{self, stdin, Write},
};

pub fn run_repl(verbose: bool) {
    let mut interpreter = Interpreter::new();
    println!("ICFP ISL Interpreter Version 1.0.0");
    println!("Enter ':q' to quit.");

    'a: loop {
        print!(" λ> ");
        let mut input = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
            println!("{}", ISLError::IO(e));
            break 'a;
        }

        if input.contains(":q") || input.contains("exit") {
            println!("{}", "Goodbye and thanks for all the fish".green());
            break 'a;
        }

        let tokens = Scanner::scan_str(&input);
        let moves = match Parser::parse_tokens(&tokens) {
            Ok(yay) => yay,
            Err(e) => {
                println!("{}", ISLError::Parser(e));
                break 'a;
            }
        };
        match interpreter.interpret(&moves, verbose) {
            Ok(cost) => {
                println!("{} {}", "Cost:".blink().bold(), cost);
            }
            Err(e) => {
                println!("{}", ISLError::Interpreter(e));
                break 'a;
            }
        };
    }
}

pub fn run_file(file_name: String, verbose: bool) {
    let mut interpreter = Interpreter::new();
    let src = match std::fs::read_to_string(file_name) {
        Ok(s) => s,
        Err(e) => {
            println!("{}", ISLError::IO(e));
            return;
        }
    };

    let tokens = Scanner::scan_str(&src);
    let moves = match Parser::parse_tokens(&tokens) {
        Ok(yay) => yay,
        Err(e) => {
            println!("{}", ISLError::Parser(e));
            return;
        }
    };
    match interpreter.interpret(&moves, verbose) {
        Ok(cost) => {
            println!("{} {}", "Total Cost:".blink().bold(), cost);
        }
        Err(e) => {
            println!("{}", ISLError::Interpreter(e));
        }
    };
}
