use crate::RODEO;
use chumsky::span::{SimpleSpan, SimpleSpanned};
use lasso::Spur;

pub mod ast;
pub mod token;

pub type Span = SimpleSpan<usize, usize>;
pub type Spanned<T> = SimpleSpanned<T, usize, usize>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Ident(Spur);

impl Ident {
    pub fn resolve(self) -> &'static str {
        RODEO.resolve(&self.0)
    }

    pub fn get_or_intern(ident: &str) -> Self {
        Self(RODEO.get_or_intern(ident))
    }
}
