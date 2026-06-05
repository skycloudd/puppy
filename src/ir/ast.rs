use crate::ir::{Ident, Spanned};
use ordered_float::OrderedFloat;

#[derive(Debug)]
pub struct Ast {
    pub statements: Spanned<Vec<Spanned<Statement>>>,
}

impl Ast {
    pub const fn new(statements: Spanned<Vec<Spanned<Statement>>>) -> Self {
        Self { statements }
    }
}

#[derive(Debug)]
pub enum Statement {
    Print(Spanned<Expression>),
}

#[derive(Debug)]
pub enum Expression {
    Number(OrderedFloat<f64>),
    Ident(Ident),
    UnaryOp {
        rhs: Spanned<Box<Self>>,
        op: Spanned<UnaryOp>,
    },
    BinaryOp {
        lhs: Spanned<Box<Self>>,
        rhs: Spanned<Box<Self>>,
        op: Spanned<BinaryOp>,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOp {
    Neg,
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}
