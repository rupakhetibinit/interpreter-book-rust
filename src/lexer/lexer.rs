use std::{iter::Peekable, str::Chars};

use super::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Lexer<'input> {
    input: &'input str,
    peekable: Peekable<Chars<'input>>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer {
            input,
            position: 0,
            peekable: input.chars().peekable(),
        };

        lexer
    }

    pub fn next_char(&mut self) -> Option<char> {
        match self.peekable.next() {
            Some(x) => {
                self.position += x.len_utf8();
                Some(x)
            }
            None => None,
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        let token = match self.next_char() {
            Some('=') => {
                if self.peekable.peek() == Some(&'=') {
                    self.next_char();
                    Token::new(TokenType::Eq, "==")
                } else {
                    Token::new(TokenType::Assign, "=")
                }
            }
            Some('{') => Token::new(TokenType::LBrace, "{"),
            Some('}') => Token::new(TokenType::RBrace, "}"),
            Some('+') => Token::new(TokenType::Plus, "+"),
            Some('-') => Token::new(TokenType::Minus, "-"),
            Some('<') => Token::new(TokenType::Lt, "<"),
            Some('>') => Token::new(TokenType::Gt, ">"),
            Some('!') => {
                if self.peekable.peek() == Some(&'=') {
                    self.next_char();
                    Token::new(TokenType::NotEq, "!=")
                } else {
                    Token::new(TokenType::Bang, "!")
                }
            }
            Some('*') => Token::new(TokenType::Asterisk, "*"),
            Some('/') => Token::new(TokenType::Slash, "/"),
            Some(',') => Token::new(TokenType::Comma, ","),
            Some(';') => Token::new(TokenType::Semicolon, ";"),
            Some('(') => Token::new(TokenType::LParen, "("),
            Some(')') => Token::new(TokenType::RParen, ")"),
            Some(c) if c.is_alphabetic() || c == '_' => {
                let literal = self.read_identifier(c);
                let tok_type = self.lookup_identifier(literal);
                Token::new(tok_type, literal)
            }
            Some(c) if c.is_ascii_digit() => {
                let literal = self.read_number(c);
                Token::new(TokenType::Int, literal)
            }
            Some(_) => Token::new(TokenType::Illegal, "Illegal"),
            None => Token::new(TokenType::Eof, ""),
        };
        token
    }

    fn read_identifier(&mut self, first: char) -> &'a str {
        let start = self.position - first.len_utf8();
        while let Some(&ch) = self.peekable.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                self.next_char();
            } else {
                break;
            }
        }
        &self.input[start..self.position]
    }

    fn read_number(&mut self, first: char) -> &'a str {
        let start = self.position - first.len_utf8();
        while let Some(&ch) = self.peekable.peek() {
            if ch.is_numeric() {
                self.next_char();
            } else {
                break;
            }
        }
        &self.input[start..self.position]
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peekable.peek() {
            if ch.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn lookup_identifier(&self, ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "else" => TokenType::Else,
            "if" => TokenType::If,
            "return" => TokenType::Return,
            _ => TokenType::Ident,
        }
    }
}
