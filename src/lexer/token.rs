use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Default, Copy)]
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

#[derive(Debug, Eq, PartialEq, Clone, Default, Copy)]
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

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TokenType : {} Literal : {}",
            self.token_type, self.literal
        )
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            TokenType::Illegal => "Illegal",
            TokenType::Eof => "Eof",
            TokenType::Ident => "Ident",
            TokenType::Int => "Int",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Bang => "!",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",
            TokenType::Lt => "<",
            TokenType::Gt => ">",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",
            TokenType::Function => "function",
            TokenType::Let => "let",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::Else => "else",
            TokenType::If => "if",
            TokenType::Return => "return",
            TokenType::Eq => "==",
            TokenType::NotEq => "!=",
            TokenType::NotSet => "notSet",
        };
        write!(f, "{}", token_str)
    }
}
