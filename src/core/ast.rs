use super::{
    lexer::Lexer,
    precedence::Precedence,
    token::{Token, TokenType},
};
#[allow(unused, dead_code)]
use core::str;
use std::{
    fmt::{self},
    ops::Deref,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = match self {
            Node::Expression(expression) => writeln!(f, "{}", expression.token_literal()),
            Node::Program(program) => {
                for statement in &program.statements {
                    writeln!(f, "{}", statement)?;
                }
                Ok(())
            }
            Node::Statement(statement) => writeln!(f, "{}", statement.token_literal()),
        };

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Let {
        token: Token,
        name: Identifier,
    },
    Return {
        token: Token,
        value: Expression,
    },
    Block {
        token: Token,
        statements: Vec<Statement>,
    },
    Expression(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let { token, name } => {
                write!(f, "let {} = {};", token.literal, name.value)
            }
            Statement::Return { value, .. } => {
                write!(f, "return {};", value.token_literal())
            }
            Statement::Expression(expression) => write!(f, "{}", expression),
            Statement::Block { statements, .. } => {
                for stmt in statements {
                    _ = write!(f, "{}", stmt);
                }
                Ok(())
            }
        }
    }
}

impl Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let { token, .. } => token.literal.clone(),
            Statement::Return { token, .. } => token.literal.clone(),
            Statement::Expression(expression) => expression.token_literal(),
            Statement::Block { token, .. } => token.literal.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    IntegerLiteral {
        token: Token,
        value: i64,
    },
    Prefix {
        token: Token,
        operator: String,
        right: Box<Option<Expression>>,
    },
    Infix {
        token: Token,
        operator: String,
        right: Box<Expression>,
        left: Box<Expression>,
    },
    IfExpression {
        token: Token,
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    Identifier(Identifier),
    None,
}

impl Expression {
    pub fn token_literal(&self) -> String {
        match self {
            Expression::IntegerLiteral { token, .. } => token.literal.clone(),
            Expression::Prefix {
                operator, right, ..
            } => match right.as_ref() {
                Some(expression) => format!("({} {})", operator, expression.token_literal()),
                None => format!("({}{})", operator, "None"),
            },
            Expression::Identifier(identifier) => identifier.value.clone(),
            Expression::None => String::from(""),
            Expression::Infix {
                left,
                operator,
                right,
                ..
            } => {
                format!(
                    "({} {} {})",
                    left.token_literal(),
                    operator,
                    right.token_literal()
                )
            }
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
                ..
            } => match alternative {
                Some(x) => {
                    format!(
                        "if {} {} else {}",
                        condition.to_string(),
                        consequence.to_string(),
                        x.to_string(),
                    )
                }
                None => format!(
                    "if {} {{ {} }}",
                    condition.to_string(),
                    consequence.to_string(),
                ),
            },
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token_literal())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    token: Token,
    value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token_literal())
    }
}

