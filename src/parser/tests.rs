use crate::{
    ast::ast::{Expression, Statement},
    lexer::lexer::Lexer,
    parser::parser::Parser,
};

#[test]
fn test_let_statements() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser
        .parse_program()
        .expect("parse_program() returned None");

    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    let expected_identifiers = ["x", "y", "foobar"];

    for (stmt, &expected_identifier) in program.statements.iter().zip(&expected_identifiers) {
        if let Statement::Let { name, .. } = stmt {
            assert_eq!(
                name.value, expected_identifier,
                "let_stmt.name.value not '{}'. got={}",
                expected_identifier, name.value
            );
            assert_eq!(
                name.token_literal(),
                expected_identifier,
                "let_stmt.name.token_literal() not '{}'. got={}",
                expected_identifier,
                name.token_literal()
            );
        } else {
            panic!("stmt not a LetStatement. got={:?}", stmt);
        }
    }
}

#[test]
fn test_return_statements() {
    let input = "
            return 5;
            return 121341;
            return 234124;
                ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser
        .parse_program()
        .expect("parse_program() returned None");

    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 1 statements. got={}",
        program.statements.len()
    );

    for stmt in program.statements.iter() {
        if let Statement::Return { token, .. } = stmt {
            assert_eq!(
                token.literal,
                stmt.token_literal(),
                "let_stmt.name.value not '{}'. got={}",
                token.literal,
                stmt.token_literal(),
            );
        } else {
            panic!("stmt not a LetStatement. got={:?}", stmt);
        }
    }
}

#[test]
fn test_prefix_expression() {
    let input = "something;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.is_some());
    assert!(parser.errors().is_empty(), "Errors while parsing");
    assert_eq!(
        program.clone().unwrap().statements.len(),
        1,
        "Program statements should have 1 length"
    );
    let program = program.unwrap();

    assert_eq!(program.statements[0].token_literal(), "something")
}

#[test]
fn test_prefix_expression_integer_literal() {
    let input = "5;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.is_some());
    assert!(parser.errors().is_empty(), "Errors while parsing");
    assert_eq!(
        program.clone().unwrap().statements.len(),
        1,
        "Program statements should have 1 length"
    );
    let program = program.unwrap();

    assert_eq!(program.statements[0].token_literal(), "5")
}

#[test]
fn test_prefix_plus_and_minus() {
    let input = "+5;
        -20;
        ";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.is_some());
    assert!(parser.errors().is_empty(), "Errors while parsing");
    assert_eq!(
        program.clone().unwrap().statements.len(),
        2,
        "Program statements should have 1 length"
    );
    let program = program.unwrap();
    let expected = [("+", "5"), ("-", "20")];

    for (stmt, &expected_identifier) in program.statements.iter().zip(&expected) {
        if let Statement::Expression(expression) = stmt {
            if let Expression::Prefix {
                token,
                operator,
                right,
            } = expression
            {
                println!("{}, {}, {:?}", token, operator, right);
                assert_eq!(token.literal, expected_identifier.0);
                assert_eq!(operator, expected_identifier.0);
                assert_eq!(
                    right.clone().unwrap().token_literal(),
                    expected_identifier.1
                );
            }
        } else {
            panic!("stmt not a LetStatement. got={:?}", stmt);
        }
    }
}

#[test]
fn test_infix_expressions() {
    let input = "
                5 + 7;
                5 - 7;
                5 * 7;
                5 / 7;
                5 > 7;
                5 < 7;
                5 == 7;
                5 != 7;
                ";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.is_some());
    assert!(parser.errors().is_empty(), "Errors while parsing");
    assert_eq!(
        program.clone().unwrap().statements.len(),
        8,
        "Program statements should have 8 length"
    );
    let program = program.unwrap();
    let expected = [
        "(5 + 7)", "(5 - 7)", "(5 * 7)", "(5 / 7)", "(5 > 7)", "(5 < 7)", "(5 == 7)", "(5 != 7)",
    ];

    for (stmt, &expected_identifier) in program.statements.iter().zip(&expected) {
        if let Statement::Expression(expression) = stmt {
            if let Expression::Infix { .. } = expression {
                assert_eq!(
                    expression.to_string().as_str(),
                    expected_identifier.to_owned()
                )
            } else {
                panic!("no infix expressions");
            }
        } else {
            panic!("stmt not a LetStatement. got={:?}", stmt);
        }
    }
}

#[test]
fn test_operator_precedence() {
    let input = "
                5 * 5 * 2 + 10 * 5 - 2;
                ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.is_some());
    assert!(parser.errors().is_empty(), "Errors while parsing");
    assert_eq!(
        program.clone().unwrap().statements.len(),
        1,
        "Program statements should have 1 length"
    );

    let program = program.unwrap();
    if let Statement::Expression(expression) = &program.statements[0] {
        assert_eq!(expression.to_string(), "((((5 * 5) * 2) + (10 * 5)) - 2)")
    }
}

#[test]
fn test_operator_precedence_with_braces() {
    let input = "
                (5 * 5 * 2 + (10 / 2)) + (10 * 5 - 2);
                ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.is_some());
    assert!(parser.errors().is_empty(), "Errors while parsing");
    assert_eq!(
        program.clone().unwrap().statements.len(),
        1,
        "Program statements should have 1 length"
    );

    let program = program.unwrap();
    if let Statement::Expression(expression) = &program.statements[0] {
        assert_eq!(
            expression.to_string(),
            "((((5 * 5) * 2) + (10 / 2)) + ((10 * 5) - 2))"
        )
    }
}

