use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    ir::{
        ast::{Expression, Program, Statement},
        token::{Ctrl, Kw, Token, Tokens},
    },
};
use chumsky::{input::MappedInput, prelude::*};

pub fn parser(tokens: &Tokens) -> (Option<Program>, Vec<Diagnostic>) {
    let eoi = tokens
        .0
        .last()
        .map_or_else(|| SimpleSpan::new(0, 0..0), |(_, span)| span.to_end());

    let (program, errors) = program_parser()
        .parse(tokens.0.split_token_span(eoi))
        .into_output_errors();

    (
        program,
        errors
            .into_iter()
            .map(|error| Diagnostic(DiagnosticType::ParserError(error.into())))
            .collect(),
    )
}

type ParserInput<'tokens> = MappedInput<
    'tokens,
    Token,
    SimpleSpan<usize, usize>,
    &'tokens [(Token, SimpleSpan<usize, usize>)],
>;
type ParserError<'tokens> = extra::Err<Rich<'tokens, Token, SimpleSpan<usize, usize>>>;

fn program_parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens>, Program, ParserError<'tokens>> {
    statement_parser().repeated().collect().map(Program::new)
}

fn statement_parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens>, Statement, ParserError<'tokens>> {
    let print = just(Token::Kw(Kw::Print))
        .ignore_then(expression_parser())
        .then_ignore(just(Token::Ctrl(Ctrl::Semicolon)))
        .map(Statement::Print);

    choice((print,))
}

fn expression_parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens>, Expression, ParserError<'tokens>> {
    recursive(|expr| {
        let simple = select! {
            Token::Ident(ident) = e => Expression::Ident(ident),
            Token::Number(n) => Expression::Number(n)
        };

        let parentheses = expr.nested_in(select_ref! {
            Token::Parentheses(inner) = e => inner.0.as_slice().split_token_span(e.span())
        });

        choice((simple, parentheses))
    })
    .boxed()
}
