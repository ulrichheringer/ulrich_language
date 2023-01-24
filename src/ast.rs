use std::collections::VecDeque;

#[derive(Debug)]
pub struct Program {
    pub body: VecDeque<Expr>,
    pub kind: String,
}
#[derive(Debug)]
pub enum ExprTypes {
    BinaryExpr,
    Identifier,
    NumberLiteral,
    NullLiteral,
}
#[derive(Debug)]
pub enum Expr {
    BinaryExpr {
        kind: ExprTypes,
        left: Box<Expr>,
        right: Box<Expr>,
        operator: String,
    },
    Identifier {
        kind: ExprTypes,
        value: String,
    },
    NumberLiteral {
        kind: ExprTypes,
        value: i64,
    },
    NullLiteral {
        kind: ExprTypes,
        value: String,
    },
}
#[derive(Debug)]
pub struct NumberLiteral {
    pub kind: ExprTypes,
    pub value: i64,
}
