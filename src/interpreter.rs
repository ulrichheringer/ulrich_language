use crate::{
    ast::{Expr, Program},
    values::RuntimeVal,
};

fn eval_numeric_binary_expr(
    lhs: Option<RuntimeVal>,
    rhs: Option<RuntimeVal>,
    operator: String,
) -> RuntimeVal {
    let result: i64;
    if let Some(RuntimeVal::NumberVal { kind: _, value }) = lhs {
        let fstvalue = value;
        if let Some(RuntimeVal::NumberVal { kind: _, value }) = rhs {
            // MATCH
            if operator == "+" {
                result = fstvalue + value;
            } else if operator == "-" {
                result = fstvalue - value;
            } else if operator == "*" {
                result = fstvalue * value;
            } else if operator == "/" {
                if value == 0 {
                    panic!("Can't divide by zero");
                }
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
    return RuntimeVal::NumberVal {
        kind: "NumberVal".to_string(),
        value: result,
    };
}

fn eval_binary_expr(left: &Box<Expr>, right: &Box<Expr>, operator: String) -> RuntimeVal {
    let lhs = evaluate(left);
    let rhs = evaluate(right);

    if let Some(RuntimeVal::NumberVal { kind: _, value: _ }) = lhs {
        if let Some(RuntimeVal::NumberVal { kind: _, value: _ }) = rhs {
            return eval_numeric_binary_expr(lhs, rhs, operator);
        }
    }
    return RuntimeVal::NullVal {
        kind: "NullVal".to_string(),
        value: "Null".to_string(),
    };
}

pub fn evaluate(ast_node: &Expr) -> Option<RuntimeVal> {
    match ast_node {
        Expr::NumberLiteral { kind: _, value } => Some(RuntimeVal::NumberVal {
            kind: "NumberVal".to_string(),
            value: *value,
        }),
        Expr::NullLiteral { kind: _, value } => Some(RuntimeVal::NullVal {
            kind: "NullVal".to_string(),
            value: value.clone(),
        }),
        Expr::BinaryExpr {
            kind: _,
            left,
            right,
            operator,
        } => Some(eval_binary_expr(
            left.clone(),
            right.clone(),
            operator.clone(),
        )),
        Expr::Identifier { kind: _, value } => Some(RuntimeVal::IdentVal {
            kind: "IdentVal".to_string(),
            value: value.clone(),
        }),
        Expr::TextLiteral { value } => Some(RuntimeVal::TextVal {
            value: value.clone(),
        }),
        _ => None,
    }
}

pub fn eval_program(program: Program) -> Option<RuntimeVal> {
    let mut last_evaluated = Some(RuntimeVal::NullVal {
        kind: "NullVal".to_string(),
        value: "null".to_string(),
    });
    for expr in &program.body {
        last_evaluated = evaluate(expr);
    }
    return last_evaluated;
}
