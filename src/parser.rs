use crate::{
    ast::{BlockId, Color, Move, Orientation, Point},
    token::{Position, Token, TokenType},
};

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug)]
pub struct Parser {
    source: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(source: &[Token]) -> Parser {
        Parser {
            source: source.to_vec(),
            current: 0,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current > self.source.len() - 2
    }

    fn increment_current(&mut self) {
        self.current += 1;
    }

    fn get_curr_position(&self) -> Position {
        if let Some(tok) = self.peek() {
            tok.position
        } else {
            self.peek().unwrap().position
        }
    }

    fn previous(&self) -> Token {
        self.source[self.current - 1].to_owned()
    }

    /// Returns the next token.
    fn peek(&self) -> Option<Token> {
        if !self.is_at_end() {
            Some(self.source[self.current].to_owned())
        } else {
            None
        }
    }

    /// Returns the next token.
    fn peek_twice(&self) -> Option<Token> {
        if self.current < self.source.len() - 2 {
            Some(self.source[self.current + 1].to_owned())
        } else {
            None
        }
    }

    /// Check if the next token matches 'token_type'
    fn check(&self, token_type: TokenType) -> bool {
        if let Some(tok) = self.peek() {
            tok.token_type == token_type
        } else {
            false
        }
    }

    /// Check if the next token matches 'token_type'
    fn check_twice(&self, token_type: TokenType) -> bool {
        if let Some(tok) = self.peek_twice() {
            tok.token_type == token_type
        } else {
            false
        }
    }

    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            let tok = Some(self.source[self.current].to_owned());
            self.increment_current();

            tok
        } else {
            None
        }
    }

    /// Returns the next token if it matches `token_type`.
    /// !Throws an error if the next type doesn't match.
    fn consume(&mut self, token_type: TokenType, msg: &str) -> ParserResult<Token> {
        if self.check(token_type.clone()) {
            Ok(self.advance().unwrap())
        } else {
            Err(expected(&[token_type], msg, self.get_curr_position()))
        }
    }

    /// Advances if next token matches 'token_type'
    /// Be careful, can cause tears
    fn matches(&mut self, token_types: &[TokenType]) -> bool {
        let mut val = false;

        for token_type in token_types {
            if self.check(token_type.clone()) {
                self.advance();
                val = true;
                break;
            }
        }

        val
    }

    // ! ENTRY POINT
    pub fn parse(&mut self) -> ParserResult<Vec<Move>> {
        self.program()
    }

    /// program -> program-line | program-line newline program
    fn program(&mut self) -> ParserResult<Vec<Move>> {
        let mut moves = Vec::new();
        while !self.is_at_end() {
            moves.push(self.program_line()?);
        }

        Ok(moves)
    }

    fn program_line(&mut self) -> ParserResult<Move> {
        let move_ = self.move_()?;

        if self.check(TokenType::NewLine) {
            self.consume(TokenType::NewLine, "Expected newline after move")?;
        }

        Ok(move_)
    }

    /// move -> cut-move
    ///       | <color-move>
    ///       | <swap-move>
    ///       | <merge-move> ;
    fn move_(&mut self) -> ParserResult<Move> {
        use TokenType::*;
        if self.check(Cut) {
            self.cut_move()
        } else if self.check(Color) {
            self.color_move()
        } else if self.check(Swap) {
            self.swap_move()
        } else if self.check(Merge) {
            self.merge_move()
        } else {
            Err(expected(
                &[
                    TokenType::Cut,
                    TokenType::Color,
                    TokenType::Swap,
                    TokenType::Merge,
                ],
                "Expected move command name at start of l",
                self.get_curr_position(),
            ))
        }
    }

    /// cut-move -> pcut-move | lcut-move ;
    fn cut_move(&mut self) -> ParserResult<Move> {
        let tok = self.consume(TokenType::Cut, "")?;
        let position = tok.position;
        let block_id = self.block()?;

        // pcut-move -> "cut" block point
        let move_ = if self.check_twice(TokenType::Number) {
            let point = self.point()?;

            Move::PCut {
                block_id,
                point,
                position,
            }

        // lcut-move -> "cut" block orientation line-number
        } else {
            let orientation = self.orientation()?;
            let line_no = self.line_number()?;

            Move::LCut {
                block_id,
                orientation,
                line_no,
                position,
            }
        };

        Ok(move_)
    }

    /// <color-move> -> "color" block color ;
    fn color_move(&mut self) -> ParserResult<Move> {
        let tok = self.consume(TokenType::Color, "")?;
        let block_id = self.block()?;
        let color = self.color()?;

        Ok(Move::Color {
            block_id,
            color,
            position: tok.position,
        })
    }

    /// swap-move -> "swap" block block ;
    fn swap_move(&mut self) -> ParserResult<Move> {
        let tok = self.consume(TokenType::Swap, "")?;
        let block_id_1 = self.block()?;
        let block_id_2 = self.block()?;

        Ok(Move::Swap {
            block_id_1,
            block_id_2,
            position: tok.position,
        })
    }

    /// merge-move -> "merge" block block ;
    fn merge_move(&mut self) -> ParserResult<Move> {
        let tok = self.consume(TokenType::Merge, "")?;
        let block_id_1 = self.block()?;
        let block_id_2 = self.block()?;

        Ok(Move::Merge {
            block_id_1,
            block_id_2,
            position: tok.position,
        })
    }

    /// orientation -> "[" <orientation-type> "]"
    fn orientation(&mut self) -> ParserResult<Orientation> {
        self.consume(
            TokenType::LeftBrace,
            "Expected left brace in the beginning of an orientation.",
        )?;
        let orientation = self.orientation_type()?;
        self.consume(
            TokenType::RightBrace,
            "Expected right brace after an orientation",
        )?;

        Ok(orientation)
    }

    /// <orientation-type> ::= <vertical> | <horizontal>
    /// <vertical> ::= "X" | "x"
    /// <horizontal> ::= "Y" | "y"
    fn orientation_type(&mut self) -> ParserResult<Orientation> {
        if let Some(tok) = self.advance() {
            match tok.token_type {
                TokenType::Horizontal => Ok(Orientation::Horizontal),
                TokenType::Vertical => Ok(Orientation::Vertical),
                _ => Err(expected(
                    &[TokenType::Vertical, TokenType::Horizontal],
                    "Expected an orientation type",
                    tok.position,
                )),
            }
        } else {
            Err(ParserError::Eof(self.get_curr_position()))
        }
    }

    // <line-number> ::= "[" <number> "]"
    fn line_number(&mut self) -> ParserResult<u64> {
        self.consume(
            TokenType::LeftBrace,
            "expected '[' at the start of a line number",
        )?;
        let num = self.number()?;
        self.consume(
            TokenType::RightBrace,
            "Expected ']' after number in a line number",
        )?;

        Ok(num)
    }

    /// <point> ::= "[" <x> "," <y> "]"
    fn point(&mut self) -> ParserResult<Point> {
        self.consume(TokenType::LeftBrace, "Expected '[' at the start of a point")?;
        let x = self.number()?;
        self.consume(
            TokenType::Comma,
            "Expected a ',' after the 'x' value in a point",
        )?;
        let y = self.number()?;
        self.consume(
            TokenType::RightBrace,
            "Expected a ']' after the 'y' value in a point",
        )?;

        Ok(Point { x, y })
    }

    /// <color> ::= "[" <r> "," <g> "," <b> "," <a> "]"
    fn color(&mut self) -> ParserResult<Color> {
        self.consume(
            TokenType::LeftBrace,
            "Expected a '[' at the beginning of a color.",
        )?;
        let r = self.rgb_value()?;
        self.consume(
            TokenType::Comma,
            "Expected a ',' after the 'r' value in a color.",
        )?;
        let g = self.rgb_value()?;
        self.consume(
            TokenType::Comma,
            "Expected a ',' after the 'g' value in a color.",
        )?;
        let b = self.rgb_value()?;
        self.consume(
            TokenType::Comma,
            "Expected a ',' after the 'b' value in a color.",
        )?;
        let a = self.rgb_value()?;
        self.consume(
            TokenType::RightBrace,
            "Expcted a ']' after the 'a' value in a color.",
        )?;

        Ok(Color(r, g, b, a))
    }

    /// <r> | <g> | <b> | <a> ::= "0", "1", "2"..."255"
    fn rgb_value(&mut self) -> ParserResult<u8> {
        let tok = self.consume(
            TokenType::Number,
            "Expected a number from 0-255 for an rgb value.",
        )?;
        let num = tok.num.unwrap();

        if num < 256 {
            Ok(num as u8)
        } else {
            Err(ParserError::TooBigForRGBA(num, tok.position))
        }
    }

    /// <block> ::= "[" <block-id> "]"
    fn block(&mut self) -> ParserResult<BlockId> {
        self.consume(
            TokenType::LeftBrace,
            "Expected a '[' at the beginning of a block.",
        )?;
        let id = self.block_id()?;
        self.consume(
            TokenType::RightBrace,
            "Expected a ']' at the end of a block.",
        )?;

        Ok(id)
    }

    // 23 . 21 . 12
    /// <block-id> ::= <id> | <id> "." <block-id>
    fn block_id(&mut self) -> ParserResult<BlockId> {
        let mut block_id = BlockId {
            prev: None,
            id: self.number()?,
        };

        while self.matches(&[TokenType::Dot]) {
            block_id = BlockId {
                prev: Some(Box::new(block_id)),
                id: self.number()?,
            };
        }

        Ok(block_id)
    }

    /// <id> | <number> ::= "0", "1", "2"...
    fn number(&mut self) -> ParserResult<u64> {
        let tok = self.consume(TokenType::Number, "")?;
        Ok(tok.num.unwrap())
    }

    /// <newline> ::= "\n"
    fn newline(&mut self) -> ParserResult<()> {
        self.consume(TokenType::NewLine, "Expected a newline.")?;

        Ok(())
    }
}

