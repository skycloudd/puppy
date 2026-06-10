use crate::ir::Span;
use chumsky::error::Rich;
use codespan_reporting::diagnostic::{self, Label};
use core::fmt::Display;

#[derive(Debug)]
pub struct Diagnostic(pub DiagnosticType);

#[derive(Debug)]
pub enum DiagnosticType {
    ParserError(ParserError),
    UndefinedName { name: &'static str, span: Span },
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
            Self::UndefinedName { name, span } => diagnostic::Diagnostic::error()
                .with_message(format!("Undefined variable '{name}'"))
                .with_label(
                    Label::primary(span.context, span.into_range()).with_message("found here"),
                ),
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
