use crate::ir::{Ident, Spanned, ast::Path};
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct Ast {
    pub statements: Spanned<Statements>,
}

type Statements = Vec<Spanned<Statement>>;

#[derive(Clone, Debug)]
pub enum Statement {
    Print(Spanned<TypedExpression>),
    Function {
        name: Spanned<Ident>,
        params: Spanned<Vec<(Spanned<Ident>, Spanned<Type>)>>,
        return_type: Option<Spanned<Type>>,
        body: Spanned<Statements>,
    },
    Block(Spanned<Statements>),
    Conditional {
        if_: Spanned<(Spanned<TypedExpression>, Spanned<Statements>)>,
        elifs: Vec<Spanned<(Spanned<TypedExpression>, Spanned<Statements>)>>,
        else_: Option<Spanned<Statements>>,
    },
    Return(Option<Spanned<TypedExpression>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Error,
    Bool,
    Int,
    Function {
        params: Spanned<Vec<Spanned<Self>>>,
        return_type: Spanned<Box<Self>>,
    },
}

#[derive(Clone, Debug)]
pub struct TypedExpression {
    pub expr: Expression,
    pub ty: Type,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Int(BigUint),
    Bool(bool),
    Path(Path),
    PrefixOp {
        expr: Spanned<Box<TypedExpression>>,
        op: Spanned<PrefixOp>,
    },
    PostfixOp {
        expr: Spanned<Box<TypedExpression>>,
        op: Spanned<PostfixOp>,
    },
    BinaryOp {
        lhs: Spanned<Box<TypedExpression>>,
        rhs: Spanned<Box<TypedExpression>>,
        op: Spanned<BinaryOp>,
    },
}

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
    FunctionCall(Spanned<Vec<Spanned<TypedExpression>>>),
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
