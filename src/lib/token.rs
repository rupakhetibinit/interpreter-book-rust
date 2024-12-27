#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident, // add, foobar, x, y, ...
    Int,   // 1343456
    // Operators
    Assign,

    Plus,
    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    // Keywords
    Function,
    Let,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, literal: &'a str) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}
