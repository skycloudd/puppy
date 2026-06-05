use crate::ir::Ident;
use chumsky::span::SimpleSpan;
use ordered_float::OrderedFloat;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tokens(pub Vec<(Token, SimpleSpan<usize, usize>)>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Error,
    Parentheses(Tokens),
    Ident(Ident),
    Number(OrderedFloat<f64>),
    Kw(Kw),
    Ctrl(Ctrl),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kw {
    Print,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ctrl {
    Semicolon,
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Error => write!(f, "<error>"),
            Self::Parentheses(_) => write!(f, "(...)"),
            Self::Ident(i) => write!(f, "{}", i.resolve()),
            Self::Number(n) => write!(f, "{n}"),
            Self::Kw(kw) => write!(f, "{kw}"),
            Self::Ctrl(ctrl) => write!(f, "{ctrl}"),
        }
    }
}

impl core::fmt::Display for Kw {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Print => write!(f, "print"),
        }
    }
}

impl core::fmt::Display for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Semicolon => write!(f, ";"),
        }
    }
}
