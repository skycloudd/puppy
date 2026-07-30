use crate::{
    interpreter::value::Value,
    ir::{
        Ident, Span, Spanned,
        ast::{InfixOp, PrefixOp},
    },
};
use chumsky::error::Rich;
use codespan_reporting::diagnostic::{self, Label};
use core::fmt::Display;

#[derive(Debug)]
pub enum Diagnostic {
    ParserError(ParserError),
    CantApplyPrefixOp {
        expr: Box<Spanned<Value>>,
        op: Spanned<PrefixOp>,
    },
    CantApplyInfixOp {
        lhs: Box<Spanned<Value>>,
        rhs: Box<Spanned<Value>>,
        op: Box<Spanned<InfixOp>>,
    },
    CantCall {
        callee: Box<Spanned<Value>>,
    },
    NameNotDefined {
        ident: Spanned<Ident>,
    },
    InvalidCondition {
        condition: Box<Spanned<Value>>,
    },
}

impl Diagnostic {
    pub fn report(&self) -> diagnostic::Diagnostic<usize> {
        match self {
            Self::ParserError(parser_error) => diagnostic::Diagnostic::error()
                .with_message(&parser_error.reason)
                .with_label(Label::primary(
                    parser_error.span.context,
                    parser_error.span.into_range(),
                )),
            Self::CantApplyPrefixOp { expr, op } => diagnostic::Diagnostic::error()
                .with_message(format!(
                    "cannot apply the operator '{}' to this value",
                    op.inner
                ))
                .with_label(Label::primary(op.span.context, op.span.into_range()))
                .with_label(
                    Label::secondary(expr.span.context, expr.span.into_range())
                        .with_message("operand"),
                ),
            Self::CantApplyInfixOp { lhs, rhs, op } => diagnostic::Diagnostic::error()
                .with_message(format!(
                    "cannot apply the operator '{}' to these two values",
                    op.inner
                ))
                .with_label(Label::primary(op.span.context, op.span.into_range()))
                .with_label(
                    Label::secondary(lhs.span.context, lhs.span.into_range())
                        .with_message("lefthand operand"),
                )
                .with_label(
                    Label::secondary(rhs.span.context, rhs.span.into_range())
                        .with_message("righthand operand"),
                ),
            Self::CantCall { callee } => diagnostic::Diagnostic::error()
                .with_message("cannot call this value as a function")
                .with_label(Label::primary(
                    callee.span.context,
                    callee.span.into_range(),
                )),
            Self::NameNotDefined { ident } => diagnostic::Diagnostic::error()
                .with_message(format!(
                    "name '{}' is not defined in this scope",
                    ident.resolve()
                ))
                .with_label(Label::primary(ident.span.context, ident.span.into_range())),
            Self::InvalidCondition { condition } => diagnostic::Diagnostic::error()
                .with_message("this value is not of type bool, so it cannot be used as condition in an if expression")
                .with_label(Label::primary(
                    condition.span.context,
                    condition.span.into_range(),
                )),
        }
    }
}

#[derive(Debug)]
pub struct ParserError {
    pub reason: String,
    pub span: Span,
}

impl<'a, T: Display> From<Rich<'a, T, Span>> for ParserError {
    fn from(value: Rich<'a, T, Span>) -> Self {
        Self {
            reason: value.reason().to_string(),
            span: *value.span(),
        }
    }
}