#[test]
fn test_parse_if_expression() {
    let input = "
            if (x < 5) { 2 + 2; }
                ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.is_some());
    assert!(parser.errors().is_empty(), "Errors while parsing");
    assert_eq!(
        program.clone().unwrap().statements.len(),
        1,
        "Program statements should have 1 length"
    );

    let program = program.unwrap();
    match &program.statements[0] {
        expression => {
            assert_eq!(expression.to_string(), "if (x < 5) { (2 + 2) }")
        }
    }
}

#[test]
fn test_function_literal() {
    let input = ["fn (x , y) { x + y; }", "fn (x) {}"];
    let expected = ["fn (x , y) { (x + y) }", "fn (x) {  }"];
    let input_string = input.join("\n");

    let binding = input_string.to_owned();
    let lexer = Lexer::new(&binding);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(program.is_some());
    assert!(parser.errors().is_empty(), "Errors while parsing");
    assert_eq!(
        program.clone().unwrap().statements.len(),
        2,
        "Program statements should have 2 length"
    );

    let program = program.unwrap();
    for (statement, expected_output) in program.statements.iter().zip(&expected) {
        match statement {
            expression => {
                assert_eq!(expression.to_string(), expected_output.to_string())
            }
        }
    }
}

#[test]
fn test_function_call() {
    let input = [
        "a + add(b * c) + d",
        "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
        "add(a + b + c * d / f + g)",
    ];
    let expected = [
        "((a + add((b * c))) + d)",
        "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        "add((((a + b) + ((c * d) / f)) + g))",
    ];
    let input_string = input.join("\n");

    let binding = input_string.to_owned();
    let lexer = Lexer::new(&binding);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert!(program.is_some());
    assert!(parser.errors().is_empty(), "Errors while parsing");
    dbg!("{}", parser.errors().clone());
    assert_eq!(
        program.clone().unwrap().statements.len(),
        3,
        "Program statements should have 2 length"
    );

    let program = program.unwrap();
    for (statement, expected_output) in program.statements.iter().zip(&expected) {
        match statement {
            expression => {
                assert_eq!(expression.to_string(), expected_output.to_string())
            }
        }
    }
}

#[test]
fn test_boolean_expression() {
    let input = [
        "true",
        "false",
        "3 < 5 == true",
        "3 > 5 == false",
        "true == true",
        "false == false",
        "true != false",
        "!true",
        "!false",
    ];
    let expected = [
        "true",
        "false",
        "((3 < 5) == true)",
        "((3 > 5) == false)",
        "(true == true)",
        "(false == false)",
        "(true != false)",
        "!true",
        "!false",
    ];
    let input_string = input.join("\n");

    let binding = input_string.to_owned();
    let lexer = Lexer::new(&binding);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert!(program.is_some());
    assert!(parser.errors().is_empty(), "Errors while parsing");
    dbg!("{}", parser.errors().clone());
    assert_eq!(
        program.clone().unwrap().statements.len(),
        input.len(),
        "Program statements did not match"
    );

    let program = program.unwrap();
    for (statement, expected_output) in program.statements.iter().zip(&expected) {
        match statement {
            expression => {
                assert_eq!(expression.to_string(), expected_output.to_string())
            }
        }
    }
}

#[test]
fn test_boolean_precedence_expression() {
    let input = [
        "1 + (2 + 3) + 4",
        "(5 + 5) * 2",
        "2 / (5 + 5)",
        "-(5 + 5)",
        "!(true == true)",
    ];
    let expected = [
        "((1 + (2 + 3)) + 4)",
        "((5 + 5) * 2)",
        "(2 / (5 + 5))",
        "-(5 + 5)",
        "!(true == true)",
    ];
    let input_string = input.join(";");

    let binding = input_string.to_owned();
    let lexer = Lexer::new(&binding);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert!(program.is_some());
    assert!(parser.errors().is_empty(), "Errors while parsing");
    dbg!("{}", parser.errors().clone());
    assert_eq!(
        program.clone().unwrap().statements.len(),
        input.len(),
        "Program statements did not match"
    );

    let program = program.unwrap();
    for (statement, expected_output) in program.statements.iter().zip(&expected) {
        match statement {
            expression => {
                assert_eq!(expression.to_string(), expected_output.to_string())
            }
        }
    }
}

#[test]
fn test_let_and_return_statements() {
    let input = [
        "return x + 2;",
        "let x = 2",
        "let y = false",
        "return true",
        "return 1 / 2",
    ];
    let expected = [
        "return (x + 2);",
        "let x = 2;",
        "let y = false;",
        "return true;",
        "return (1 / 2);",
    ];
    let input_string = input.join(";");

    let binding = input_string.to_owned();
    let lexer = Lexer::new(&binding);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert!(program.is_some());
    assert!(parser.errors().is_empty(), "Errors while parsing");
    dbg!("{}", parser.errors().clone());
    assert_eq!(
        program.clone().unwrap().statements.len(),
        input.len(),
        "Program statements did not match"
    );

    let program = program.unwrap();
    for (statement, expected_output) in program.statements.iter().zip(&expected) {
        match statement {
            expression => {
                assert_eq!(expression.to_string(), expected_output.to_string())
            }
        }
    }
}
