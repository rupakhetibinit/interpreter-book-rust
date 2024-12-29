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
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    let literal = self.read_identifier();
                    let tok_type = self.lookup_identifier(&literal);
                    return Token::new(tok_type, &literal);
                } else if self.ch.is_ascii_digit() {
                    let literal = self.read_number();
                    return Token::new(TokenType::Int, &literal);
                }

                Token::new(
                    TokenType::Illegal,
                    &self.input[self.position..=self.position],
                )
            }
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
!-/*5;
5 < 10 > 5;
if (5 < 10) {
return true;
} else {
return false;
}
if (x == 2) {
return true;
}";
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
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Gt, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::LParen, "("),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::If, "if"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Eq, "=="),
            (TokenType::Int, "2"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input.to_owned());

        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let token = lexer.next_token();
            // println!("{:?}", token);
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
