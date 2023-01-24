use crate::{
    ast::{Expr, Program},
    values::RuntimeVal,
};

pub fn evaluate(ast_node: &Expr) -> RuntimeVal {
    match ast_node {
        Expr::NumberLiteral { kind: _, value } => RuntimeVal::NumberVal {
            kind: "NumberLiteral".to_string(),
            value: *value,
        },
        Expr::NullLiteral { kind: _, value } => RuntimeVal::NullVal {
            kind: "NullLiteral".to_string(),
            value: value.clone(),
        },
        _ => panic!("damn"),
    }
}

pub fn eval_program(program: Program) -> RuntimeVal {
    let mut last_evaluated = RuntimeVal::NullVal {
        kind: "NullLiteral".to_string(),
        value: "null".to_string(),
    };
    for expr in &program.body {
        last_evaluated = evaluate(expr);
    }
    return last_evaluated;
}
