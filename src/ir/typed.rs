use crate::ir::{
    Ident, Spanned,
    ast::{InfixOp, PostfixOp, PrefixOp},
};
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct Ast(pub Vec<Spanned<ModuleExpression>>);

#[derive(Clone, Debug)]
pub enum ModuleExpression {
    Expression(Spanned<TypedExpression>),
    Let {
        name: Spanned<Ident>,
        params: Vec<Spanned<Ident>>,
        expr: Spanned<TypedExpression>,
    },
}

#[derive(Clone, Debug)]
pub struct TypedExpression {
    pub expr: Expression,
    pub ty: Type,
}

#[derive(Clone, Debug)]
pub enum Type {
    Int,
    Bool,
    Function(Box<Self>, Box<Self>),
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
