use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub num: Option<u64>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Identifier(String),
    Number,
    Cut,
    Color,
    Swap,
    Merge,
    Vertical,
    Horizontal,
    HashTag,
    LeftBrace,
    RightBrace,
    NewLine,
    Comma,
    Dot,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub col: (usize, usize),
    pub src: Rc<Vec<u8>>,
    pub indices: (usize, usize),
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "on line {} column {}-{}",
            self.line, self.col.0, self.col.1
        )
    }
}

impl Position {
    pub fn new(
        line: usize,
        col: (usize, usize),
        src: &Rc<Vec<u8>>,
        indices: (usize, usize),
    ) -> Position {
        Position {
            line,
            col,
            src: src.clone(),
            indices,
        }
    }
}
