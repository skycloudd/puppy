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
        params: Spanned<Vec<(Spanned<Ident>, Spanned<Path>)>>,
        return_type: Option<Spanned<Path>>,
        body: Spanned<Statements>,
        return_expr: Option<Spanned<TypedExpression>>,
    },
    Block(Spanned<Statements>),
    Conditional {
        if_: Spanned<ConditionalBranch>,
        elifs: Vec<Spanned<ConditionalBranch>>,
        else_: Option<Spanned<Statements>>,
    },
}

#[derive(Clone, Debug)]
pub struct ConditionalBranch {
    pub condition: Spanned<TypedExpression>,
    pub block: Spanned<Statements>,
}

#[derive(Clone, Debug)]
pub enum Type {
    Error,
    Bool,
    Int,
    User(Path),
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
    Ident(Ident),
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
