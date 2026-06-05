use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    ir::{SourceProgram, Token, TokenList},
    span::Span,
};
use chumsky::{input::WithContext, prelude::*};
use salsa::Accumulator as _;

#[salsa::tracked]
pub fn lexer(db: &dyn crate::Db, source: SourceProgram) -> Option<TokenList<'_>> {
    let source_text = source.text(db);
    let file_ctx = source.file_ctx(db);

    let (tokens, errors) = tokens_parser()
        .parse(source_text.with_context(file_ctx))
        .into_output_errors();

    for error in errors {
        let diagnostic = Diagnostic(DiagnosticType::ParserError(error.into()));

        diagnostic.accumulate(db);
    }

    tokens.map(|tokens| TokenList::new(db, tokens, file_ctx))
}

fn tokens_parser<'src>() -> impl Parser<
    'src,
    WithContext<Span, &'src str>,
    Vec<(Token, Span)>,
    extra::Err<Rich<'src, char, Span>>,
> {
    let num = text::int(10)
        .then(just('.').then(text::digits(10).or_not()).or_not())
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Token::Number);

    let ident = text::ascii::ident().map(|ident: &str| match ident {
        "fn" => Token::Fn,
        "print" => Token::Print,
        _ => Token::Ident(ident.to_string()),
    });

    let ctrl = choice((just(';').to(Token::Semicolon),));

    let token = choice((num, ident, ctrl));

    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    token
        .map_with(|tok, e| (tok, e.span()))
        .padded_by(comment.repeated())
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
