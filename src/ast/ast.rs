use core::fmt;

use crate::{StringArena, lexer::token::Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'a> {
    Let {
        token: Token<'a>,
        name: Identifier<'a>,
    },
    Return {
        token: Token<'a>,
        value: Expression<'a>,
    },
    Block {
        token: Token<'a>,
        statements: Vec<Statement<'a>>,
    },
    Expression(Expression<'a>),
}

impl<'a> fmt::Display for Statement<'a> {
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

impl<'a> Statement<'a> {
    pub fn token_literal(&self) -> String {
        match self {
            Statement::Let { token, .. } => token.literal.to_string(),
            Statement::Return { token, .. } => token.literal.to_string(),
            Statement::Expression(expression) => expression.token_literal(),
            Statement::Block { token, .. } => token.literal.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'a> {
    Integer {
        token: Token<'a>,
        value: i64,
    },
    Prefix {
        token: Token<'a>,
        operator: usize,
        right: Box<Option<Expression<'a>>>,
    },
    Infix {
        token: Token<'a>,
        operator: usize,
        right: Box<Expression<'a>>,
        left: Box<Expression<'a>>,
    },
    If {
        token: Token<'a>,
        condition: Box<Expression<'a>>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    Function {
        token: Token<'a>,
        parameters: Vec<Identifier>,
        body: Box<Statement>,
    },
    Call {
        token: Token<'a>,
        function: Box<Expression<'a>>,
        arguments: Option<Vec<Expression<'a>>>,
    },
    Identifier(Identifier),
    None,
}

impl<'a> Expression<'a> {
    pub fn token_literal(&self) -> String {
        match self {
            Expression::Integer { token, .. } => token.literal.to_string(),
            Expression::Prefix {
                operator, right, ..
            } => match right.as_ref() {
                Some(expression) => format!("({} {})", operator, expression.token_literal()),
                None => format!("({}{})", operator, "None"),
            },
            Expression::Identifier(identifier) => identifier.value.to_string(),
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
            Expression::If {
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
            Expression::Function {
                parameters, body, ..
            } => format!(
                "fn ({}) {{ {} }}",
                parameters.to_vec().iter().collect::<Vec<_>>().join(" , "),
                body
            ),
            Expression::Call {
                function,
                arguments,
                ..
            } => {
                let mut args: String = String::new();

                if let Some(argument) = arguments {
                    let args_formatted = argument
                        .iter()
                        .map(|f| f.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");

                    args.push_str(&args_formatted);
                }
                format!("{}({})", function.token_literal(), args)
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token_literal())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program<'a> {
    pub statements: Vec<Statement>,
    pub arena: StringArena<'a>,
}

impl<'a> Program<'a> {
    pub fn new(arena: StringArena<'a>) -> Self {
        Self {
            statements: Vec::new(),
            arena,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier<'a> {
    pub token: Token<'a>,
    pub value: &'a str,
}

// impl fmt::Display for Identifier {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.token_literal())
//     }
// }

impl<'a> Identifier<'a> {
    pub fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}
