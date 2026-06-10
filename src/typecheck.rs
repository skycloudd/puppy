use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    ir::{
        Ident, Span, Spanned,
        ast::{Ast, Expression, Statement},
        typed::{self, Type, TypedExpression},
    },
};
use chumsky::span::SpanWrap as _;
use core::hash::Hash;
use rustc_hash::FxHashMap;

pub fn typecheck(ast: Ast) -> (typed::Ast, Vec<Diagnostic>) {
    let mut typechecker = Typechecker::default();

    (typechecker.ast(ast), typechecker.diagnostics)
}

#[derive(Debug, Default)]
struct Typechecker {
    diagnostics: Vec<Diagnostic>,
    scopes: Scopes<Ident, Type>,
}

impl Typechecker {
    fn ast(&mut self, ast: Ast) -> typed::Ast {
        self.scopes.push_empty();
        let statements = self.statements(ast.statements);
        self.scopes.pop();

        typed::Ast { statements }
    }

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
            Statement::Block(statements) => {
                self.scopes.push_empty();
                let statements = self.statements(statements);
                self.scopes.pop();

                typed::Statement::Block(statements)
            }
            Statement::Conditional { if_, elifs, else_ } => todo!(),
        })
    }

    fn expression(&mut self, expression: Spanned<Expression>) -> Spanned<TypedExpression> {
        map_with_span(expression, |expression, span| match expression {
            Expression::Int(value) => TypedExpression {
                expr: typed::Expression::Int(value),
                ty: Type::Int,
            },
            Expression::Bool(value) => TypedExpression {
                expr: typed::Expression::Bool(value),
                ty: Type::Bool,
            },
            Expression::Ident(ident) => {
                let ty = self.scopes.get(&ident).cloned().unwrap_or_else(|| {
                    self.diagnostics
                        .push(Diagnostic(DiagnosticType::UndefinedName {
                            name: ident.resolve(),
                            span,
                        }));

                    Type::Error
                });

                TypedExpression {
                    expr: typed::Expression::Ident(ident),
                    ty,
                }
            }
            Expression::Path(path) => TypedExpression {
                expr: typed::Expression::Path(path),
                ty: todo!(),
            },
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

#[derive(Debug)]
struct Scopes<K: Eq + Hash, V> {
    base: FxHashMap<K, V>,
    rest: Vec<FxHashMap<K, V>>,
}

impl<K: Eq + Hash, V> Scopes<K, V> {
    fn get(&self, k: &K) -> Option<&V> {
        self.rest
            .iter()
            .rev()
            .find_map(|scope| scope.get(k))
            .or_else(|| self.base.get(k))
    }

    fn insert(&mut self, k: K, v: V) {
        self.rest.last_mut().unwrap_or(&mut self.base).insert(k, v);
    }

    fn push_empty(&mut self) {
        self.rest.push(FxHashMap::default());
    }

    fn pop(&mut self) {
        self.rest.pop().unwrap();
    }
}

impl<K: Eq + Hash, V> Default for Scopes<K, V> {
    fn default() -> Self {
        Self {
            base: FxHashMap::default(),
            rest: vec![],
        }
    }
}
