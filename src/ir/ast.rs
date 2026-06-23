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
        params: Spanned<Vec<(Spanned<Ident>, Spanned<ParsedType>)>>,
        return_type: Option<Spanned<ParsedType>>,
        body: Spanned<Statements>,
    },
    Block(Spanned<Statements>),
    Conditional {
        if_: Spanned<(Spanned<Expression>, Spanned<Statements>)>,
        elifs: Vec<Spanned<(Spanned<Expression>, Spanned<Statements>)>>,
        else_: Option<Spanned<Statements>>,
    },
    Return(Option<Spanned<Expression>>),
}

#[derive(Clone, Debug)]
pub enum ParsedType {
    Path(Path),
    Function {
        params: Spanned<Vec<Spanned<Self>>>,
        return_type: Spanned<Box<Self>>,
    },
}

#[derive(Clone, Debug)]
pub enum Expression {
    Int(BigUint),
    Bool(bool),
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
