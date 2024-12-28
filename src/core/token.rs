#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum TokenType {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident, // add, foobar, x, y, ...
    Int,   // 1343456
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
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
    True,
    False,
    Else,
    If,
    Return,
    Eq,
    NotEq,
    #[default]
    NotSet,
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Self {
        Self {
            token_type,
            literal: literal.to_string(),
        }
    }
}
