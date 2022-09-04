#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub position: Position,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Identifier(String),
    Number(u64),
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
}

#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub col: (usize, usize),
}

impl Position {
    pub fn new(line: usize, col: (usize, usize)) -> Position {
        Position { line, col }
    }
}
