use core::fmt;
use std::borrow::Cow;

use crate::lexer::token::Token;

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
    pub fn token_literal(&self) -> String {
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
    Integer {
        token: Token,
        value: i64,
    },
    Boolean {
        token: Token,
        value: bool,
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
    If {
        token: Token,
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    Function {
        token: Token,
        parameters: Vec<Identifier>,
        body: Box<Statement>,
    },
    Call {
        token: Token,
        function: Box<Expression>,
        arguments: Option<Vec<Expression>>,
    },
    Identifier(Identifier),
    None,
}

impl Expression {
    pub fn token_literal(&self) -> String {
        match self {
            Expression::Integer { token, .. } => token.literal.clone(),
            Expression::Prefix {
                operator, right, ..
            } => match right.as_ref() {
                Some(expression) => format!("{}{}", operator, expression.token_literal()),
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
                parameters
                    .to_vec()
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(" , "),
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
            Expression::Boolean { value, .. } => format!("{}", value.to_string()),
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
    pub statements: Vec<Statement>,
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
    pub token: Token,
    pub value: Cow<'static, str>,
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
