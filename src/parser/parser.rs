use std::borrow::Cow;

use crate::{
    ast::ast::{Expression, Identifier, Program, Statement},
    lexer::{
        lexer::Lexer,
        precedence::Precedence,
        token::{Token, TokenType},
    },
};

#[derive(Debug, Clone)]
pub struct Parser {
    pub lexer: Lexer,
    pub curr_token: Token,
    pub peek_token: Token,
    pub errors: Vec<String>,
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
        // if !self.expect_peek(TokenType::Int) {
        //     return None;
        // }

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
                value: Cow::Owned(self.curr_token.literal.clone()),
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

    pub fn parse_function_literal(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        let parameters = self.parse_function_parameters()?;

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = Box::new(self.parse_block_statement());

        return Some(Expression::Function {
            token,
            parameters,
            body,
        });
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
            TokenType::Plus | TokenType::Minus | TokenType::Bang => self.parse_prefix_expression(),
            TokenType::True | TokenType::False => self.parse_boolean_expression(),
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::If => self.parse_if_expression(),
            TokenType::Function => self.parse_function_literal(),
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
            TokenType::LParen => self.parse_call_expression(expression),
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
            value: Cow::Owned(self.curr_token.literal.clone()),
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

        Some(Expression::Integer {
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
            TokenType::LParen => Precedence::Call,
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

        return Some(Expression::If {
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

    fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut identifiers: Vec<Identifier> = vec![];

        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        identifiers.push(Identifier {
            token: self.curr_token.clone(),
            value: Cow::Owned(self.curr_token.literal.clone()),
        });

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            identifiers.push(Identifier {
                token: self.curr_token.clone(),
                value: Cow::Owned(self.curr_token.literal.clone()),
            });
        }

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_call_expression(&mut self, expression: Expression) -> Option<Expression> {
        let token = self.curr_token.clone();
        let function = Box::new(expression);

        let arguments = self.parse_call_arguments();

        Some(Expression::Call {
            token,
            function,
            arguments,
        })
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Expression>> {
        let mut args: Vec<Expression> = vec![];

        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return None;
        }

        self.next_token();

        args.push(self.parse_expression_w_precedence(Precedence::Lowest)?);

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression_w_precedence(Precedence::Lowest)?);
        }

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        Some(args)
    }

    fn parse_boolean_expression(&self) -> Option<Expression> {
        return Some(Expression::Boolean {
            token: self.curr_token.clone(),
            value: self.curr_token_is(TokenType::True),
        });
    }
}
