use crate::span::{Ctx, Span};
use ordered_float::OrderedFloat;

#[salsa::input(debug)]
pub struct SourceProgram {
    #[returns(ref)]
    pub text: String,

    pub file_ctx: Ctx,
}

#[salsa::tracked(debug)]
pub struct TokenList<'db> {
    #[tracked]
    #[returns(ref)]
    pub tokens: Vec<(Token, Span)>,

    pub file_ctx: Ctx,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Print,
    Fn,
    Number(OrderedFloat<f64>),
    Ident(String),
    Semicolon,
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Print => "print".to_string(),
                Self::Fn => "fn".to_string(),
                Self::Number(ordered_float) => ordered_float.to_string(),
                Self::Ident(i) => i.clone(),
                Self::Semicolon => ";".to_string(),
            }
        )
    }
}

#[salsa::tracked(debug)]
pub struct Program<'db> {
    #[tracked]
    #[returns(ref)]
    pub statements: Vec<Statement<'db>>,
}

#[salsa::tracked(debug)]
pub struct Function<'db> {
    pub name: FunctionId<'db>,

    #[tracked]
    #[returns(ref)]
    pub args: Vec<VariableId<'db>>,

    #[tracked]
    #[returns(ref)]
    pub body: Expression<'db>,
}

#[salsa::interned(debug)]
pub struct FunctionId<'db> {
    #[returns(ref)]
    pub text: String,

    span: Span,
}

#[salsa::interned(debug)]
pub struct VariableId<'db> {
    #[returns(ref)]
    pub text: String,

    span: Span,
}

#[derive(Debug, Hash, PartialEq, Eq, salsa::Update)]
pub struct Statement<'db> {
    pub data: StatementData<'db>,
    pub span: Span,
}

impl<'db> Statement<'db> {
    pub const fn new(data: StatementData<'db>, span: Span) -> Self {
        Statement { data, span }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, salsa::Update)]
pub enum StatementData<'db> {
    Print(Expression<'db>),
}

#[derive(Debug, Hash, PartialEq, Eq, salsa::Update)]
pub struct Expression<'db> {
    pub data: ExpressionData<'db>,
    pub span: Span,
}

impl<'db> Expression<'db> {
    pub const fn new(data: ExpressionData<'db>, span: Span) -> Self {
        Expression { data, span }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, salsa::Update)]
pub enum ExpressionData<'db> {
    Number(OrderedFloat<f64>),
    Variable(VariableId<'db>),
}