#[derive(Debug, Diagnostic, Error)]
pub enum ParserError {
    #[diagnostic(code(parser::Parser))]
    #[error("Expected {0:?}, at {1:?}.")]
    Expected(TokenType, String, Position),
    #[error("Expected One of {token_types:?} at {position:?}, {reason}.")]
    ExpectedOneOf {
        token_types: Vec<TokenType>,
        reason: String,
        position: Position,
    },
    #[error("{0}, at {1:?}")]
    TooBigForRGBA(u64, Position),
    #[error("At the end of the file, at {0:?}.")]
    Eof(Position),
}

fn expected(token_types: &[TokenType], reason: &str, position: Position) -> ParserError {
    if token_types.len() == 1 {
        ParserError::Expected(token_types[0].clone(), reason.to_string(), position)
    } else {
        ParserError::ExpectedOneOf {
            token_types: token_types.to_vec(),
            reason: reason.to_string(),
            position,
        }
    }
}

pub type ParserResult<T> = Result<T, ParserError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;

    fn can_parse(src: &str, verbose: bool) {
        let tokens = Scanner::scan_str(src);
        if verbose {
            tokens
                .iter()
                .enumerate()
                .for_each(|(i, tok)| println!("[{i}] :-> {tok:?}"))
        }
        let mut parsy = Parser::new(&tokens);
        let res = parsy.parse();

        match res {
            Ok(yay) => println!("SUCCESS: {yay:?}"),
            Err(neh) => panic!("FAILURE, MOTHER RUSSIA IS DISAPPOINTED:\n{neh}"),
        }
    }

    #[test]
    fn can_parse_pcut_move() {
        can_parse("cut [69] [1, 2]", false)
    }

    #[test]
    fn can_parse_lcut_move() {
        can_parse("cut [69] [y] [75]", false)
    }

    #[test]
    fn can_parse_color_move() {
        can_parse("color [12] [0, 0, 0, 1]\ncut [69] [y] [17]\n", false)
    }

    #[test]
    fn can_parse_swap_move() {
        can_parse("swap [69] [96]", false)
    }

    #[test]
    fn can_parse_merge_move() {
        can_parse("merge [69] [96]", false)
    }
}
