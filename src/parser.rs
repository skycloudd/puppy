use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    ir::{
        ast::{Ast, BinaryOp, Expression, Statement, UnaryOp},
        token::{Ctrl, Kw, Token, Tokens},
    },
};
use chumsky::{input::MappedInput, prelude::*};

pub fn parser(tokens: &Tokens) -> (Option<Ast>, Vec<Diagnostic>) {
    let eoi = tokens
        .0
        .last()
        .map_or_else(|| SimpleSpan::new(0, 0..0), |(_, span)| span.to_end());

    let (ast, errors) = ast_parser()
        .parse(tokens.0.split_token_span(eoi))
        .into_output_errors();

    (
        ast,
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

fn ast_parser<'tokens>() -> impl Parser<'tokens, ParserInput<'tokens>, Ast, ParserError<'tokens>> {
    statement_parser()
        .spanned()
        .repeated()
        .collect()
        .spanned()
        .map(Ast::new)
}

fn statement_parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens>, Statement, ParserError<'tokens>> {
    let print = just(Token::Kw(Kw::Print))
        .ignore_then(expression_parser().spanned())
        .then_ignore(just(Token::Ctrl(Ctrl::Semicolon)))
        .map(Statement::Print);

    choice((print,))
}

fn expression_parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens>, Expression, ParserError<'tokens>> {
    recursive(|expr| {
        let parentheses = expr.nested_in(select_ref! {
            Token::Parentheses(inner) = e => inner.0.as_slice().split_token_span(e.span())
        });

        let simple = select! {
            Token::Ident(ident) = e => Expression::Ident(ident),
            Token::Number(n) => Expression::Number(n)
        };

        let atom = choice((parentheses, simple)).boxed();

        let unary_op = choice((just(Token::Ctrl(Ctrl::Minus)).to(UnaryOp::Neg),)).spanned();

        let unary = unary_op
            .repeated()
            .foldr(
                atom.map(Box::new).spanned(),
                |op: Spanned<UnaryOp, SimpleSpan<usize, usize>>,
                 expr: Spanned<Box<Expression>, SimpleSpan<usize, usize>>| {
                    let span = op.span.union(expr.span);

                    Box::new(Expression::UnaryOp { rhs: expr, op }).with_span(span)
                },
            )
            .boxed();

        let factor_op = choice((
            just(Token::Ctrl(Ctrl::Star)).to(BinaryOp::Mul),
            just(Token::Ctrl(Ctrl::Slash)).to(BinaryOp::Div),
        ))
        .spanned();

        let factor = unary
            .clone()
            .foldl(factor_op.then(unary).repeated(), |lhs, (op, rhs)| {
                let span = lhs.span.union(rhs.span);

                Box::new(Expression::BinaryOp { lhs, rhs, op }).with_span(span)
            })
            .boxed();

        let sum_op = choice((
            just(Token::Ctrl(Ctrl::Plus)).to(BinaryOp::Add),
            just(Token::Ctrl(Ctrl::Minus)).to(BinaryOp::Sub),
        ))
        .spanned();

        let sum = factor
            .clone()
            .foldl(sum_op.then(factor).repeated(), |lhs, (op, rhs)| {
                let span = lhs.span.union(rhs.span);

                Box::new(Expression::BinaryOp { lhs, rhs, op }).with_span(span)
            })
            .boxed();

        sum.map(|s| *s.inner)
    })
}
