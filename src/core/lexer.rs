use crate::core::token::{Token, TokenType};

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
            ch: '\0',
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
            '=' => Token::new(TokenType::Assign, "="),
            '{' => Token::new(TokenType::LBrace, "{"),
            '}' => Token::new(TokenType::RBrace, "}"),
            '+' => Token::new(TokenType::Plus, "+"),
            ',' => Token::new(TokenType::Comma, ","),
            ';' => Token::new(TokenType::Semicolon, ";"),
            '(' => Token::new(TokenType::LParen, "("),
            ')' => Token::new(TokenType::RParen, ")"),
            '\0' => Token::new(TokenType::Eof, ""),
            c if c.is_alphabetic() => {
                let literal = self.read_identifier();
                let token_type = self.lookup_identifier(&literal);
                return Token::new(token_type, &literal);
            }
            c if c.is_ascii_digit() => {
                let literal = self.read_number();
                return Token::new(TokenType::Int, &literal);
            }
            _ => Token::new(
                TokenType::Illegal,
                &self.input[self.position..=self.position],
            ),
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let start_position = self.position;
        while self.ch.is_alphabetic() {
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
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn lookup_identifier(&self, ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            _ => TokenType::Ident,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::token::TokenType;

    use super::Lexer;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
";
        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
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
