use super::object::Object;
use crate::ast::{Program, ast};

pub fn eval(node: ast::Statement) -> Object {
    match node {
        ast::Statement::Let { token, name, value } => todo!(),
        ast::Statement::Return { token, value } => todo!(),
        ast::Statement::Block { token, statements } => todo!(),
        ast::Statement::Expression(expression) => match expression {
            ast::Expression::Integer { token, value } => return Object::Int(value),
            ast::Expression::Boolean { token, value } => todo!(),
            ast::Expression::Prefix {
                token,
                operator,
                right,
            } => todo!(),
            ast::Expression::Infix {
                token,
                operator,
                right,
                left,
            } => todo!(),
            ast::Expression::If {
                token,
                condition,
                consequence,
                alternative,
            } => todo!(),
            ast::Expression::Function {
                token,
                parameters,
                body,
            } => todo!(),
            ast::Expression::Call {
                token,
                function,
                arguments,
            } => todo!(),
            ast::Expression::Identifier(identifier) => todo!(),
            ast::Expression::None => todo!(),
        },
    }
}

pub fn eval_program(program: &mut Program) -> Object {
    let mut result: Object = Object::Null;
    for stmt in program.statements.iter() {
        result = eval(stmt.clone())
    }
    result
}
