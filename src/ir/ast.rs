use crate::ir::{Ident, Spanned};
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct Ast(pub Vec<Spanned<ModuleExpression>>);

#[derive(Clone, Debug)]
pub enum ModuleExpression {
    Expression(Spanned<Expression>),
    Let {
        name: Spanned<Ident>,
        params: Vec<Spanned<Ident>>,
        expr: Spanned<Expression>,
    },
}

#[derive(Clone, Debug)]
pub enum Expression {
    Unit,
    Int(BigUint),
    Bool(bool),
    Ident(Ident),
    Let {
        name: Spanned<Ident>,
        params: Vec<Spanned<Ident>>,
        expr: Spanned<Box<Self>>,
        in_: Spanned<Box<Self>>,
    },
    Call {
        callee: Spanned<Box<Self>>,
        arg: Spanned<Box<Self>>,
    },
    Semicolon(Spanned<Box<Self>>, Option<Spanned<Box<Self>>>),
    PrefixOp {
        expr: Spanned<Box<Self>>,
        op: Spanned<PrefixOp>,
    },
    InfixOp {
        lhs: Spanned<Box<Self>>,
        rhs: Spanned<Box<Self>>,
        op: Spanned<InfixOp>,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum PrefixOp {
    Pos,
    Neg,
    LogicalNot,
    BitwiseNot,
}

#[derive(Clone, Copy, Debug)]
pub enum InfixOp {
    Mul,
    Div,
    Modulo,
    Add,
    Sub,
    LeftBitshift,
    RightBitshift,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
}

mod pretty_print {
    use crate::ir::ast::{Ast, Expression, ModuleExpression};
    use ptree::TreeItem;
    use std::{borrow::Cow, io};

    impl<'a> TreeItem for &'a Ast {
        type Child = &'a ModuleExpression;

        fn write_self<W>(&self, f: &mut W, style: &ptree::Style) -> io::Result<()>
        where
            W: io::Write,
        {
            write!(f, "{}", style.paint("Ast"))
        }

        fn children(&self) -> Cow<'_, [Self::Child]> {
            Cow::from(self.0.iter().map(|e| &e.inner).collect::<Vec<_>>())
        }
    }

    impl<'a> TreeItem for &'a ModuleExpression {
        type Child = &'a Expression;

        fn write_self<W>(&self, f: &mut W, style: &ptree::Style) -> io::Result<()>
        where
            W: io::Write,
        {
            write!(
                f,
                "{}",
                style.paint(match self {
                    ModuleExpression::Expression(_) => "expr".to_owned(),
                    ModuleExpression::Let {
                        name,
                        params,
                        expr: _,
                    } => format!(
                        "let {} {}",
                        name.resolve(),
                        params
                            .iter()
                            .map(|param| param.resolve())
                            .collect::<Vec<_>>()
                            .join(" ")
                    ),
                })
            )
        }

        fn children(&self) -> Cow<'_, [Self::Child]> {
            Cow::from(vec![match &self {
                ModuleExpression::Expression(expr)
                | ModuleExpression::Let {
                    name: _,
                    params: _,
                    expr,
                } => &expr.inner,
            }])
        }
    }

    impl<'a> TreeItem for &'a Expression {
        type Child = &'a Expression;

        fn write_self<W>(&self, f: &mut W, style: &ptree::Style) -> io::Result<()>
        where
            W: io::Write,
        {
            write!(
                f,
                "{}",
                style.paint(match self {
                    Expression::Unit => "()".to_owned(),
                    Expression::Int(value) => value.to_string(),
                    Expression::Bool(value) => value.to_string(),
                    Expression::Ident(ident) => ident.resolve().to_owned(),
                    Expression::Let {
                        name,
                        params,
                        expr: _,
                        in_: _,
                    } => format!(
                        "let {} {}",
                        name.resolve(),
                        params
                            .iter()
                            .map(|param| param.resolve())
                            .collect::<Vec<_>>()
                            .join(" ")
                    ),
                    Expression::Call { callee: _, arg: _ } => "call".to_owned(),
                    Expression::Semicolon(_, _) => ";".to_owned(),
                    Expression::PrefixOp { expr: _, op } => format!("prefix {:?}", op.inner),
                    Expression::InfixOp { lhs: _, rhs: _, op } => format!("infix {:?}", op.inner),
                })
            )
        }

        fn children(&self) -> Cow<'_, [Self::Child]> {
            Cow::from(match self {
                Expression::Unit
                | Expression::Int(_)
                | Expression::Bool(_)
                | Expression::Ident(_) => vec![],
                Expression::Let {
                    name: _,
                    params: _,
                    expr,
                    in_,
                } => vec![expr.inner.as_ref(), in_.inner.as_ref()],
                Expression::Call { callee, arg } => vec![callee.inner.as_ref(), arg.inner.as_ref()],
                Expression::Semicolon(lhs, rhs) => {
                    vec![
                        lhs.inner.as_ref(),
                        rhs.as_ref()
                            .map_or_else(|| panic!(), |rhs| rhs.inner.as_ref()),
                    ]
                }
                Expression::PrefixOp { expr, op: _ } => vec![expr.inner.as_ref()],
                Expression::InfixOp { lhs, rhs, op: _ } => {
                    vec![lhs.inner.as_ref(), rhs.inner.as_ref()]
                }
            })
        }
    }
}
