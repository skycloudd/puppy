use crate::{
    diagnostics::Diagnostic,
    ir::{
        Span, Spanned,
        ast::{Ast, Expression, Statement},
        typed,
    },
};
use chumsky::span::SpanWrap;

pub fn typecheck(ast: Ast) -> (typed::Ast, Vec<Diagnostic>) {
    let mut typechecker = Typechecker::default();

    let statements = typechecker.statements(ast.statements);

    (typed::Ast { statements }, typechecker.diagnostics)
}

#[derive(Debug, Default)]
struct Typechecker {
    diagnostics: Vec<Diagnostic>,
}

impl Typechecker {
    fn statements(
        &mut self,
        statements: Spanned<Vec<Spanned<Statement>>>,
    ) -> Spanned<Vec<Spanned<typed::Statement>>> {
        map(statements, |statements| {
            statements
                .into_iter()
                .map(|statement| self.statement(statement))
                .collect()
        })
    }

    fn statement(&mut self, statement: Spanned<Statement>) -> Spanned<typed::Statement> {
        map(statement, |statement| match statement {
            Statement::Print(expr) => typed::Statement::Print(self.expression(expr)),
            Statement::Function {
                name,
                params,
                return_type,
                body,
                return_expr,
            } => todo!(),
            Statement::Block(statements) => todo!(),
            Statement::Conditional { if_, elifs, else_ } => todo!(),
        })
    }

    fn expression(&mut self, expression: Spanned<Expression>) -> Spanned<typed::TypedExpression> {
        map(expression, |expression| match expression {
            Expression::Int(value) => todo!(),
            Expression::Bool(value) => todo!(),
            Expression::Ident(ident) => todo!(),
            Expression::Path(path) => todo!(),
            Expression::PrefixOp { expr, op } => todo!(),
            Expression::PostfixOp { expr, op } => todo!(),
            Expression::BinaryOp { lhs, rhs, op } => todo!(),
        })
    }
}

fn map<T, U, F>(spanned: Spanned<T>, f: F) -> Spanned<U>
where
    F: FnOnce(T) -> U,
{
    f(spanned.inner).with_span(spanned.span)
}

fn map_with_span<T, U, F>(spanned: Spanned<T>, f: F) -> Spanned<U>
where
    F: FnOnce(T, Span) -> U,
{
    f(spanned.inner, spanned.span).with_span(spanned.span)
}
