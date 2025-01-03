use std::{iter::Peekable, str::Chars};

use super::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    original_input: &'a str,
    input: Peekable<Chars<'a>>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            original_input: input,
            input: input.chars().peekable(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.input.next() {
            Some('=') => {
                if self.input.peek() == Some(&'=') {
                    self.input.next();
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
                if self.input.peek() == Some(&'=') {
                    self.input.next();
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
            None => Token::new(TokenType::Eof, ""),
            Some(c) if c.is_alphabetic() || c == '_' => {
                let literal = self.read_identifier(c);
                let tok_type = self.lookup_identifier(literal);
                return Token::new(tok_type, &literal);
            }
            Some(c) if c.is_ascii_digit() => {
                let literal = self.read_number(c);
                return Token::new(TokenType::Int, literal);
            }
            Some(_) => Token::new(TokenType::Illegal, "Illegal"),
        };
        token
    }

    fn read_identifier(&mut self, start: char) -> &'a str {
        let start_pos = self.position + start.len_utf8();
        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.position += c.len_utf8();
                self.input.next();
            } else {
                break;
            }
        }
        &self.original_input[start_pos..self.position]
    }

    fn read_number(&mut self, start: char) -> &'a str {
        let start_pos = self.position + start.len_utf8();
        while let Some(&c) = self.input.peek() {
            if c.is_ascii_digit() {
                self.position += c.len_utf8();
                self.input.next();
            } else {
                break;
            }
        }
        &self.original_input[start_pos..self.position]
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
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
