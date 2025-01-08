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

#[test]
pub fn test_if_expression() {
    let inputs_expected = [
        ("if (true) { 10 }", "10"),
        ("if (false) { 10 }", "nil"),
        ("if (1) { 10 }", "10"),
        ("if (1 < 2) { 10 }", "10"),
        ("if (1 > 2) { 10 }", "nil"),
        ("if (1 > 2) { 10 } else { 20 }", "20"),
        ("if (1 < 2) { 10 } else { 20 }", "10"),
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

#[test]
fn test_return_statements() {
    let inputs_expected = [
        ("return 10;", 10),
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
        (
            r#"if (10 > 1) {
        if (10 > 1) {
        return 10;
        }
        129
        return 1;
        }"#,
            10,
        ),
    ];

    for (input, expected) in inputs_expected {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert!(program.is_some());
        assert!(parser.errors.is_empty());

        let mut program = program.unwrap();
        dbg!(program.clone());
        let actual = eval_program(&mut program);

        assert_eq!(format!("{}", actual), format!("{}", expected))
    }
}

#[test]
fn test_early_returns() {
    let inputs_expected = [
        (
            r#"if (10 > 1) {
                    if (10 > 1) {
                        return 10;
                    }
                    129;
                    return 1;
                }"#,
            10,
        ),
        (
            r#"
                if (5 > 2) {
                    if (10 > 1) {
                        if (3 > 1) {
                            return 15;
                        }
                        return 10;
                    }
                    return 5;
                }
                return 0;
                "#,
            15,
        ),
        (
            r#"
                if (5 < 2) {
                    return 1;
                } else {
                    if (10 > 5) {
                        return 2;
                    } else {
                        return 3;
                    }
                    return 4;  // This should never be reached
                }
                return 5;  // This should never be reached
                "#,
            2,
        ),
    ];

    for (input, expected) in inputs_expected {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert!(program.is_some(), "Failed to parse program: {}", input);
        assert!(
            parser.errors.is_empty(),
            "Parser errors for input: {}",
            input
        );

        let mut program = program.unwrap();
        let actual = eval_program(&mut program);

        assert_eq!(
            format!("{}", actual),
            format!("{}", expected),
            "Test failed for input:\n{}\nExpected {}, got {}",
            input,
            expected,
            actual
        );
    }
}
