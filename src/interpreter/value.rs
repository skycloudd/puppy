use crate::ir::{Ident, ast::Expression};
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub enum Value {
    Unit,
    Int(BigUint),
    Bool(bool),
    Function {
        params: Vec<Ident>,
        body: Expression,
    },
}
