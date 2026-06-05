use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    ir::{
        Span, Spanned,
        ast::{Ast, BinaryOp, Expression, PostfixOp, PrefixOp, Statement},
        token::{Ctrl, Kw, Token, Tokens},
    },
};
use chumsky::{input::MappedInput, prelude::*};

pub fn parser(tokens: &Tokens) -> (Option<Ast>, Vec<Diagnostic>) {
    let eoi = tokens
        .0
        .last()
        .map_or_else(|| Span::new(0, 0..0), |(_, span)| span.to_end());

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

type ParserInput<'tokens> = MappedInput<'tokens, Token, Span, &'tokens [(Token, Span)]>;
type ParserError<'tokens> = extra::Err<Rich<'tokens, Token, Span>>;

fn ast_parser<'tokens>() -> impl Parser<'tokens, ParserInput<'tokens>, Ast, ParserError<'tokens>> {
    statement_parser()
        .spanned()
        .repeated()
        .collect()
        .spanned()
        .map(Ast::new)
        .boxed()
}

fn statement_parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens>, Statement, ParserError<'tokens>> {
    let print = just(Token::Kw(Kw::Print))
        .ignore_then(expression_parser().spanned())
        .then_ignore(just(Token::Ctrl(Ctrl::Semicolon)))
        .map(Statement::Print)
        .boxed();

    choice((print,)).boxed()
}

fn expression_parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens>, Expression, ParserError<'tokens>> {
    recursive(|expr| {
        let parentheses = expr
            .clone()
            .nested_in(select_ref! {
                Token::Parentheses(inner) = e => inner.0.as_slice().split_token_span(e.span())
            })
            .boxed();

        let simple = select! {
            Token::Ident(ident) = e => Expression::Ident(ident),
            Token::Number(n) => Expression::Number(n)
        }
        .boxed();

        let atom = choice((parentheses, simple)).boxed();

        let postfix_op = choice((
            just(Token::Ctrl(Ctrl::DoublePlus)).to(PostfixOp::Inc),
            just(Token::Ctrl(Ctrl::DoubleMinus)).to(PostfixOp::Dec),
            just(Token::Ctrl(Ctrl::Dot))
                .ignore_then(select! { Token::Ident(ident) => ident })
                .map(PostfixOp::FieldAccess),
        ))
        .spanned()
        .boxed();

        let postfix = atom
            .map(Box::new)
            .spanned()
            .foldl(
                postfix_op.repeated(),
                |expr: Spanned<Box<Expression>>, op: Spanned<PostfixOp>| {
                    let span = expr.span.union(op.span);

                    Box::new(Expression::PostfixOp { expr, op }).with_span(span)
                },
            )
            .boxed();

        let prefix_op = choice((
            just(Token::Ctrl(Ctrl::DoublePlus)).to(PrefixOp::Inc),
            just(Token::Ctrl(Ctrl::DoubleMinus)).to(PrefixOp::Dec),
            just(Token::Ctrl(Ctrl::Plus)).to(PrefixOp::Pos),
            just(Token::Ctrl(Ctrl::Minus)).to(PrefixOp::Neg),
            just(Token::Ctrl(Ctrl::Tilde)).to(PrefixOp::BitwiseNot),
        ))
        .spanned()
        .boxed();

        let prefix = prefix_op
            .repeated()
            .foldr(
                postfix,
                |op: Spanned<PrefixOp>, expr: Spanned<Box<Expression>>| {
                    let span = op.span.union(expr.span);

                    Box::new(Expression::PrefixOp { expr, op }).with_span(span)
                },
            )
            .boxed();

        macro_rules! binary_op {
            ($name:ident, $prev:expr, $($ctrl:expr => $bin_op:expr),+ $(,)?) => {
                let $name = {
                    let op = choice(($(just(Token::Ctrl($ctrl)).to($bin_op),)+))
                        .spanned()
                        .boxed()
                        .boxed();

                    $prev
                        .clone()
                        .foldl(op.then($prev).repeated(), |lhs, (op, rhs)| {
                            let span = lhs.span.union(rhs.span);
                            Box::new(Expression::BinaryOp { lhs, rhs, op }).with_span(span)
                        })
                        .boxed()
                };
            };
        }

        binary_op!(
            factor,
            prefix,
            Ctrl::Star => BinaryOp::Mul,
            Ctrl::Slash => BinaryOp::Div,
            Ctrl::Percent => BinaryOp::Modulo,
        );

        binary_op!(
            sum,
            factor,
            Ctrl::Plus => BinaryOp::Add,
            Ctrl::Minus => BinaryOp::Sub,
        );

        binary_op!(
            bitshift,
            sum,
            Ctrl::DoubleLt => BinaryOp::LeftBitshift,
            Ctrl::DoubleGt => BinaryOp::RightBitshift,
        );

        binary_op!(
            bitwise_and,
            bitshift,
            Ctrl::Ampersand => BinaryOp::BitwiseAnd,
        );

        binary_op!(
            bitwise_xor,
            bitwise_and,
            Ctrl::Caret => BinaryOp::BitwiseXor,
        );

        binary_op!(
            bitwise_or,
            bitwise_xor,
            Ctrl::Pipe => BinaryOp::BitwiseOr,
        );

        bitwise_or.map(|s| *s.inner).boxed()
    })
    .boxed()
}
