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

#[derive(Clone, Debug)]
pub enum Expression {
    Number(OrderedFloat<f64>),
    Ident(Ident),
    PrefixOp {
        expr: Spanned<Box<Self>>,
        op: Spanned<PrefixOp>,
    },
    PostfixOp {
        expr: Spanned<Box<Self>>,
        op: Spanned<PostfixOp>,
    },
    BinaryOp {
        lhs: Spanned<Box<Self>>,
        rhs: Spanned<Box<Self>>,
        op: Spanned<BinaryOp>,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum PrefixOp {
    Inc,
    Dec,
    Pos,
    Neg,
    BitwiseNot,
}

#[derive(Clone, Debug)]
pub enum PostfixOp {
    Inc,
    Dec,
    FieldAccess(Ident),
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    LeftBitshift,
    RightBitshift,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
}
