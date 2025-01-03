use super::{Lexer, TokenType};

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

    let mut lexer = Lexer::new(input);

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
