use crate::ir::Ident;
use ordered_float::OrderedFloat;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub const fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug)]
pub enum Statement {
    Print(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Number(OrderedFloat<f64>),
    Ident(Ident),
}
