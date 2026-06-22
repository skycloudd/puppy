use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    ir::{
        Ident, Span, Spanned,
        ast::{Ast, Expression, Statement},
        typed::{self, FunctionType, Type, TypedExpression},
    },
};
use chumsky::span::SpanWrap as _;
use slab_tree::{NodeId, Tree, TreeBuilder};

pub fn typecheck(ast: Ast) -> (typed::Ast, Vec<Diagnostic>) {
    let mut typechecker = Typechecker::new();

    (typechecker.ast(ast), typechecker.diagnostics)
}

#[derive(Debug)]
struct Typechecker {
    diagnostics: Vec<Diagnostic>,
    item_tree: Tree<Item>,
    current_node: NodeId,
}

impl Typechecker {
    fn new() -> Self {
        let mut item_tree = Tree::new();
        let current_node = item_tree.set_root(Item::Global);

        Self {
            diagnostics: vec![],
            item_tree,
            current_node,
        }
    }

    fn ast(&mut self, ast: Ast) -> typed::Ast {
        let statements = self.statements(ast.statements);

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
                self.current_node = self.new_node_in(self.current_node, Item::Block);

                let statements = self.statements(statements);

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
            Expression::Path(path) => {
                let mut current_node = None;
                let mut current_type = Type::Error;

                for component in &path.0 {
                    let resolve_in = current_node.unwrap_or(self.current_node);

                    if let Some((node, ty)) = self.resolve_ident_in(component.resolve(), resolve_in)
                    {
                        current_node = Some(node);
                        current_type = ty;
                    } else {
                        self.diagnostics
                            .push(Diagnostic(DiagnosticType::UndefinedName {
                                name: component.resolve(),
                                span: component.span,
                            }));

                        current_type = Type::Error;
                        break;
                    }
                }

                TypedExpression {
                    expr: typed::Expression::Path(path),
                    ty: current_type,
                }
            }
            Expression::PrefixOp { expr, op } => todo!(),
            Expression::PostfixOp { expr, op } => todo!(),
            Expression::BinaryOp { lhs, rhs, op } => todo!(),
        })
    }

    fn new_node_in(&mut self, parent: NodeId, ty: Item) -> NodeId {
        self.item_tree.get_mut(parent).unwrap().append(ty).node_id()
    }

    fn resolve_ident_in(&self, ident: &'static str, in_: NodeId) -> Option<(NodeId, Type)> {
        for child in self.item_tree.get(in_).unwrap().children() {
            match child.data() {
                Item::Global | Item::Block => {}
                Item::Named { name, ty } => {
                    if *name == ident {
                        return Some((child.node_id(), ty.clone()));
                    }
                }
            }
        }

        None
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
enum Item {
    Global,
    Block,
    Named { name: &'static str, ty: Type },
}
