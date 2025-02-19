use std::fmt;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Eq, Hash)]
#[logos(skip r"[ \t\r\n]+")]
#[logos(error = String)]
pub enum Token<'a> {
    #[regex("[a-zA-Z][a-zA-Z0-9]*")]
    Ident(&'a str),
    #[regex("[0-9]+", |lex| lex.slice().parse::<isize>().unwrap())]
    Int(isize),
    #[token("=")]
    Assign,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("!")]
    Bang,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("fn")]
    Function,
    #[token("let")]
    Let,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("else")]
    Else,
    #[token("if")]
    If,
    #[token("return")]
    Return,
    #[token("==")]
    Eq,
    #[token("!=")]
    NotEq,
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Ident(x) => write!(f, "{}", x),
            Token::Int(i) => write!(f, "{}", i),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Else => write!(f, "else"),
            Token::If => write!(f, "if"),
            Token::Return => write!(f, "return"),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::BracketOpen => write!(f, "["),
            Token::BracketClose => write!(f, "]"),
        }
    }
}
