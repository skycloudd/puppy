use crate::{diagnostics::Diagnostic, lexer::lexer, parser::parser};

pub fn compile(source: &str, file_id: usize) -> Vec<Diagnostic> {
    let mut errors = vec![];

    let (tokens, lexer_errors) = lexer(source, file_id);

    errors.extend(lexer_errors);

    if let Some((program, parser_errors)) = tokens.as_ref().map(parser) {
        errors.extend(parser_errors);
    }

    errors
}
