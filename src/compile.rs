use crate::{diagnostics::Diagnostic, lexer::lexer, parser::parser, typecheck::typecheck};

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

    if let Some(ast) = ast {
        let (typed_ast, typecheck_errors) = typecheck(ast);

        errors.extend(typecheck_errors);

        dbg!(typed_ast);
    }

    errors
}
