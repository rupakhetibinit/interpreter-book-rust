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

#[test]
pub fn test_bool_eval() {
    let input = ["true;", "false"];
    let expected = [true, false];

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

#[test]
pub fn test_prefix_operator_eval() {
    let input = ["!true", "!false", "!5", "!!true", "!!false", "!!5"];
    let expected = [false, true, false, true, false, true];

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

#[test]
pub fn test_minus_operator() {
    let input = ["-1", "-14", "-12341234", "12341234", "-124234", "1234124"];
    let expected = [-1, -14, -12341234, 12341234, -124234, 1234124];

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

#[test]
pub fn test_infix_operators() {
    let inputs_expected = [
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10)", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        (" - 50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for (input, expected) in inputs_expected {
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
