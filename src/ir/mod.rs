use crate::RODEO;
use lasso::Spur;

pub mod ast;
pub mod token;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ident(Spur);

impl Ident {
    pub fn resolve(&self) -> &str {
        RODEO.resolve(&self.0)
    }

    pub fn get_or_intern(ident: &str) -> Self {
        Self(RODEO.get_or_intern(ident))
    }
}
