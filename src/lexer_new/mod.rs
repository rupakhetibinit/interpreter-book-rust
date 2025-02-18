pub mod ast;
#[cfg(test)]
pub mod tests;

use std::fmt;

use chumsky::combinator::IgnoreThen;
use chumsky::prelude::*;

use chumsky::text::whitespace;
use chumsky::{Parser, error::Simple, select, text, text::Padded};
use logos::Logos;

use ast::{Expression, Identifier, Program, Statement};

use crate::lexer::token;

#[derive(Logos, Debug, PartialEq, Clone, Eq, Hash)]
#[logos(skip r"[ \t\r\n]+")]
#[logos(error = String)]
pub enum Token {
    #[regex("[a-zA-Z][a-zA-Z0-9]*", |lex| lex.slice().to_string())]
    Ident(String),
    #[regex("[0-9]+", |lex| lex.slice().parse::<isize>().unwrap())]
    Int(isize),
    #[token("=")]
    Assign,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("!")]
    Bang,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("fn")]
    Function,
    #[token("let")]
    Let,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("else")]
    Else,
    #[token("if")]
    If,
    #[token("return")]
    Return,
    #[token("==")]
    Eq,
    #[token("!=")]
    NotEq,
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Ident(x) => write!(f, "{}", x),
            Token::Int(i) => write!(f, "{}", i),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Else => write!(f, "else"),
            Token::If => write!(f, "if"),
            Token::Return => write!(f, "return"),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::BracketOpen => write!(f, "["),
            Token::BracketClose => write!(f, "]"),
        }
    }
}

fn expression() -> impl Parser<Token, Expression, Error = Simple<Token>> {
    recursive(|expr| {
        // Helper for creating token from operator string
        let token_from_op = |op: &str| -> Token {
            match op {
                "+" => Token::Plus,
                "-" => Token::Minus,
                "*" => Token::Asterisk,
                "/" => Token::Slash,
                "==" => Token::Eq,
                "!=" => Token::NotEq,
                "<" => Token::Lt,
                ">" => Token::Gt,
                "!" => Token::Bang,
                _ => panic!("Unsupported operator: {}", op),
            }
        };

        // Parse term - the highest precedence expressions (atoms, function calls, etc.)
        let term = {
            // Boolean literals
            let boolean = select! {
                Token::True => Expression::Boolean { token: Token::True, value: true },
                Token::False => Expression::Boolean { token: Token::False, value: false },
            };

            // Identifiers and integers
            let ident_or_int = select! {
                Token::Int(i) => Expression::Integer { token: Token::Int(i), value: i },
                Token::Ident(x) => Expression::Identifier(Identifier { token: Token::Ident(x.clone()), value: x })
            };

            // Parenthesized expressions
            let paren_expr = just(Token::LParen)
                .ignore_then(expr.clone())
                .then_ignore(just(Token::RParen));

            // Function literal: fn(param1, param2) { body }
            let function = just(Token::Function)
                .ignore_then(just(Token::LParen))
                .ignore_then(
                    select! {
                        Token::Ident(name) => Identifier { token: Token::Ident(name.clone()), value: name }
                    }
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                )
                .then_ignore(just(Token::RParen))
                .then(
                    just(Token::LBrace)
                    .ignore_then(statement().repeated())
                    .then_ignore(just(Token::RBrace))
                    .map(|stmts| Statement::Block {
                        token: Token::LBrace,
                        statements: stmts,
                    })
                )
                .map(|(params, body)| Expression::Function {
                    token: Token::Function,
                    parameters: params,
                    body: Box::new(body),
                });

            // If expressions: if (condition) { consequence } else { alternative }
            let if_expr = just(Token::If)
                .ignore_then(just(Token::LParen))
                .ignore_then(expr.clone())
                .then_ignore(just(Token::RParen))
                .then(
                    just(Token::LBrace)
                        .ignore_then(statement().repeated())
                        .then_ignore(just(Token::RBrace))
                        .map(|stmts| Statement::Block {
                            token: Token::LBrace,
                            statements: stmts,
                        }),
                )
                .then(
                    just(Token::Else)
                        .ignore_then(just(Token::LBrace))
                        .ignore_then(statement().repeated())
                        .then_ignore(just(Token::RBrace))
                        .map(|stmts| Statement::Block {
                            token: Token::LBrace,
                            statements: stmts,
                        })
                        .or_not(),
                )
                .map(|((condition, consequence), alternative)| Expression::If {
                    token: Token::If,
                    condition: Box::new(condition),
                    consequence: Box::new(consequence),
                    alternative: alternative.map(Box::new),
                });

            // Combine all basic expressions
            boolean
                .or(ident_or_int)
                .or(paren_expr)
                .or(function)
                .or(if_expr)
        };

        // Prefix expressions (!expr, -expr)
        let prefix = select! {
            Token::Bang => "!",
            Token::Minus => "-",
        }
        .map(move |op| (op, token_from_op(op)))
        .then(expr.clone())
        .map(|((op, token), right)| Expression::Prefix {
            token,
            operator: op.to_string(),
            right: Box::new(Some(right)),
        });

        // Term or prefix expression
        let atom = prefix.or(term);

        // Handle function calls
        let function_calls = atom
            .then(
                just(Token::LParen)
                    .ignore_then(
                        expr.clone()
                            .separated_by(just(Token::Comma))
                            .allow_trailing()
                            .or_not(),
                    )
                    .then_ignore(just(Token::RParen))
                    .repeated(),
            )
            .map(|(initial, call_args)| {
                // Apply each function call to the result of the previous one
                call_args
                    .into_iter()
                    .fold(initial, |acc, args| Expression::Call {
                        token: Token::LParen,
                        function: Box::new(acc),
                        arguments: args.map(|args| args.into_iter().collect()),
                    })
            });

        // Factor: function_calls with multiplication/division
        let factor = function_calls
            .then(
                select! {
                    Token::Asterisk => "*",
                    Token::Slash => "/",
                }
                .map(move |op| (op, token_from_op(op)))
                .then(function_calls)
                .repeated(),
            )
            .map(|(initial, operations)| {
                operations
                    .into_iter()
                    .fold(initial, |acc, ((op, token), right)| Expression::Infix {
                        token,
                        operator: op.to_string(),
                        left: Box::new(acc),
                        right: Box::new(right),
                    })
            });

        // Term: factor with addition/subtraction
        let term = factor
            .then(
                select! {
                    Token::Plus => "+",
                    Token::Minus => "-",
                }
                .map(move |op| (op, token_from_op(op)))
                .then(factor)
                .repeated(),
            )
            .map(|(initial, operations)| {
                operations
                    .into_iter()
                    .fold(initial, |acc, ((op, token), right)| Expression::Infix {
                        token,
                        operator: op.to_string(),
                        left: Box::new(acc),
                        right: Box::new(right),
                    })
            });

        // Comparison: term with comparison operators
        let comparison = term
            .then(
                select! {
                    Token::Eq => "==",
                    Token::NotEq => "!=",
                    Token::Lt => "<",
                    Token::Gt => ">",
                }
                .map(move |op| (op, token_from_op(op)))
                .then(term)
                .repeated(),
            )
            .map(|(initial, operations)| {
                operations
                    .into_iter()
                    .fold(initial, |acc, ((op, token), right)| Expression::Infix {
                        token,
                        operator: op.to_string(),
                        left: Box::new(acc),
                        right: Box::new(right),
                    })
            });

        comparison
    })
}

