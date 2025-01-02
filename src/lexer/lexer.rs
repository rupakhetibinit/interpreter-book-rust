use crate::arena::arena;

use super::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0 as char,
        };

        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position > self.input.len() {
            self.ch = '\0'
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap_or('\0')
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::Eq, "==")
                } else {
                    Token::new(TokenType::Assign, "=")
                }
            }
            '{' => Token::new(TokenType::LBrace, "{"),
            '}' => Token::new(TokenType::RBrace, "}"),
            '+' => Token::new(TokenType::Plus, "+"),
            '-' => Token::new(TokenType::Minus, "-"),
            '<' => Token::new(TokenType::Lt, "<"),
            '>' => Token::new(TokenType::Gt, ">"),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::NotEq, "!=")
                } else {
                    Token::new(TokenType::Bang, "!")
                }
            }
            '*' => Token::new(TokenType::Asterisk, "*"),
            '/' => Token::new(TokenType::Slash, "/"),
            ',' => Token::new(TokenType::Comma, ","),
            ';' => Token::new(TokenType::Semicolon, ";"),
            '(' => Token::new(TokenType::LParen, "("),
            ')' => Token::new(TokenType::RParen, ")"),
            '\u{0}' => Token::new(TokenType::Eof, ""),
            c if c.is_alphabetic() || c == '_' => {
                let literal = self.read_identifier();
                let tok_type = self.lookup_identifier(&literal);
                return Token::new(tok_type, &literal);
            }
            c if c.is_ascii_digit() => {
                let literal = self.read_number();
                return Token::new(TokenType::Int, &literal);
            }
            _ => Token::new(TokenType::Illegal, &self.ch.to_string()),
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let start_position = self.position;
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        self.input[start_position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let start_position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[start_position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() || self.ch == '\t' || self.ch == '\r' || self.ch == '\n' {
            self.read_char();
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

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            0 as char
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }
}
