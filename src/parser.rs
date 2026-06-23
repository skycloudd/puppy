use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    ir::{
        Span, Spanned,
        ast::{Ast, BinaryOp, Expression, ParsedType, Path, PostfixOp, PrefixOp, Statement},
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
    statement_parser()
        .spanned()
        .repeated()
        .collect()
        .spanned()
        .map(|statements| Ast { statements })
        .boxed()
}

fn statement_parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens>, Statement, ParserError<'tokens>> {
    recursive(|statement| {
        let print = just(Token::Kw(Kw::Print))
            .ignore_then(expression_parser().spanned())
            .then_ignore(just(Token::Ctrl(Ctrl::Semicolon)))
            .map(Statement::Print)
            .boxed();

        let function = function_parser(statement.clone()).boxed();

        let block = statement
            .clone()
            .spanned()
            .repeated()
            .collect()
            .nested_in(select_ref! {
                Token::CurlyBraces(inner) = e => inner.0.as_slice().split_token_span(e.span())
            })
            .spanned()
            .map(Statement::Block)
            .boxed();

        let conditional = conditional_parser(statement).boxed();

        let return_ = just(Token::Kw(Kw::Return))
            .ignore_then(expression_parser().spanned().or_not())
            .then_ignore(just(Token::Ctrl(Ctrl::Semicolon)))
            .map(Statement::Return)
            .boxed();

        choice((print, function, block, conditional, return_)).boxed()
    })
}

fn function_parser<'tokens>(
    statement: impl Parser<'tokens, ParserInput<'tokens>, Statement, ParserError<'tokens>> + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens>, Statement, ParserError<'tokens>> {
    let ident = select! { Token::Ident(ident) => ident }.spanned();

    let params = ident
        .then_ignore(just(Token::Ctrl(Ctrl::Colon)))
        .then(type_parser().spanned())
        .separated_by(just(Token::Ctrl(Ctrl::Comma)))
        .allow_trailing()
        .collect()
        .nested_in(select_ref! {
            Token::Parentheses(inner) = e => inner.0.as_slice().split_token_span(e.span())
        })
        .spanned()
        .boxed();

    let body = statement
        .spanned()
        .repeated()
        .collect()
        .spanned()
        .nested_in(select_ref! {
            Token::CurlyBraces(inner) = e => inner.0.as_slice().split_token_span(e.span())
        })
        .boxed();

    just(Token::Kw(Kw::Fn))
        .ignore_then(ident)
        .then(params)
        .then(
            just(Token::Ctrl(Ctrl::Arrow))
                .ignore_then(type_parser().spanned())
                .or_not(),
        )
        .then(body)
        .map(
            |(((name, params), return_type), body)| Statement::Function {
                name,
                params,
                return_type,
                body,
            },
        )
        .boxed()
}

