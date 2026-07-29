use crate::{
    interpreter::scopes::Scopes,
    ir::{Ident, ast::Expression},
};
use core::fmt;
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub enum Value {
    Unit,
    Int(BigUint),
    Bool(bool),
    Function {
        params: Vec<Ident>,
        env: Scopes,
        body: Expression,
    },
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unit => "()".to_owned(),
                Self::Int(value) => value.to_string(),
                Self::Bool(value) => value.to_string(),
                Self::Function {
                    params,
                    env: _,
                    body,
                } => format!(
                    "fn ({}) = {:?}",
                    params
                        .iter()
                        .map(|i| i.resolve())
                        .collect::<Vec<_>>()
                        .join(" "),
                    body
                ),
            }
        )
    }
}