impl Identifier {
    pub fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
    lexer: Lexer,
    curr_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            lexer,
            curr_token: Token::default(),
            peek_token: Token::default(),
            errors: vec![],
        };

        // Read 2 tokens so curr_token and next_token are both set
        p.next_token();
        p.next_token();

        p
    }

    pub fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program::new();

        while !self.curr_token_is(TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        Some(program)
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::Int) {
            return None;
        }

        let stmt = Statement::Return {
            token: self.curr_token.clone(),
            value: Expression::None,
        };

        while !self.curr_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(stmt)
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let stmt = Statement::Let {
            name: Identifier {
                token: self.curr_token.clone(),
                value: self.curr_token.literal.clone(),
            },
            token: self.curr_token.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        while !self.curr_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(stmt)
    }

    pub fn curr_token_is(&self, token_type: TokenType) -> bool {
        self.curr_token.token_type == token_type
    }

    pub fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type.clone()) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    pub fn peek_error(&mut self, token: TokenType) {
        let message = format!(
            "expected next token to be {:?}, got {:?} instead",
            token, self.peek_token.token_type
        );
        self.errors.push(message);
    }

    pub fn parse_expression_prefix(&mut self) -> Option<Expression> {
        match self.curr_token.token_type {
            TokenType::Ident => Some(self.parse_identifier()),
            TokenType::Int => self.parse_integer_literal(),
            TokenType::Plus | TokenType::Minus => self.parse_prefix_expression(),
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::If => self.parse_if_expression(),
            _ => None,
        }
    }

    pub fn parse_expression_infix(&mut self, expression: Expression) -> Option<Expression> {
        match self.curr_token.token_type {
            TokenType::Eq
            | TokenType::NotEq
            | TokenType::Minus
            | TokenType::Plus
            | TokenType::Asterisk
            | TokenType::Slash
            | TokenType::Gt
            | TokenType::Lt => self.parse_infix_expression(expression),
            _ => None,
        }
    }

    pub fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();
        let operator = self.curr_token.literal.clone();

        self.next_token();

        let right = self.parse_expression_w_precedence(Precedence::Prefix);

        Some(Expression::Prefix {
            token,
            operator,
            right: Box::new(right),
        })
    }

    pub fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let token = self.curr_token.clone();
        let operator = self.curr_token.literal.clone();
        let left = Box::new(left);
        let precedence = self.curr_precedence();
        self.next_token();
        let right = Box::new(self.parse_expression_w_precedence(precedence)?);

        Some(Expression::Infix {
            token,
            operator,
            right,
            left,
        })
    }

    pub fn parse_identifier(&self) -> Expression {
        Expression::Identifier(Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        })
    }

    pub fn parse_integer_literal(&mut self) -> Option<Expression> {
        let literal = self.curr_token.literal.parse::<i64>().ok().or_else(|| {
            self.errors.push(format!(
                "Could not parse {} as integer",
                self.curr_token.literal
            ));
            None
        })?;

        Some(Expression::IntegerLiteral {
            token: self.curr_token.clone(),
            value: literal,
        })
    }

    pub fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression_w_precedence(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Expression(expression))
    }

    pub fn parse_expression_w_precedence(&mut self, prededence: Precedence) -> Option<Expression> {
        let mut left = self.parse_expression_prefix()?;

        while !self.peek_token_is(TokenType::Semicolon) && prededence < self.peek_precedence() {
            self.next_token();
            if let Some(infix) = self.parse_expression_infix(left.clone()) {
                left = infix
            }
        }
        Some(left)
    }

    fn get_precedence_of_token(token_type: TokenType) -> Precedence {
        match token_type {
            TokenType::Eq | TokenType::NotEq => Precedence::Equals,
            TokenType::Lt | TokenType::Gt => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Asterisk | TokenType::Slash => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    pub fn peek_precedence(&self) -> Precedence {
        Self::get_precedence_of_token(self.peek_token.token_type.clone())
    }

    pub fn curr_precedence(&self) -> Precedence {
        Self::get_precedence_of_token(self.curr_token.token_type.clone())
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let expression = self.parse_expression_w_precedence(Precedence::Lowest)?;

        if self.expect_peek(TokenType::RParen) {
            Some(expression)
        } else {
            None
        }
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        self.next_token();

        let condition = Box::new(self.parse_expression_w_precedence(Precedence::Lowest)?);

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let consequence = Box::new(self.parse_block_statement());

        return Some(Expression::IfExpression {
            token,
            condition,
            consequence,
            alternative: None,
        });
    }

    fn parse_block_statement(&mut self) -> Statement {
        let mut statements: Vec<Statement> = Vec::new();
        let token = self.curr_token.clone();

        while !self.curr_token_is(TokenType::RBrace) && !self.curr_token_is(TokenType::Eof) {
            let stmt = self.parse_statement();
            match stmt {
                Some(x) => statements.push(x),
                None => {}
            };
            self.next_token();
        }

        return Statement::Block { token, statements };
    }
}

#[cfg(test)]
mod tests {

    use core::panic;

    use super::*;
    use crate::core::ast::Statement;
    use crate::core::lexer::Lexer;

    #[test]
    fn test_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

        let lexer = Lexer::new(input.to_owned());
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
            if let Statement::Let { token: _, name } = stmt {
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

        let lexer = Lexer::new(input.to_owned());
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
        let lexer = Lexer::new(input.to_owned());
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
        let lexer = Lexer::new(input.to_owned());
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
        let lexer = Lexer::new(input.to_owned());
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
        let lexer = Lexer::new(input.to_owned());
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
            "(5 + 7)", "(5 - 7)", "(5 * 7)", "(5 / 7)", "(5 > 7)", "(5 < 7)", "(5 == 7)",
            "(5 != 7)",
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

        let lexer = Lexer::new(input.to_owned());
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

        let lexer = Lexer::new(input.to_owned());
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

        let lexer = Lexer::new(input.to_owned());
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

    // TODO
    // #[test]
    // fn test_let_statements_print() {
    //     let input = ["let x = 2;", "let y = something;", "let x = 2 + 2;"];
    //     let expected = ["let x = 2;", "let y = something;", "let x = (2 + 2);"];

    //     let lexer = Lexer::new(input.concat());
    //     let mut parser = Parser::new(lexer);
    //     let program = parser.parse_program();
    //     assert!(program.is_some());
    //     assert!(parser.errors().is_empty(), "Errors while parsing");

    //     for (stmt, expect) in program.unwrap().statements.iter().zip(expected) {
    //         dbg!(stmt);
    //         assert_eq!(stmt.to_string(), expect);
    //     }
    // }
}
