use crate::{
    diagnostics::Diagnostic,
    interpreter::env::Env,
    ir::{Ident, ast::Expression},
};
use core::fmt;
use num_bigint::BigInt;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Value {
    Unit,
    Int(BigInt),
    Bool(bool),
    Function {
        param: Ident,
        env: Env,
        body: Expression,
    },
    Builtin(Rc<dyn Builtin>),
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
                    param,
                    env: _,
                    body: _,
                } => format!("fn ({}) = <expr>", param.resolve()),
                Self::Builtin(_builtin) => "<builtin>".to_owned(),
            }
        )
    }
}

pub trait Builtin: Fn(Value) -> Result<Value, Diagnostic> {}

impl<T: Fn(Value) -> Result<Value, Diagnostic>> Builtin for T {}

impl fmt::Debug for dyn Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<builtin>")
    }
}
