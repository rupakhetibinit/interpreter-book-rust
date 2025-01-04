use crate::{Lexer, Parser, eval::eval::eval_program};

#[test]
pub fn test_int_eval() {
    let input = ["5;", "10"];
    let expected = [5, 10];

    for (input, expected) in input.iter().zip(expected) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert!(program.is_some());
        assert!(parser.errors.is_empty());

        let mut program = program.unwrap();
        let actual = eval_program(&mut program);

        assert_eq!(format!("{}", actual), format!("{}", expected))
    }
}
