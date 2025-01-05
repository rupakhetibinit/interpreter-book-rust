use crate::{
    ast::ast::{Expression, Identifier, Program, Statement},
    lexer::{
        lexer::Lexer,
        precedence::Precedence,
        token::{Token, TokenType},
    },
};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub curr_token: Token<'a>,
    pub peek_token: Token<'a>,
    pub errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
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
        self.curr_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program<'a>> {
        let mut program = Program::new();

        while !self.curr_token_is(TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        Some(program)
    }

    pub fn parse_statement(&mut self) -> Option<Statement<'a>> {
        match self.curr_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement<'a>> {
        let token = self.curr_token;

        self.next_token();

        let value = self.parse_expression_w_precedence(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Return { token, value })
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement<'a>> {
        let token = self.curr_token;
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = Identifier {
            token: self.curr_token,
            value: self.curr_token.literal,
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        self.next_token();

        let value = self.parse_expression_w_precedence(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let { token, name, value })
    }

    pub fn parse_function_literal(&mut self) -> Option<Expression<'a>> {
        let token = self.curr_token;

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        let parameters = self.parse_function_parameters()?;

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = Box::new(self.parse_block_statement());

        Some(Expression::Function {
            token,
            parameters,
            body,
        })
    }

    pub fn curr_token_is(&self, token_type: TokenType) -> bool {
        self.curr_token.token_type == token_type
    }

    pub fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
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

    pub fn parse_expression_prefix(&mut self) -> Option<Expression<'a>> {
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

    pub fn parse_expression_infix(&mut self, expression: Expression<'a>) -> Option<Expression<'a>> {
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

    pub fn parse_prefix_expression(&mut self) -> Option<Expression<'a>> {
        let token = self.curr_token;
        let operator = self.curr_token.literal;

        self.next_token();

        let right = self.parse_expression_w_precedence(Precedence::Prefix);

        Some(Expression::Prefix {
            token,
            operator,
            right: Box::new(right),
        })
    }

    pub fn parse_infix_expression(&mut self, left: Expression<'a>) -> Option<Expression<'a>> {
        let token = self.curr_token;
        let operator = self.curr_token.literal;
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

    pub fn parse_identifier(&self) -> Expression<'a> {
        Expression::Identifier(Identifier {
            token: self.curr_token,
            value: self.curr_token.literal,
        })
    }

    pub fn parse_integer_literal(&mut self) -> Option<Expression<'a>> {
        let literal = self.curr_token.literal.parse::<i64>().ok().or_else(|| {
            self.errors.push(format!(
                "Could not parse {} as integer",
                self.curr_token.literal
            ));
            None
        })?;

        Some(Expression::Integer {
            token: self.curr_token,
            value: literal,
        })
    }

    pub fn parse_expression_statement(&mut self) -> Option<Statement<'a>> {
        let expression = self.parse_expression_w_precedence(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Expression(expression))
    }

    pub fn parse_expression_w_precedence(
        &mut self,
        prededence: Precedence,
    ) -> Option<Expression<'a>> {
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
        Self::get_precedence_of_token(self.peek_token.token_type)
    }

    pub fn curr_precedence(&self) -> Precedence {
        Self::get_precedence_of_token(self.curr_token.token_type)
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression<'a>> {
        self.next_token();

        let expression = self.parse_expression_w_precedence(Precedence::Lowest)?;

        if self.expect_peek(TokenType::RParen) {
            Some(expression)
        } else {
            None
        }
    }

    fn parse_if_expression(&mut self) -> Option<Expression<'a>> {
        let token = self.curr_token;

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

        Some(Expression::If {
            token,
            condition,
            consequence,
            alternative: None,
        })
    }

    fn parse_block_statement(&mut self) -> Statement<'a> {
        let mut statements: Vec<Statement> = Vec::new();
        let token = self.curr_token;

        while !self.curr_token_is(TokenType::RBrace) && !self.curr_token_is(TokenType::Eof) {
            let stmt = self.parse_statement();
            if let Some(x) = stmt {
                statements.push(x)
            };
            self.next_token();
        }

        Statement::Block { token, statements }
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Identifier<'a>>> {
        let mut identifiers: Vec<Identifier> = vec![];

        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        identifiers.push(Identifier {
            token: self.curr_token,
            value: self.curr_token.literal,
        });

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            identifiers.push(Identifier {
                token: self.curr_token,
                value: self.curr_token.literal,
            });
        }

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_call_expression(&mut self, expression: Expression<'a>) -> Option<Expression<'a>> {
        let token = self.curr_token;
        let function = Box::new(expression);

        let arguments = self.parse_call_arguments();

        Some(Expression::Call {
            token,
            function,
            arguments,
        })
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Expression<'a>>> {
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

    fn parse_boolean_expression(&self) -> Option<Expression<'a>> {
        Some(Expression::Boolean {
            token: self.curr_token,
            value: self.curr_token_is(TokenType::True),
        })
    }
}
