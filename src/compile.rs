use crate::{diagnostics::Diagnostic, lexer::lexer, parser::parser};
use ptree::print_tree;

pub fn compile(source: &str, file_id: usize) -> Vec<Diagnostic> {
    let mut errors = vec![];

    let (tokens, lexer_errors) = lexer(source, file_id);

    errors.extend(lexer_errors);

    let ast = tokens
        .as_ref()
        .map(parser)
        .and_then(|(ast, parser_errors)| {
            errors.extend(parser_errors);
            ast
        });

    if let Some(ast) = ast.as_ref() {
        print_tree(&ast).unwrap();
    }

    errors
}
