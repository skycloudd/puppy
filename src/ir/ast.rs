use crate::ir::{Ident, Spanned};
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct Ast {
    pub statements: Spanned<Vec<Spanned<Statement>>>,
}

impl Ast {
    pub const fn new(statements: Spanned<Vec<Spanned<Statement>>>) -> Self {
        Self { statements }
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Print(Spanned<Expression>),
    Function {
        name: Spanned<Ident>,
        params: Spanned<Vec<(Spanned<Ident>, Spanned<Ident>)>>,
        return_type: Spanned<Ident>,
        body: Spanned<Vec<Spanned<Self>>>,
        return_expr: Option<Spanned<Expression>>,
    },
    Block(Spanned<Vec<Spanned<Self>>>),
    Conditional {
        condition: Spanned<Expression>,
        if_: Spanned<Vec<Spanned<Self>>>,
        elifs: Vec<(Spanned<Expression>, Spanned<Vec<Spanned<Self>>>)>,
        else_: Option<Spanned<Vec<Spanned<Self>>>>,
    },
}

#[derive(Clone, Debug)]
pub enum Expression {
    Int(BigUint),
    Bool(bool),
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
    FieldAccess(Spanned<Ident>),
    FunctionCall(Spanned<Vec<Spanned<Expression>>>),
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
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
}
