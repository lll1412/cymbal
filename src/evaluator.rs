use crate::ast::{BlockStatement, Expression, Infix, Prefix, Program, Statement};
use crate::object::Object;

pub fn eval(program: Program) -> Object {
    eval_statements(program.statements)
}

fn eval_statements(statements: Vec<Statement>) -> Object {
    let mut result = Object::Null;
    for statement in statements {
        result = eval_statement(statement);
    }
    result
}

fn eval_statement(statement: Statement) -> Object {
    match statement {
        Statement::Expression(exp) => eval_expression(exp),
        _ => Object::Null,
    }
}

fn eval_expression(expression: Expression) -> Object {
    match expression {
        Expression::IntegerLiteral(int) => Object::Integer(int),
        Expression::Boolean(value) => Object::Boolean(value),
        Expression::Prefix(prefix, exp) => eval_prefix_expression(prefix, *exp),
        Expression::Infix(infix, left, right) => eval_infix_expression(infix, *left, *right),
        Expression::If(condition, consequence, alternative) => {
            eval_if_expression(*condition, consequence, alternative)
        }
        _ => Object::Null,
    }
}

fn eval_prefix_expression(prefix: Prefix, exp: Expression) -> Object {
    let obj = eval_expression(exp);

    match prefix {
        Prefix::Bang => match obj {
            Object::Boolean(value) => Object::Boolean(!value),
            _ => Object::Null,
        },
        Prefix::Minus => match obj {
            Object::Integer(value) => Object::Integer(-value),
            _ => Object::Null,
        },
    }
}

fn eval_infix_expression(infix: Infix, left_exp: Expression, right_exp: Expression) -> Object {
    let left_obj = eval_expression(left_exp);
    let right_obj = eval_expression(right_exp);

    match infix {
        Infix::Eq => match (left_obj, right_obj) {
            (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left == right),
            (Object::Boolean(left), Object::Boolean(right)) => Object::Boolean(left == right),
            _ => Object::Boolean(false),
        },
        Infix::NotEq => match (left_obj, right_obj) {
            (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left != right),
            (Object::Boolean(left), Object::Boolean(right)) => Object::Boolean(left != right),
            _ => Object::Boolean(false),
        },
        Infix::Lt => match (left_obj, right_obj) {
            (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left < right),
            _ => Object::Boolean(false),
        },
        Infix::Gt => match (left_obj, right_obj) {
            (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left > right),
            _ => Object::Boolean(false),
        },
        Infix::Plus => match (left_obj, right_obj) {
            (Object::Integer(left), Object::Integer(right)) => Object::Integer(left + right),
            _ => Object::Null,
        },
        Infix::Minus => match (left_obj, right_obj) {
            (Object::Integer(left), Object::Integer(right)) => Object::Integer(left - right),
            _ => Object::Null,
        },
        Infix::Asterisk => match (left_obj, right_obj) {
            (Object::Integer(left), Object::Integer(right)) => Object::Integer(left * right),
            _ => Object::Null,
        },
        Infix::Slash => match (left_obj, right_obj) {
            (Object::Integer(left), Object::Integer(right)) => Object::Integer(left / right),
            _ => Object::Null,
        },
    }
}

fn eval_if_expression(
    condition: Expression,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
) -> Object {
    if is_truthy(eval_expression(condition)) {
        eval_block_statement(consequence)
    } else {
        alternative
            .map(|a| eval_block_statement(a))
            .unwrap_or(Object::Null)
    }
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Boolean(value) => value,
        Object::Null => false,
        _ => true,
    }
}

fn eval_block_statement(block: BlockStatement) -> Object {
    eval_statements(block.statements)
}
