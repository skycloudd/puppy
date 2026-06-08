use crate::ir::{Ident, Span};
use num_bigint::BigUint;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tokens(pub Vec<(Token, Span)>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Error,
    Parentheses(Tokens),
    CurlyBraces(Tokens),
    Ident(Ident),
    Int(BigUint),
    Bool(bool),
    Kw(Kw),
    Ctrl(Ctrl),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kw {
    Print,
    Fn,
    If,
    Elif,
    Else,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrl {
    DoublePlus,
    DoubleMinus,
    DoubleLt,
    DoubleGt,
    Arrow,
    DoubleEquals,
    NotEquals,
    LessThanEquals,
    GreaterThanEquals,
    DoubleColon,
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
    Comma,
    Colon,
    LessThan,
    GreaterThan,
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Error => write!(f, "<error>"),
            Self::Parentheses(_) => write!(f, "(...)"),
            Self::CurlyBraces(_) => write!(f, "{{...}}"),
            Self::Ident(i) => write!(f, "{}", i.resolve()),
            Self::Int(n) => write!(f, "{n}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Kw(kw) => write!(f, "{kw}"),
            Self::Ctrl(ctrl) => write!(f, "{ctrl}"),
        }
    }
}

impl core::fmt::Display for Kw {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Print => write!(f, "print"),
            Self::Fn => write!(f, "fn"),
            Self::If => write!(f, "if"),
            Self::Elif => write!(f, "elif"),
            Self::Else => write!(f, "else"),
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
            Self::Arrow => write!(f, "->"),
            Self::DoubleEquals => write!(f, "=="),
            Self::NotEquals => write!(f, "!="),
            Self::LessThanEquals => write!(f, "<="),
            Self::GreaterThanEquals => write!(f, ">="),
            Self::DoubleColon => write!(f, "::"),
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
            Self::Comma => write!(f, ","),
            Self::Colon => write!(f, ":"),
            Self::LessThan => write!(f, "<"),
            Self::GreaterThan => write!(f, ">"),
        }
    }
}
