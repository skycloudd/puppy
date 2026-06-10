#![allow(dead_code)]

use crate::ir::{Ident, Spanned};
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct Ast {
    pub statements: Spanned<Statements>,
}

type Statements = Vec<Spanned<Statement>>;

#[derive(Clone, Debug)]
pub enum Statement {
    Print(Spanned<Expression>),
    Function {
        name: Spanned<Ident>,
        params: Spanned<Vec<(Spanned<Ident>, Spanned<Path>)>>,
        return_type: Option<Spanned<Path>>,
        body: Spanned<Statements>,
        return_expr: Option<Spanned<Expression>>,
    },
    Block(Spanned<Statements>),
    Conditional {
        if_: Spanned<ConditionalBranch>,
        elifs: Spanned<Vec<Spanned<ConditionalBranch>>>,
        else_: Option<Spanned<Statements>>,
    },
}

#[derive(Clone, Debug)]
pub struct ConditionalBranch {
    pub condition: Spanned<Expression>,
    pub block: Spanned<Statements>,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Int(BigUint),
    Bool(bool),
    Ident(Ident),
    Path(Path),
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

#[derive(Clone, Debug)]
pub struct Path(pub Vec<Spanned<Ident>>);

#[derive(Clone, Copy, Debug)]
pub enum PrefixOp {
    Inc,
    Dec,
    Pos,
    Neg,
    LogicalNot,
    BitwiseNot,
}

#[derive(Clone, Debug)]
pub enum PostfixOp {
    Inc,
    Dec,
    FieldAccess(Spanned<Ident>),
    FunctionCall(Spanned<Vec<Spanned<Expression>>>),
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
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
