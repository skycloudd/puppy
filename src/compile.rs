use crate::{diagnostics::Diagnostic, lexer::lexer, parser::parser, typecheck};

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

    dbg!(&ast);

    if let Some(ast) = ast {
        let (tc_ast, tc_errors) = typecheck::typecheck(ast);

        errors.extend(tc_errors);

        dbg!(tc_ast);
    }

    errors
}
