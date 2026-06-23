use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    ir::{
        Ident, Span,
        token::{Ctrl, Kw, Token, Tokens},
    },
};
use chumsky::{input::WithContext, prelude::*};

type LexerInput<'src> = WithContext<Span, &'src str>;
type LexerError<'src> = extra::Err<Rich<'src, char, Span>>;

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

fn tokens_parser<'src>() -> impl Parser<'src, LexerInput<'src>, Vec<(Token, Span)>, LexerError<'src>>
{
    recursive(|tokens| {
        let int = text::int(10)
            .to_slice()
            .from_str()
            .unwrapped()
            .map(Token::Int)
            .boxed();

        let kw_ident = text::ascii::ident()
            .map(|ident: &str| match ident {
                "print" => Token::Kw(Kw::Print),
                "fn" => Token::Kw(Kw::Fn),
                "if" => Token::Kw(Kw::If),
                "elif" => Token::Kw(Kw::Elif),
                "else" => Token::Kw(Kw::Else),
                "return" => Token::Kw(Kw::Return),
                "true" => Token::Bool(true),
                "false" => Token::Bool(false),
                _ => Token::Ident(Ident::get_or_intern(ident)),
            })
            .boxed();

        let ctrl_double = choice((
            just("++").to(Ctrl::DoublePlus),
            just("--").to(Ctrl::DoubleMinus),
            just("<<").to(Ctrl::DoubleLt),
            just(">>").to(Ctrl::DoubleGt),
            just("->").to(Ctrl::Arrow),
            just("==").to(Ctrl::DoubleEquals),
            just("!=").to(Ctrl::NotEquals),
            just("<=").to(Ctrl::LessThanEquals),
            just(">=").to(Ctrl::GreaterThanEquals),
            just("::").to(Ctrl::DoubleColon),
            just("&&").to(Ctrl::DoubleAmpersand),
            just("||").to(Ctrl::DoublePipe),
        ));

        let ctrl_single = choice((
            just(';').to(Ctrl::Semicolon),
            just('+').to(Ctrl::Plus),
            just('-').to(Ctrl::Minus),
            just('*').to(Ctrl::Star),
            just('/').to(Ctrl::Slash),
            just('%').to(Ctrl::Percent),
            just('.').to(Ctrl::Dot),
            just('~').to(Ctrl::Tilde),
            just('&').to(Ctrl::Ampersand),
            just('^').to(Ctrl::Caret),
            just('|').to(Ctrl::Pipe),
            just(',').to(Ctrl::Comma),
            just(':').to(Ctrl::Colon),
            just('<').to(Ctrl::LessThan),
            just('>').to(Ctrl::GreaterThan),
            just('=').to(Ctrl::Equals),
            just('!').to(Ctrl::Bang),
        ));

        let ctrl = choice((ctrl_double, ctrl_single)).map(Token::Ctrl).boxed();

        let parentheses = tokens
            .clone()
            .delimited_by(just('('), just(')'))
            .recover_with(via_parser(nested_delimiters(
                '(',
                ')',
                [('{', '}'), ('[', ']')],
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
                [('(', ')'), ('[', ']')],
                |span| vec![(Token::Error, span)],
            )))
            .map(Tokens)
            .map(Token::CurlyBraces)
            .boxed();

        let square_brackets = tokens
            .clone()
            .delimited_by(just('['), just(']'))
            .recover_with(via_parser(nested_delimiters(
                '[',
                ']',
                [('(', ')'), ('{', '}')],
                |span| vec![(Token::Error, span)],
            )))
            .map(Tokens)
            .map(Token::SquareBrackets)
            .boxed();

        let comment = just("//")
            .then(any().and_is(just('\n').not()).repeated())
            .padded()
            .boxed();

        choice((
            int,
            kw_ident,
            ctrl,
            parentheses,
            curly_braces,
            square_brackets,
        ))
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
