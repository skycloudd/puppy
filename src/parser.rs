use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    ir::{
        Ident, Span, Spanned,
        ast::{Ast, Expression, InfixOp, ModuleExpression, PostfixOp, PrefixOp},
        token::{Ctrl, Kw, Token, Tokens},
    },
};
use chumsky::{input::MappedInput, prelude::*};

type ParserInput<'tokens> = MappedInput<'tokens, Token, Span, &'tokens [(Token, Span)]>;
type ParserError<'tokens> = extra::Err<Rich<'tokens, Token, Span>>;

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

fn ast_parser<'tokens>() -> impl Parser<'tokens, ParserInput<'tokens>, Ast, ParserError<'tokens>> {
    mod_expression_parser()
        .spanned()
        .repeated()
        .collect()
        .map(Ast)
        .boxed()
}

fn mod_expression_parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens>, ModuleExpression, ParserError<'tokens>> {
    let let_ = just(Token::Kw(Kw::Let))
        .ignore_then(ident_parser().spanned())
        .then(ident_parser().spanned().repeated().collect())
        .then_ignore(just(Token::Ctrl(Ctrl::Equals)))
        .then(expression_parser().spanned())
        .map(|((name, params), expr)| ModuleExpression::Let { name, params, expr })
        .boxed();

    let expr = expression_parser()
        .spanned()
        .map(ModuleExpression::Expression);

    choice((expr, let_))
}

fn expression_parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens>, Expression, ParserError<'tokens>> {
    recursive(|expression| {
        let parentheses = expression
            .clone()
            .nested_in(select_ref! {
                Token::Parentheses(inner) = e => inner.0.as_slice().split_token_span(e.span())
            })
            .boxed();

        let simple = select! {
            Token::Int(value) => Expression::Int(value),
            Token::Bool(value) => Expression::Bool(value),
            Token::Ident(ident) => Expression::Ident(ident),
        }
        .boxed();

        let let_ = just(Token::Kw(Kw::Let))
            .ignore_then(ident_parser().spanned())
            .then(ident_parser().spanned().repeated().collect())
            .then_ignore(just(Token::Ctrl(Ctrl::Equals)))
            .then(expression.clone().map(Box::new).spanned())
            .then_ignore(just(Token::Kw(Kw::In)))
            .then(expression.map(Box::new).spanned())
            .map(|(((name, params), expr), in_)| Expression::Let {
                name,
                params,
                expr,
                in_,
            })
            .boxed();

        let atom = choice((parentheses, let_, simple))
            .map(Box::new)
            .spanned()
            .boxed();

        let postfix_op = choice((just(Token::Ctrl(Ctrl::Dot))
            .ignore_then(select! { Token::Ident(ident) => ident }.spanned())
            .map(PostfixOp::FieldAccess),))
        .spanned()
        .boxed();

        let postfix = choice((atom.foldl(
            postfix_op.repeated(),
            |expr: Spanned<Box<Expression>>, op: Spanned<PostfixOp>| {
                let span = expr.span.union(op.span);

                Box::new(Expression::PostfixOp { expr, op }).with_span(span)
            },
        ),))
        .boxed();

        let prefix_op = choice((
            just(Token::Ctrl(Ctrl::Plus)).to(PrefixOp::Pos),
            just(Token::Ctrl(Ctrl::Minus)).to(PrefixOp::Neg),
            just(Token::Ctrl(Ctrl::Bang)).to(PrefixOp::LogicalNot),
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

        macro_rules! infix_op {
            ($prev:expr, $($ctrl:expr => $infix_op:expr),+ $(,)?) => {
                {
                    let op = choice(($(just(Token::Ctrl($ctrl)).to($infix_op),)+))
                        .spanned()
                        .boxed();

                    $prev
                        .clone()
                        .foldl(op.then($prev).repeated(), |lhs, (op, rhs)| {
                            let span = lhs.span.union(rhs.span);
                            Box::new(Expression::InfixOp { lhs, rhs, op }).with_span(span)
                        })
                        .boxed()
                }
            };
        }

        let factor = infix_op!(
            prefix,
            Ctrl::Star => InfixOp::Mul,
            Ctrl::Slash => InfixOp::Div,
            Ctrl::Percent => InfixOp::Modulo,
        );

        let sum = infix_op!(
            factor,
            Ctrl::Plus => InfixOp::Add,
            Ctrl::Minus => InfixOp::Sub,
        );

        let bitshift = infix_op!(
            sum,
            Ctrl::DoubleLt => InfixOp::LeftBitshift,
            Ctrl::DoubleGt => InfixOp::RightBitshift,
        );

        let relational_less_greater = infix_op!(
            bitshift,
            Ctrl::LessThan => InfixOp::LessThan,
            Ctrl::GreaterThan => InfixOp::GreaterThan,
            Ctrl::LessThanEquals => InfixOp::LessThanEquals,
            Ctrl::GreaterThanEquals => InfixOp::GreaterThanEquals
        );

        let relational_equal = infix_op!(
            relational_less_greater,
            Ctrl::DoubleEquals => InfixOp::Equal,
            Ctrl::NotEquals => InfixOp::NotEqual,
        );

        let bitwise_and = infix_op!(
            relational_equal,
            Ctrl::Ampersand => InfixOp::BitwiseAnd,
        );

        let bitwise_exclusive_or = infix_op!(
            bitwise_and,
            Ctrl::Caret => InfixOp::BitwiseXor,
        );

        let bitwise_or = infix_op!(
            bitwise_exclusive_or,
            Ctrl::Pipe => InfixOp::BitwiseOr,
        );

        let logical_and = infix_op!(
            bitwise_or,
            Ctrl::DoubleAmpersand => InfixOp::LogicalAnd,
        );

        let logical_or = infix_op!(
            logical_and,
            Ctrl::DoublePipe => InfixOp::LogicalOr,
        );

        let call = logical_or
            .clone()
            .foldl(logical_or.repeated(), |callee, arg| {
                let span = callee.span.union(arg.span);
                Box::new(Expression::Call { callee, arg }).with_span(span)
            })
            .boxed();

        let semicolon = call
            .clone()
            .foldl(
                just(Token::Ctrl(Ctrl::Semicolon))
                    .spanned()
                    .then(call.or_not())
                    .repeated(),
                |lhs, (op, rhs)| {
                    let span = lhs.span.union(rhs.as_ref().map_or(op.span, |rhs| rhs.span));
                    Box::new(Expression::Semicolon(lhs, rhs)).with_span(span)
                },
            )
            .boxed();

        semicolon.map(|expr| *expr.inner)
    })
    .boxed()
}

fn ident_parser<'tokens>() -> impl Parser<'tokens, ParserInput<'tokens>, Ident, ParserError<'tokens>>
{
    select! {
        Token::Ident(ident) => ident
    }
}
