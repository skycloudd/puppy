use chumsky::{error::Rich, span::SimpleSpan};
use codespan_reporting::diagnostic::Label;
use core::fmt::Display;

#[derive(Debug)]
pub struct Diagnostic(pub DiagnosticType);

#[derive(Debug)]
pub enum DiagnosticType {
    ParserError(ParserError),
}

impl DiagnosticType {
    pub fn report(&self) -> codespan_reporting::diagnostic::Diagnostic<usize> {
        match self {
            Self::ParserError(parser_error) => codespan_reporting::diagnostic::Diagnostic::error()
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
    pub span: SimpleSpan<usize, usize>,
}

impl<'a, T: Display> From<Rich<'a, T, SimpleSpan<usize, usize>>> for ParserError {
    fn from(value: Rich<'a, T, SimpleSpan<usize, usize>>) -> Self {
        Self {
            reason: value.reason().to_string(),
            span: *value.span(),
        }
    }
}
