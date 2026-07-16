use crate::ir::Span;
use chumsky::error::Rich;
use codespan_reporting::diagnostic::{self, Label};
use core::fmt::Display;

#[derive(Debug)]
pub struct Diagnostic(pub DiagnosticType);

#[derive(Debug)]
pub enum DiagnosticType {
    ParserError(ParserError),
}

impl DiagnosticType {
    pub fn report(&self) -> diagnostic::Diagnostic<usize> {
        match self {
            Self::ParserError(parser_error) => diagnostic::Diagnostic::error()
                .with_message(&parser_error.reason)
                .with_label(Label::primary(
                    parser_error.span.context,
                    parser_error.span.into_range(),
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
