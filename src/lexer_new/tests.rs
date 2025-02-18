use logos::Logos;

use super::Token;

#[test]
pub fn test_new_lexer() {
    let input = "let five = 5;
    let ten = 10;
    let array = [1,2,3,4,5,,];
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

    let lexer = Token::lexer(input);
    for tokens in lexer.into_iter() {
        println!("{}", tokens.unwrap());
    }
}
