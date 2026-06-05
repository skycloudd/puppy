use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    ir::{
        SourceProgram,
        token::{Ctrl, Kw, Token, Tokens},
    },
};
use chumsky::{input::WithContext, prelude::*};
use salsa::Accumulator as _;

#[salsa::tracked]
pub fn lexer(db: &dyn crate::Db, source: SourceProgram) -> Option<Tokens<'_>> {
    let source_text = source.text(db);
    let file_ctx = source.file_ctx(db);

    let (tokens, errors) = tokens_parser()
        .parse(source_text.with_context(file_ctx))
        .into_output_errors();

    for error in errors {
        let diagnostic = Diagnostic(DiagnosticType::ParserError(error.into()));

        diagnostic.accumulate(db);
    }

    tokens.map(|tokens| Tokens::new(db, tokens))
}

type LexerInput<'src> = WithContext<SimpleSpan<usize, usize>, &'src str>;
type LexerError<'src> = extra::Err<Rich<'src, char, SimpleSpan<usize, usize>>>;

fn tokens_parser<'src>()
-> impl Parser<'src, LexerInput<'src>, Vec<(Token, SimpleSpan<usize, usize>)>, LexerError<'src>> {
    recursive(|tokens| {
        let num = text::int(10)
            .then(just('.').then(text::digits(10).or_not()).or_not())
            .to_slice()
            .from_str()
            .unwrapped()
            .map(Token::Number);

        let kw_ident = text::ascii::ident().map(|ident: &str| match ident {
            "print" => Token::Kw(Kw::Print),
            _ => Token::Ident(ident.to_string()),
        });

        let ctrl = choice((just(';').to(Ctrl::Semicolon),)).map(Token::Ctrl);

        let parentheses = tokens
            .clone()
            .delimited_by(just('('), just(')'))
            .recover_with(via_parser(nested_delimiters(
                '(',
                ')',
                [('{', '}')],
                |span| vec![(Token::Error, span)],
            )))
            .map(Token::Parentheses);

        let comment = just("//")
            .then(any().and_is(just('\n').not()).repeated())
            .padded()
            .labelled("comment");

        choice((parentheses, kw_ident, num, ctrl))
            .map_with(|tok, e| (tok, e.span()))
            .padded_by(comment.repeated())
            .padded()
            .repeated()
            .collect()
    })
    .then_ignore(end())
}
