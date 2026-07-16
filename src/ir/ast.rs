use crate::ir::{Ident, Spanned};
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct Ast(pub Vec<Spanned<ModuleExpression>>);

#[derive(Clone, Debug)]
pub enum ModuleExpression {
    Expression(Spanned<Expression>),
    Let {
        name: Spanned<Ident>,
        params: Vec<Spanned<Ident>>,
        expr: Spanned<Expression>,
    },
}

#[derive(Clone, Debug)]
pub enum Expression {
    Int(BigUint),
    Bool(bool),
    Ident(Ident),
    Let {
        name: Spanned<Ident>,
        params: Vec<Spanned<Ident>>,
        expr: Spanned<Box<Self>>,
        in_: Spanned<Box<Self>>,
    },
    Call {
        callee: Spanned<Box<Self>>,
        arg: Spanned<Box<Self>>,
    },
    Semicolon(Spanned<Box<Self>>, Option<Spanned<Box<Self>>>),
    PrefixOp {
        expr: Spanned<Box<Self>>,
        op: Spanned<PrefixOp>,
    },
    PostfixOp {
        expr: Spanned<Box<Self>>,
        op: Spanned<PostfixOp>,
    },
    InfixOp {
        lhs: Spanned<Box<Self>>,
        rhs: Spanned<Box<Self>>,
        op: Spanned<InfixOp>,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum PrefixOp {
    Pos,
    Neg,
    LogicalNot,
    BitwiseNot,
}

#[derive(Clone, Debug)]
pub enum PostfixOp {
    FieldAccess(Spanned<Ident>),
}

#[derive(Clone, Copy, Debug)]
pub enum InfixOp {
    Mul,
    Div,
    Modulo,
    Add,
    Sub,
    LeftBitshift,
    RightBitshift,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
}
