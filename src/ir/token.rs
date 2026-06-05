use crate::ir::{Ident, Span};
use ordered_float::OrderedFloat;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tokens(pub Vec<(Token, Span)>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Error,
    Parentheses(Tokens),
    CurlyBraces(Tokens),
    Ident(Ident),
    Number(OrderedFloat<f64>),
    Kw(Kw),
    Ctrl(Ctrl),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kw {
    Print,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrl {
    DoublePlus,
    DoubleMinus,
    DoubleLt,
    DoubleGt,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Dot,
    Tilde,
    Ampersand,
    Caret,
    Pipe,
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Error => write!(f, "<error>"),
            Self::Parentheses(_) => write!(f, "(...)"),
            Self::CurlyBraces(_) => write!(f, "{{...}}"),
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
            Self::DoublePlus => write!(f, "++"),
            Self::DoubleMinus => write!(f, "--"),
            Self::DoubleLt => write!(f, "<<"),
            Self::DoubleGt => write!(f, ">>"),
            Self::Semicolon => write!(f, ";"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Percent => write!(f, "%"),
            Self::Dot => write!(f, "."),
            Self::Tilde => write!(f, "~"),
            Self::Ampersand => write!(f, "&"),
            Self::Caret => write!(f, "^"),
            Self::Pipe => write!(f, "|"),
        }
    }
}
