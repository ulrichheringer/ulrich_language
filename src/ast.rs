use std::collections::VecDeque;

#[derive(Debug)]
pub struct Program {
    pub body: VecDeque<Expr>,
    pub kind: String,
}
#[derive(Debug)]
pub enum Expr {
    BinaryExpr {
        kind: String,
        left: Box<Expr>,
        right: Box<Expr>,
        operator: String,
    },
    Identifier {
        kind: String,
        value: String,
    },
    NumberLiteral {
        kind: String,
        value: i64,
    },
}
