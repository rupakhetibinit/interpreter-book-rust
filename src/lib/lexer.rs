use crate::lib::token::{Token, TokenType};

use super::token;

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
            read_position: 1,
            ch: '\0',
        };

        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position > self.input.len() {
            self.ch = '\0'
        } else {
            self.ch = self.input.chars().nth(self.position).unwrap_or('\0')
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '=' => Token::new(TokenType::Assign, "="),
            '{' => Token::new(TokenType::LBrace, "{"),
            '}' => Token::new(TokenType::RBrace, "}"),
            '+' => Token::new(TokenType::Plus, "+"),
            ',' => Token::new(TokenType::Comma, ","),
            ';' => Token::new(TokenType::Semicolon, ";"),
            '(' => Token::new(TokenType::LParen, "("),
            ')' => Token::new(TokenType::RParen, ")"),
            _ => Token::new(TokenType::Eof, ""),
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::token::TokenType;

    use super::Lexer;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = vec![
            (TokenType::Assign, "="),
            (TokenType::Plus, "+"),
            (TokenType::LParen, "("),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::RBrace, "}"),
            (TokenType::Comma, ","),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input.to_owned());

        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let token = lexer.next_token();
            println!("{:?}", token);
            assert_eq!(
                &token.token_type, expected_type,
                "tests[{}] - token type wrong. expected={:?}, got={:?}",
                i, expected_type, token
            );
            assert_eq!(
                &token.literal, expected_literal,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, expected_literal, token.literal
            );
        }
    }
}
