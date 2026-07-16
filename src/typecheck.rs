use crate::{
    diagnostics::Diagnostic,
    ir::{
        Span, Spanned,
        ast::{Ast, Expression, ModuleExpression},
        typed::{self, TypedExpression},
    },
};
use chumsky::span::SpanWrap as _;
use polytype::{Context, Name};

pub fn typecheck(ast: Ast) -> (typed::Ast, Vec<Diagnostic>) {
    let mut typechecker = Typechecker::default();

    (typechecker.ast(ast), typechecker.diagnostics)
}

#[derive(Debug, Default)]
struct Typechecker {
    diagnostics: Vec<Diagnostic>,
    ctx: Context,
}

impl Typechecker {
    fn ast(&mut self, ast: Ast) -> typed::Ast {
        typed::Ast(
            ast.0
                .into_iter()
                .map(|expr| self.mod_expression(expr))
                .collect(),
        )
    }

    fn mod_expression(
        &mut self,
        mod_expression: Spanned<ModuleExpression>,
    ) -> Spanned<typed::ModuleExpression> {
        map(mod_expression, |mod_expression| match mod_expression {
            ModuleExpression::Expression(expr) => {
                typed::ModuleExpression::Expression(self.expression(expr))
            }
            ModuleExpression::Let { name, params, expr } => todo!(),
        })
    }

    fn expression(&mut self, expression: Spanned<Expression>) -> Spanned<TypedExpression> {
        map(expression, |expression| match expression {
            Expression::Int(value) => TypedExpression {
                expr: typed::Expression::Int(value),
                ty: typed::Type::Int,
            },
            Expression::Bool(value) => TypedExpression {
                expr: typed::Expression::Bool(value),
                ty: typed::Type::Bool,
            },
            Expression::Ident(ident) => {
                let ty = todo!();

                TypedExpression {
                    expr: typed::Expression::Ident(ident),
                    ty,
                }
            }
            Expression::Let {
                name,
                params,
                expr,
                in_,
            } => todo!(),
            Expression::Call { callee, arg } => todo!(),
            Expression::Semicolon(spanned, spanned1) => todo!(),
            Expression::PrefixOp { expr, op } => todo!(),
            Expression::PostfixOp { expr, op } => todo!(),
            Expression::InfixOp { lhs, rhs, op } => todo!(),
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

fn boxed<T>(spanned: Spanned<T>) -> Spanned<Box<T>> {
    map(spanned, Box::new)
}

fn unboxed<T>(spanned: Spanned<Box<T>>) -> Spanned<T> {
    map(spanned, |spanned| *spanned)
}
