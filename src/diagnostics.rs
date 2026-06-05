use crate::span::Span;
use chumsky::error::Rich;
use codespan_reporting::diagnostic::Label;
use core::fmt::Display;

#[salsa::accumulator]
#[derive(Debug)]
pub struct Diagnostic(pub DiagnosticType);

#[derive(Debug)]
pub enum DiagnosticType {
    ParserError(ParserError),
}

impl DiagnosticType {
    pub fn report(&self) -> codespan_reporting::diagnostic::Diagnostic<usize> {
        match self {
            Self::ParserError(parser_error) => {
                let span = parser_error.span.inner();

                codespan_reporting::diagnostic::Diagnostic::error()
                    .with_message(&parser_error.reason)
                    .with_label(Label::primary(span.context.0, span.into_range()))
            }
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
