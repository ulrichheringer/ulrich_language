use crate::{
    ast::{Expr, Program},
    values::RuntimeVal,
};

fn eval_numeric_binary_expr(lhs: RuntimeVal, rhs: RuntimeVal, operator: String) -> RuntimeVal {
    let result: i64;
    if let RuntimeVal::NumberVal { kind: _, value } = lhs {
        let fstvalue = value;
        if let RuntimeVal::NumberVal { kind: _, value } = rhs {
            if operator == "+" {
                result = fstvalue + value;
            } else if operator == "-" {
                result = fstvalue - value;
            } else if operator == "*" {
                result = fstvalue * value;
            } else if operator == "/" {
                result = fstvalue / value;
            } else {
                result = fstvalue % value;
            }
        } else {
            panic!("Error in binary expr evaluation: rhs");
        }
    } else {
        panic!("Error in binary expr evaluation: lhs");
    }
    RuntimeVal::NumberVal {
        kind: "NumberVal".to_string(),
        value: result,
    }
}

fn eval_binary_expr(left: &Box<Expr>, right: &Box<Expr>, operator: String) -> RuntimeVal {
    let lhs = evaluate(left);
    let rhs = evaluate(right);

    if let RuntimeVal::NumberVal { kind: _, value: _ } = lhs {
        if let RuntimeVal::NumberVal { kind: _, value: _ } = rhs {
            return eval_numeric_binary_expr(lhs, rhs, operator);
        }
    }
    RuntimeVal::NullVal {
        kind: "NullVal".to_string(),
        value: "Null".to_string(),
    }
}

pub fn evaluate(ast_node: &Expr) -> RuntimeVal {
    match ast_node {
        Expr::NumberLiteral { kind: _, value } => RuntimeVal::NumberVal {
            kind: "NumberVal".to_string(),
            value: *value,
        },
        Expr::NullLiteral { kind: _, value } => RuntimeVal::NullVal {
            kind: "NullVal".to_string(),
            value: value.clone(),
        },
        Expr::BinaryExpr {
            kind: _,
            left,
            right,
            operator,
        } => eval_binary_expr(left.clone(), right.clone(), operator.clone()),
        _ => panic!("damn"),
    }
}

pub fn eval_program(program: Program) -> RuntimeVal {
    let mut last_evaluated = RuntimeVal::NullVal {
        kind: "NullVal".to_string(),
        value: "null".to_string(),
    };
    for expr in &program.body {
        last_evaluated = evaluate(expr);
    }
    return last_evaluated;
}