// Helper to convert string operators to token types
impl Token {
    fn from(op: &str) -> Self {
        match op {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Asterisk,
            "/" => Token::Slash,
            "==" => Token::Eq,
            "!=" => Token::NotEq,
            "<" => Token::Lt,
            ">" => Token::Gt,
            "!" => Token::Bang,
            _ => panic!("Unsupported operator: {}", op),
        }
    }
}

fn atom() -> impl Parser<Token, Expression, Error = Simple<Token>> {
    let int = select! {
        Token::Int(i) => Expression::Integer { token: Token::Int(i), value: i }
    };

    let ident = select! {
        Token::Ident(x) => Expression::Identifier(Identifier { token: Token::Ident(x.clone()), value: x })
    };

    // Function literal: fn(param1, param2) { body }
    let function = just(Token::Function)
        .ignore_then(just(Token::LParen))
        .ignore_then(
            select! {
                Token::Ident(name) => Identifier { token: Token::Ident(name.clone()), value: name }
            }
            .separated_by(just(Token::Comma))
            .allow_trailing(),
        )
        .then_ignore(just(Token::RParen))
        .then(
            just(Token::LBrace)
                .ignore_then(statement().repeated())
                .then_ignore(just(Token::RBrace))
                .map(|stmts| Statement::Block {
                    token: Token::LBrace,
                    statements: stmts,
                }),
        )
        .map(|(params, body)| Expression::Function {
            token: Token::Function,
            parameters: params,
            body: Box::new(body),
        });

    int.or(ident).or(function)
}

fn let_statement() -> impl Parser<Token, Statement, Error = Simple<Token>> {
    just(Token::Let)
        .ignore_then(select! {
            Token::Ident(name) => Identifier {token: Token::Ident(name.clone()),value:name}
        })
        .then_ignore(just(Token::Assign))
        .then(expression())
        .then_ignore(just(Token::Semicolon))
        .map(|(name, value)| Statement::Let {
            token: Token::Let,
            name,
            value,
        })
}

fn return_statement() -> impl Parser<Token, Statement, Error = Simple<Token>> {
    just(Token::Return)
        .ignore_then(expression())
        .map(|expr| Statement::Return {
            token: Token::Return,
            value: expr,
        })
}

fn expression_statement() -> impl Parser<Token, Statement, Error = Simple<Token>> {
    expression()
        .then(just(Token::Semicolon).or_not())
        .map(|(expr, _)| Statement::Expression(expr))
}

fn statement() -> impl Parser<Token, Statement, Error = Simple<Token>> {
    let_statement()
        .or(return_statement())
        .or(expression_statement())
}

pub fn parser() -> impl Parser<Token, Program, Error = Simple<Token>> {
    statement()
        .repeated()
        .then_ignore(end())
        .map(|stmts| Program { statements: stmts })
}
