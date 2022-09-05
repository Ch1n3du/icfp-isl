use std::rc::Rc;

use crate::token::{Position, Token, TokenType};

#[derive(Debug)]
pub struct Scanner {
    source: Rc<Vec<u8>>,
    start: usize,
    current: usize,
    line: usize,
    col: (usize, usize),
}

impl Scanner {
    fn new(source: &[u8]) -> Scanner {
        Scanner {
            source: Rc::new(source.to_vec()),
            start: 0,
            current: 0,
            line: 0,
            col: (0, 0),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn increment_current(&mut self) {
        self.current += 1;
        self.col.1 += 1;
    }

    fn increment_line(&mut self) {
        self.line += 1;
        self.col = (0, 0)
    }

    /// Returns the next character.
    fn peek(&self) -> Option<u8> {
        if !self.is_at_end() {
            Some(self.source[self.current])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<u8> {
        if !self.is_at_end() {
            let chary = Some(self.source[self.current]);
            self.increment_current();

            chary
        } else {
            None
        }
    }

    fn get_curr_position(&self) -> Position {
        Position::new(
            self.line,
            self.col,
            &self.source,
            (self.start, self.current),
        )
    }

    fn get_curr_lexeme(&mut self) -> String {
        self.source[self.start..self.current]
            .iter()
            .cloned()
            .map(|c| c as char)
            .collect::<String>()
    }

    fn mk_token_with_num(&mut self, token_type: TokenType, num: Option<u64>) -> Option<Token> {
        let token = Token {
            token_type,
            num,
            position: self.get_curr_position(),
        };

        Some(token)
    }

    fn mk_token(&mut self, token_type: TokenType) -> Option<Token> {
        self.mk_token_with_num(token_type, None)
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            self.col = (self.col.1, self.col.1);
            match self.scan_token() {
                Some(token) => tokens.push(token),
                None => (),
            };
        }
        tokens.push(self.mk_token(TokenType::Eof).unwrap());
        tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        let tok = self.advance().unwrap();

        match tok {
            b'x' | b'X' => self.mk_token(TokenType::Vertical),
            b'y' | b'Y' => self.mk_token(TokenType::Horizontal),
            b'[' => self.mk_token(TokenType::LeftBrace),
            b']' => self.mk_token(TokenType::RightBrace),
            b',' => self.mk_token(TokenType::Comma),
            b'.' => self.mk_token(TokenType::Dot),
            b'#' => self.scan_comment(),
            b if b.is_ascii_digit() => self.scan_digit(),
            b if b.is_ascii_alphabetic() => self.scan_ident(),
            b'\n' => {
                let tok = self.mk_token(TokenType::NewLine);
                self.increment_line();
                tok
            }
            _ => None,
        }
    }

    fn scan_digit(&mut self) -> Option<Token> {
        while !self.is_at_end() && self.peek().unwrap().is_ascii_digit() {
            self.advance();
        }

        let num = self
            .get_curr_lexeme()
            .parse::<u64>()
            .expect("Error parsing number into u64");

        self.mk_token_with_num(TokenType::Number, Some(num))
    }

    fn scan_ident(&mut self) -> Option<Token> {
        while !self.is_at_end() && is_valid_ident(self.peek().unwrap()) {
            self.advance();
        }

        let token_type = match self.get_curr_lexeme().as_str() {
            "cut" => TokenType::Cut,
            "color" => TokenType::Color,
            "swap" => TokenType::Swap,
            "merge" => TokenType::Merge,
            // ! Change to Scanner Result
            lex => panic!("Unexpected identifier: {lex}"),
        };

        self.mk_token(token_type)
    }

    fn scan_comment(&mut self) -> Option<Token> {
        while !self.is_at_end() && self.peek().unwrap() != b'\n' {
            self.advance();

            if let Some(b'\n') = self.peek() {
                self.advance();
                break;
            }
        }

        None
    }

    pub fn scan_str(src: &str) -> Vec<Token> {
        let mut scanny = Scanner::new(src.as_bytes());
        scanny.scan_tokens()
    }
}

fn is_valid_ident(chary: u8) -> bool {
    chary.is_ascii_alphabetic() || chary == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_symbols() {
        let tokens = Scanner::scan_str("x X y Y [ ] , . \n 69 cut color swap merge");

        let expected_tokens = vec![
            TokenType::Vertical,
            TokenType::Vertical,
            TokenType::Horizontal,
            TokenType::Horizontal,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Comma,
            TokenType::Dot,
            TokenType::NewLine,
            TokenType::Number,
            TokenType::Cut,
            TokenType::Color,
            TokenType::Swap,
            TokenType::Merge,
            TokenType::Eof,
        ];

        assert_eq!(
            tokens.len(),
            expected_tokens.len(),
            "Incorrect number of tokens, expected {:?} got {:?}",
            tokens.len(),
            expected_tokens.len(),
        );

        expected_tokens
            .into_iter()
            .zip(tokens.into_iter().map(|t| t.token_type))
            .for_each(|(expected, token)| {
                assert_eq!(expected, token, "Expected {:?}, got {:?}", expected, token)
            });
    }
}
