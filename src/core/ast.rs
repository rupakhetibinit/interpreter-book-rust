use super::{
    lexer::Lexer,
    token::{Token, TokenType},
};
#[allow(unused, dead_code)]
use core::str;
use std::fmt::{self, format, write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = match self {
            Node::Expression(expression) => writeln!(f, "x"),
            Node::Program(program) => {
                for statement in &program.statements {
                    writeln!(f, "{}", statement)?;
                }
                Ok(())
            }
            Node::Statement(statement) => writeln!(f, "x"),
        };

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    LetStatement { token: Token, name: Identifier },
    ReturnStatement { token: Token, value: Expression },
    ExpressionStatement(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::LetStatement { token, name } => {
                write!(f, "Token :  {} Name:  {}", token, name)
            }
            Statement::ReturnStatement { token, value } => {
                write!(f, "Token: {}  Value: {}", token, value)
            }
            Statement::ExpressionStatement { .. } => write!(f, "Expression",),
        }
    }
}

impl Statement {
    fn token_literal(&self) -> &str {
        match self {
            Statement::LetStatement { token, .. } => &token.literal,
            Statement::ReturnStatement { token, .. } => &token.literal,
            Statement::ExpressionStatement(expression) => expression.token_literal(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    IntegerLiteral {
        token: Token,
        value: i64,
    },
    PrefixExpression {
        token: Token,
        operator: String,
        right: Box<Option<Expression>>,
    },
    Identifier(Identifier),
    NONE,
}

impl Expression {
    pub fn token_literal(&self) -> &str {
        let formatted_string = match self {
            Expression::IntegerLiteral { token, value: _ } => &token.literal,
            Expression::PrefixExpression {
                token,
                operator: _,
                right: _,
            } => token.literal.as_str(),
            Expression::Identifier(identifier) => identifier.token_literal(),
            Expression::NONE => "",
        };

        formatted_string
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

    pub fn token_literal(&self) -> &str {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            ""
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Let {
    token: Token,
    name: Identifier,
    value: Identifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    token: Token,
    value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token : {} Value : {}", self.token, self.value)
    }
}

impl Identifier {
    pub fn token_literal(&self) -> &str {
        &self.token.literal
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

        let stmt = Statement::ReturnStatement {
            token: self.curr_token.clone(),
            value: Expression::NONE,
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

        let stmt = Statement::LetStatement {
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
        return self.curr_token.token_type == token_type;
    }

    pub fn peek_token_is(&self, token_type: TokenType) -> bool {
        return self.peek_token.token_type == token_type;
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
            _ => None,
        }
    }

    pub fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();
        let operator = self.curr_token.literal.clone();

        self.next_token();

        let right = self.parse_expression_w_precedence(Precedence::PREFIX);

        Some(Expression::PrefixExpression {
            token,
            operator,
            right: Box::new(right),
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
        let expression = self.parse_expression_w_precedence(Precedence::LOWEST)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::ExpressionStatement(expression))
    }

    pub fn parse_expression_w_precedence(&mut self, prededence: Precedence) -> Option<Expression> {
        self.parse_expression_prefix()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Precedence {
    LOWEST = 1,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

#[cfg(test)]
mod tests {
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
            if let Statement::LetStatement { token: _, name } = stmt {
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
            if let Statement::ReturnStatement { token, .. } = stmt {
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
            if let Statement::ExpressionStatement(expression) = stmt {
                if let Expression::PrefixExpression {
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
}
