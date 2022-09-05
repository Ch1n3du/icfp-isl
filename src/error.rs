use crate::{interpreter::InterpreterError, parser::ParserError};

use colored::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ISLError {
    Parser(#[from] ParserError),
    Interpreter(#[from] InterpreterError),
    IO(#[from] std::io::Error),
}

impl std::fmt::Display for ISLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ISLError::*;
        match self {
            Parser(err) => {
                write!(
                    f,
                    "{} {}",
                    "Parser Error:".red().bold(),
                    format!("{err}").white()
                )
            }
            Interpreter(err) => {
                write!(
                    f,
                    "{} {}",
                    "Interpreter Error:".red().bold(),
                    format!("{err}").white()
                )
            }
            IO(err) => {
                write!(
                    f,
                    "{} {}",
                    "Parser Error:".red().bold(),
                    format!("{err}").white()
                )
            }
        }
    }
}

pub type ISLResult<T> = Result<T, ISLError>;
