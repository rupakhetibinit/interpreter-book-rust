use crate::{Lexer, Parser, eval::eval::eval};

#[test]
pub fn test_int_eval() {
    let input = ["5;", "10"];
    let expected = [5, 10];

    let lexer = Lexer::new(input.join("").to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    assert!(program.is_some());
    assert!(parser.errors.is_empty());

    for (stmt, expected) in program.unwrap().statements.iter().zip(expected) {
        let actual = eval(stmt.clone());
        assert_eq!(format!("{}", actual), format!("{}", expected))
    }
}
