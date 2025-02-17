#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    Arrow,
    BraceLeft,
    BraceRight,
    Error,
    Identifier,
    Integer,
    Return,
    Semicolon,
}

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub start: usize,
    pub length: usize,
}
