use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    ir::{
        Ident, Span,
        token::{Ctrl, Kw, Token, Tokens},
    },
};
use chumsky::{input::WithContext, prelude::*};

pub fn lexer(source: &str, file_id: usize) -> (Option<Tokens>, Vec<Diagnostic>) {
    let (tokens, errors) = tokens_parser()
        .parse(source.with_context(file_id))
        .into_output_errors();

    (
        tokens.map(Tokens),
        errors
            .into_iter()
            .map(|error| Diagnostic(DiagnosticType::ParserError(error.into())))
            .collect(),
    )
}

type LexerInput<'src> = WithContext<Span, &'src str>;
type LexerError<'src> = extra::Err<Rich<'src, char, Span>>;

fn tokens_parser<'src>() -> impl Parser<'src, LexerInput<'src>, Vec<(Token, Span)>, LexerError<'src>>
{
    recursive(|tokens| {
        let num = text::int(10)
            .then(just('.').then(text::digits(10).or_not()).or_not())
            .to_slice()
            .from_str()
            .unwrapped()
            .map(Token::Number)
            .boxed();

        let kw_ident = text::ascii::ident()
            .map(|ident: &str| match ident {
                "print" => Token::Kw(Kw::Print),
                _ => Token::Ident(Ident::get_or_intern(ident)),
            })
            .boxed();

        let ctrl = choice((
            just("++").to(Ctrl::DoublePlus),
            just("--").to(Ctrl::DoubleMinus),
            just("<<").to(Ctrl::DoubleLt),
            just(">>").to(Ctrl::DoubleGt),
            just(';').to(Ctrl::Semicolon),
            just('+').to(Ctrl::Plus),
            just('-').to(Ctrl::Minus),
            just('*').to(Ctrl::Star),
            just('/').to(Ctrl::Slash),
            just('%').to(Ctrl::Percent),
            just('.').to(Ctrl::Dot),
            just('~').to(Ctrl::Tilde),
        ))
        .map(Token::Ctrl)
        .boxed();

        let parentheses = tokens
            .clone()
            .delimited_by(just('('), just(')'))
            .recover_with(via_parser(nested_delimiters(
                '(',
                ')',
                [('{', '}')],
                |span| vec![(Token::Error, span)],
            )))
            .map(Tokens)
            .map(Token::Parentheses)
            .boxed();

        let curly_braces = tokens
            .clone()
            .delimited_by(just('{'), just('}'))
            .recover_with(via_parser(nested_delimiters(
                '{',
                '}',
                [('(', ')')],
                |span| vec![(Token::Error, span)],
            )))
            .map(Tokens)
            .map(Token::CurlyBraces)
            .boxed();

        let comment = just("//")
            .then(any().and_is(just('\n').not()).repeated())
            .padded()
            .boxed();

        choice((num, kw_ident, ctrl, parentheses, curly_braces))
            .map_with(|tok, e| (tok, e.span()))
            .padded_by(comment.repeated())
            .padded()
            .repeated()
            .collect()
            .boxed()
    })
    .then_ignore(end())
    .boxed()
}
