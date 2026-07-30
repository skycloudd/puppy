use crate::ir::{Ident, Span};
use core::fmt;
use num_bigint::BigInt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tokens(pub Vec<(Token, Span)>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Error,
    Parentheses(Tokens),
    CurlyBraces(Tokens),
    SquareBrackets(Tokens),
    Ident(Ident),
    Unit,
    Int(BigInt),
    Bool(bool),
    Kw(Kw),
    Ctrl(Ctrl),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kw {
    Let,
    In,
    If,
    Then,
    Else,
    Fn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrl {
    DoubleLt,
    DoubleGt,
    Arrow,
    DoubleEquals,
    NotEquals,
    LessThanEquals,
    GreaterThanEquals,
    DoubleColon,
    DoubleAmpersand,
    DoublePipe,

    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Dot,
    Ampersand,
    Caret,
    Pipe,
    Comma,
    Colon,
    LessThan,
    GreaterThan,
    Equals,
    Bang,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => write!(f, "<error>"),
            Self::Parentheses(_) => write!(f, "(...)"),
            Self::CurlyBraces(_) => write!(f, "{{...}}"),
            Self::SquareBrackets(_) => write!(f, "[...]"),
            Self::Ident(ident) => write!(f, "{}", ident.resolve()),
            Self::Unit => write!(f, "()"),
            Self::Int(value) => write!(f, "{value}"),
            Self::Bool(value) => write!(f, "{value}"),
            Self::Kw(kw) => write!(f, "{kw}"),
            Self::Ctrl(ctrl) => write!(f, "{ctrl}"),
        }
    }
}

impl fmt::Display for Kw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Let => write!(f, "let"),
            Self::In => write!(f, "in"),
            Self::If => write!(f, "if"),
            Self::Then => write!(f, "then"),
            Self::Else => write!(f, "else"),
            Self::Fn => write!(f, "fn"),
        }
    }
}

impl fmt::Display for Ctrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DoubleLt => write!(f, "<<"),
            Self::DoubleGt => write!(f, ">>"),
            Self::Arrow => write!(f, "->"),
            Self::DoubleEquals => write!(f, "=="),
            Self::NotEquals => write!(f, "!="),
            Self::LessThanEquals => write!(f, "<="),
            Self::GreaterThanEquals => write!(f, ">="),
            Self::DoubleColon => write!(f, "::"),
            Self::DoubleAmpersand => write!(f, "&&"),
            Self::DoublePipe => write!(f, "||"),

            Self::Semicolon => write!(f, ";"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Percent => write!(f, "%"),
            Self::Dot => write!(f, "."),
            Self::Ampersand => write!(f, "&"),
            Self::Caret => write!(f, "^"),
            Self::Pipe => write!(f, "|"),
            Self::Comma => write!(f, ","),
            Self::Colon => write!(f, ":"),
            Self::LessThan => write!(f, "<"),
            Self::GreaterThan => write!(f, ">"),
            Self::Equals => write!(f, "="),
            Self::Bang => write!(f, "!"),
        }
    }
}
