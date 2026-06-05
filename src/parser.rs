use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    ir::{
        ast::{Expression, ExpressionData, Ident, Program, Statement, StatementData},
        token::{Ctrl, Kw, Token, Tokens},
    },
};
use chumsky::{input::MappedInput, prelude::*};
use salsa::Accumulator as _;

#[salsa::tracked]
pub fn parser<'db>(db: &'db dyn crate::Db, tokens: Tokens<'db>) -> Option<Program<'db>> {
    let tokens = tokens.tokens(db);

    let eoi = tokens
        .last()
        .map_or_else(|| SimpleSpan::new(0, 0..0), |(_, span)| span.to_end());

    let awa = tokens.split_token_span(eoi);

    let (program, errors) = program_parser(db).parse(awa).into_output_errors();

    for error in errors {
        let diagnostic = Diagnostic(DiagnosticType::ParserError(error.into()));

        diagnostic.accumulate(db);
    }

    program
}

type ParserInput<'a> =
    MappedInput<'a, Token, SimpleSpan<usize, usize>, &'a [(Token, SimpleSpan<usize, usize>)]>;
type ParserError<'a> = extra::Err<Rich<'a, Token, SimpleSpan<usize, usize>>>;

fn program_parser<'a: 'db, 'db: 'a>(
    db: &'db dyn crate::Db,
) -> impl Parser<'a, ParserInput<'a>, Program<'db>, ParserError<'a>> {
    statement_parser(db)
        .repeated()
        .collect()
        .map(|stmts| Program::new(db, stmts))
}

fn statement_parser<'a: 'db, 'db: 'a>(
    db: &'db dyn crate::Db,
) -> impl Parser<'a, ParserInput<'a>, Statement<'db>, ParserError<'a>> {
    let print = just(Token::Kw(Kw::Print))
        .ignore_then(expression_parser(db))
        .map(StatementData::Print);

    choice((print,))
        .then_ignore(just(Token::Ctrl(Ctrl::Semicolon)))
        .map_with(|data, e| Statement::new(data, e.span()))
}

fn expression_parser<'a: 'db, 'db: 'a>(
    db: &'db dyn crate::Db,
) -> impl Parser<'a, ParserInput<'a>, Expression<'db>, ParserError<'a>> {
    recursive(|expr| {
        let simple = select! {
            Token::Ident(ident) = e => ExpressionData::Ident(Ident::new(db, ident, e.span())),
            Token::Number(n) => ExpressionData::Number(n)
        };

        let parentheses = expr.nested_in(select_ref! {
            Token::Parentheses(inner) = e => inner.as_slice().split_token_span(e.span())
        });

        choice((simple, parentheses))
    })
    .map_with(|data, e| Expression::new(data, e.span()))
    .boxed()
}
