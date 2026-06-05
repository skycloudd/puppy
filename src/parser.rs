use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    ir::{
        Expression, ExpressionData, Program, Statement, StatementData, Token, TokenList, VariableId,
    },
    span::Span,
};
use chumsky::{input::ValueInput, prelude::*};
use salsa::Accumulator as _;

#[salsa::tracked]
pub fn parser<'db>(db: &'db dyn crate::Db, token_list: TokenList<'db>) -> Option<Program<'db>> {
    let tokens = token_list.tokens(db);

    let eoi = tokens.last().map_or_else(
        || Span::new(token_list.file_ctx(db), 0..0),
        |(_, span)| span.to_end(),
    );

    let (program, errors) = program_parser(db)
        .parse(tokens.split_token_span(eoi))
        .into_output_errors();

    for error in errors {
        let diagnostic = Diagnostic(DiagnosticType::ParserError(error.into()));

        diagnostic.accumulate(db);
    }

    program
}

fn program_parser<'a, 'db, I>(
    db: &'db dyn crate::Db,
) -> impl Parser<'a, I, Program<'db>, extra::Err<Rich<'a, Token, Span>>>
where
    I: ValueInput<'a, Token = Token, Span = Span>,
{
    statement_parser(db)
        .repeated()
        .collect()
        .map(|stmts| Program::new(db, stmts))
}

fn statement_parser<'a, 'db, I>(
    db: &'db dyn crate::Db,
) -> impl Parser<'a, I, Statement<'db>, extra::Err<Rich<'a, Token, Span>>>
where
    I: ValueInput<'a, Token = Token, Span = Span>,
{
    choice((print_statement_parser(db),))
        .then_ignore(just(Token::Semicolon))
        .map_with(|data, e| Statement::new(data, e.span()))
}

fn print_statement_parser<'a, 'db, I>(
    db: &'db dyn crate::Db,
) -> impl Parser<'a, I, StatementData<'db>, extra::Err<Rich<'a, Token, Span>>>
where
    I: ValueInput<'a, Token = Token, Span = Span>,
{
    just(Token::Print)
        .ignore_then(expression_parser(db))
        .map(StatementData::Print)
}

fn expression_parser<'a, 'db, I>(
    db: &'db dyn crate::Db,
) -> impl Parser<'a, I, Expression<'db>, extra::Err<Rich<'a, Token, Span>>>
where
    I: ValueInput<'a, Token = Token, Span = Span>,
{
    let val = select! {
        Token::Number(n) => ExpressionData::Number(n),
        Token::Ident(ident) = e => ExpressionData::Variable(VariableId::new(db,ident,e.span()))
    };

    val.map_with(|data, e| Expression::new(data, e.span()))
}
