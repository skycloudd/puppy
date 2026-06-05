use chumsky::span::SimpleSpan;
use ordered_float::OrderedFloat;

#[salsa::tracked(debug)]
pub struct Program<'db> {
    #[tracked]
    #[returns(ref)]
    pub statements: Vec<Statement<'db>>,
}

#[salsa::tracked(debug)]
pub struct Function<'db> {
    pub name: Ident<'db>,

    #[tracked]
    #[returns(ref)]
    pub args: Vec<Ident<'db>>,

    #[tracked]
    #[returns(ref)]
    pub body: Expression<'db>,
}

#[salsa::interned(debug)]
pub struct Ident<'db> {
    #[returns(ref)]
    pub text: String,

    span: SimpleSpan<usize, usize>,
}

#[derive(Debug, Hash, PartialEq, Eq, salsa::Update)]
pub struct Statement<'db> {
    pub data: StatementData<'db>,
    pub span: SimpleSpan<usize, usize>,
}

impl<'db> Statement<'db> {
    pub const fn new(data: StatementData<'db>, span: SimpleSpan<usize, usize>) -> Self {
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
    pub span: SimpleSpan<usize, usize>,
}

impl<'db> Expression<'db> {
    pub const fn new(data: ExpressionData<'db>, span: SimpleSpan<usize, usize>) -> Self {
        Expression { data, span }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, salsa::Update)]
pub enum ExpressionData<'db> {
    Number(OrderedFloat<f64>),
    Ident(Ident<'db>),
}