fn conditional_parser<'tokens>(
    statement: impl Parser<'tokens, ParserInput<'tokens>, Statement, ParserError<'tokens>>
    + Clone
    + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens>, Statement, ParserError<'tokens>> {
    let block = statement
        .spanned()
        .repeated()
        .collect()
        .nested_in(select_ref! {
            Token::CurlyBraces(inner) = e => inner.0.as_slice().split_token_span(e.span())
        })
        .spanned()
        .boxed();

    just(Token::Kw(Kw::If))
        .ignore_then(expression_parser().spanned())
        .then(block.clone())
        .map(|(condition, block)| (condition, block))
        .spanned()
        .then(
            just(Token::Kw(Kw::Elif))
                .ignore_then(expression_parser().spanned())
                .then(block.clone())
                .map(|(condition, block)| (condition, block))
                .spanned()
                .repeated()
                .collect(),
        )
        .then(just(Token::Kw(Kw::Else)).ignore_then(block).or_not())
        .map(|((if_, elifs), else_)| Statement::Conditional { if_, elifs, else_ })
        .boxed()
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

        let path = path_parser().map(Expression::Path);

        let simple = select! {
            Token::Int(value) => Expression::Int(value),
            Token::Bool(value) => Expression::Bool(value),
        }
        .boxed();

        let atom = choice((parentheses, simple, path)).boxed();

        let postfix_op = choice((
            just(Token::Ctrl(Ctrl::DoublePlus)).to(PostfixOp::Inc),
            just(Token::Ctrl(Ctrl::DoubleMinus)).to(PostfixOp::Dec),
            just(Token::Ctrl(Ctrl::Dot))
                .ignore_then(select! { Token::Ident(ident) => ident }.spanned())
                .map(PostfixOp::FieldAccess),
            expression
                .spanned()
                .separated_by(just(Token::Ctrl(Ctrl::Comma)))
                .allow_trailing()
                .collect()
                .nested_in(select_ref! {
                    Token::Parentheses(inner) = e => inner.0.as_slice().split_token_span(e.span())
                })
                .spanned()
                .map(PostfixOp::FunctionCall),
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

        macro_rules! binary_op {
            ($prev:expr, $($ctrl:expr => $bin_op:expr),+ $(,)?) => {
                {
                    let op = choice(($(just(Token::Ctrl($ctrl)).to($bin_op),)+))
                        .spanned()
                        .boxed();

                    $prev
                        .clone()
                        .foldl(op.then($prev).repeated(), |lhs, (op, rhs)| {
                            let span = lhs.span.union(rhs.span);
                            Box::new(Expression::BinaryOp { lhs, rhs, op }).with_span(span)
                        })
                        .boxed()
                }
            };
        }

        let factor = binary_op!(
            prefix,
            Ctrl::Star => BinaryOp::Mul,
            Ctrl::Slash => BinaryOp::Div,
            Ctrl::Percent => BinaryOp::Modulo,
        );

        let sum = binary_op!(
            factor,
            Ctrl::Plus => BinaryOp::Add,
            Ctrl::Minus => BinaryOp::Sub,
        );

        let bitshift = binary_op!(
            sum,
            Ctrl::DoubleLt => BinaryOp::LeftBitshift,
            Ctrl::DoubleGt => BinaryOp::RightBitshift,
        );

        let relational_less_greater = binary_op!(
            bitshift,
            Ctrl::LessThan => BinaryOp::LessThan,
            Ctrl::GreaterThan => BinaryOp::GreaterThan,
            Ctrl::LessThanEquals => BinaryOp::LessThanEquals,
            Ctrl::GreaterThanEquals => BinaryOp::GreaterThanEquals
        );

        let relational_equal = binary_op!(
            relational_less_greater,
            Ctrl::DoubleEquals => BinaryOp::Equal,
            Ctrl::NotEquals => BinaryOp::NotEqual,
        );

        let bitwise_and = binary_op!(
            relational_equal,
            Ctrl::Ampersand => BinaryOp::BitwiseAnd,
        );

        let bitwise_exclusive_or = binary_op!(
            bitwise_and,
            Ctrl::Caret => BinaryOp::BitwiseXor,
        );

        let bitwise_or = binary_op!(
            bitwise_exclusive_or,
            Ctrl::Pipe => BinaryOp::BitwiseOr,
        );

        let logical_and = binary_op!(
            bitwise_or,
            Ctrl::DoubleAmpersand => BinaryOp::LogicalAnd,
        );

        let logical_or = binary_op!(
            logical_and,
            Ctrl::DoublePipe => BinaryOp::LogicalOr,
        );

        logical_or.map(|expr| *expr.inner).boxed()
    })
    .boxed()
}

fn path_parser<'tokens>() -> impl Parser<'tokens, ParserInput<'tokens>, Path, ParserError<'tokens>>
{
    select! { Token::Ident(ident) => ident }
        .spanned()
        .separated_by(just(Token::Ctrl(Ctrl::DoubleColon)))
        .at_least(1)
        .collect()
        .map(Path)
}

fn type_parser<'tokens>()
-> impl Parser<'tokens, ParserInput<'tokens>, ParsedType, ParserError<'tokens>> {
    let path = path_parser().map(ParsedType::Path);

    let function = todo();

    choice((path, function))
}
