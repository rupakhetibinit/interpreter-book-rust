use core::fmt;

use crate::lexer::token::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'s> {
    Let {
        token: Token<'s>,
        name: Identifier<'s>,
        value: Expression<'s>,
    },
    Return {
        token: Token<'s>,
        value: Expression<'s>,
    },
    Block {
        token: Token<'s>,
        statements: Vec<Statement<'s>>,
    },
    Expression(Expression<'s>),
}

impl<'s> fmt::Display for Statement<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let { token, name, value } => {
                write!(f, "{} {} = {};", token.literal, name.token.literal, value)
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

// impl<'s> Statement<'s> {
//     pub fn token_literal(&self) -> &'s str {
//         match self {
//             Statement::Let { token, .. } => token.literal,
//             Statement::Return { token, .. } => token.literal,
//             Statement::Expression(expression) => &expression.token_literal(),
//             Statement::Block { token, .. } => token.literal,
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'e> {
    Integer {
        token: Token<'e>,
        value: i64,
    },
    Boolean {
        token: Token<'e>,
        value: bool,
    },
    Prefix {
        token: Token<'e>,
        operator: &'e str,
        right: Box<Option<Expression<'e>>>,
    },
    Infix {
        token: Token<'e>,
        operator: &'e str,
        right: Box<Expression<'e>>,
        left: Box<Expression<'e>>,
    },
    If {
        token: Token<'e>,
        condition: Box<Expression<'e>>,
        consequence: Box<Statement<'e>>,
        alternative: Option<Box<Statement<'e>>>,
    },
    Function {
        token: Token<'e>,
        parameters: Vec<Identifier<'e>>,
        body: Box<Statement<'e>>,
    },
    Call {
        token: Token<'e>,
        function: Box<Expression<'e>>,
        arguments: Option<Vec<Expression<'e>>>,
    },
    Identifier(Identifier<'e>),
    None,
}

impl<'e> Expression<'e> {
    pub fn token_literal(&self) -> String {
        match self {
            Expression::Integer { token, .. } => token.literal.clone().to_owned(),
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
            } => format!(
                "({} {} {})",
                left.token_literal(),
                operator,
                right.token_literal()
            ),
            Expression::If {
                condition,
                consequence,
                alternative,
                ..
            } => match alternative {
                Some(x) => format!(
                    "if {} {} else {}",
                    condition.to_string(),
                    consequence.to_string(),
                    x.to_string(),
                ),
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

impl<'e> fmt::Display for Expression<'e> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token_literal())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program<'p> {
    pub statements: Vec<Statement<'p>>,
}

impl<'p> Program<'p> {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier<'i> {
    pub token: Token<'i>,
    pub value: &'i str,
}

impl<'i> fmt::Display for Identifier<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token_literal())
    }
}

impl<'i> Identifier<'i> {
    pub fn token_literal(&self) -> &'i str {
        self.token.literal.clone()
    }
}
