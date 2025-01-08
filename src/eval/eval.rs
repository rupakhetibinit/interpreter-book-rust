use super::object::Object;
use crate::ast::{Program, Statement, ast};

pub fn eval(node: ast::Statement) -> Option<Object> {
    match node {
        ast::Statement::Let { token, name, value } => Some(todo!()),
        ast::Statement::Return { value, .. } => {
            let val = eval(Statement::Expression(value));
            Some(Object::ReturnValue(Box::new(val)))
        }
        ast::Statement::Block { statements, .. } => Some(eval_block_statement(&statements)),
        ast::Statement::Expression(expression) => match expression {
            ast::Expression::Integer { value, .. } => Some(Object::Int(value)),
            ast::Expression::Boolean { value, .. } => Some(Object::Bool(value)),
            ast::Expression::Prefix {
                right, operator, ..
            } => {
                let rt = eval(ast::Statement::Expression(right.unwrap()))?;
                eval_prefix_expression(operator, rt)
            }
            ast::Expression::Infix {
                operator,
                right,
                left,
                ..
            } => {
                let lt = eval(ast::Statement::Expression(*left))?;
                let rt = eval(ast::Statement::Expression(*right))?;
                eval_infix_expression(operator, lt, rt)
            }
            ast::Expression::If {
                condition,
                consequence,
                alternative,
                ..
            } => eval_if_expression(condition, consequence, alternative),
            ast::Expression::Function {
                token,
                parameters,
                body,
            } => Some(todo!()),
            ast::Expression::Call {
                token,
                function,
                arguments,
            } => Some(todo!()),
            ast::Expression::Identifier(identifier) => Some(todo!()),
            ast::Expression::None => Some(todo!()),
        },
    }
}

fn eval_block_statement(statements: &[Statement<'_>]) -> Object {
    let mut result = Some(Object::Null);

    for stmt in statements.iter() {
        let res = eval(stmt.clone());
        match res {
            Some(obj @ Object::ReturnValue(_)) => return obj,
            Some(obj) => result = Some(obj),
            None => continue,
        }
    }
    result.unwrap()
}

fn eval_if_expression(
    condition: Box<ast::Expression<'_>>,
    consequence: Box<Statement<'_>>,
    alternative: Option<Box<Statement<'_>>>,
) -> Option<Object> {
    let condition = eval(Statement::Expression(*condition))?;

    if is_truthy(condition) {
        eval(*consequence)
    } else {
        alternative.map(|alt| eval(*alt)).unwrap_or(None)
    }
}

fn is_truthy(object: Object) -> bool {
    match object {
        Object::Int(_) => true,
        Object::Bool(b) => b,
        Object::Null => false,
        _ => true,
    }
}

fn eval_infix_expression(operator: &str, lt: Object, rt: Object) -> Option<Object> {
    match (lt, rt) {
        (Object::Int(l), Object::Int(r)) => eval_integer_infix_operation(operator, l, r),
        _ => None,
    }
}

fn eval_integer_infix_operation(operator: &str, l: i64, r: i64) -> Option<Object> {
    match operator {
        "+" => Some(Object::Int(l + r)),
        "-" => Some(Object::Int(l - r)),
        "*" => Some(Object::Int(l * r)),
        "/" => Some(Object::Int(l / r)),
        "<" => Some(Object::Bool(l < r)),
        ">" => Some(Object::Bool(l > r)),
        "!=" => Some(Object::Bool(l != r)),
        "==" => Some(Object::Bool(l == r)),
        _ => None,
    }
}

pub fn eval_program(program: &mut Program) -> Object {
    let mut result = Some(Object::Null);

    for stmt in program.statements.iter() {
        let return_value = eval(stmt.clone());
        if let Some(Object::ReturnValue(_)) = return_value {
            return return_value.unwrap();
        }
        result = return_value;
    }

    result.unwrap_or(Object::Null)
}

// pub fn eval_statements(statements: &[Statement]) -> Object {
//     let mut result = Some(Object::Null);
//     for stmt in statements.iter() {
//         result = eval(stmt.clone());
//     }
//     match result {
//         Some(x) => x,
//         None => Object::Null,
//     }
// }

pub fn eval_prefix_expression(operator: &str, right: Object) -> Option<Object> {
    match operator {
        "-" => eval_minus_prefix_operator_expression(right),
        "!" => Some(eval_bang_operator_expression(right)),
        _ => None,
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Option<Object> {
    match right {
        Object::Int(i) => Some(Object::Int(-i)),
        _ => None,
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        Object::Bool(r) => Object::Bool(!r),
        Object::Int(_) => Object::Bool(false),
        Object::Null => Object::Bool(true),
        _ => Object::Null,
    }
}
