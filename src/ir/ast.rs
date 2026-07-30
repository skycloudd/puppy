use crate::ir::{Ident, Spanned};
use core::fmt;
use num_bigint::BigInt;

#[derive(Clone, Debug)]
pub struct Ast(pub Vec<Spanned<ModuleExpression>>);

#[derive(Clone, Debug)]
pub enum ModuleExpression {
    Expression(Spanned<Expression>),
    Let {
        name: Spanned<Ident>,
        expr: Spanned<Expression>,
    },
}

#[derive(Clone, Debug)]
pub enum Expression {
    Unit,
    Int(BigInt),
    Bool(bool),
    Ident(Spanned<Ident>),
    Function {
        param: Spanned<Ident>,
        body: Spanned<Box<Self>>,
    },
    Let {
        name: Spanned<Ident>,
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
    IfThenElse {
        condition: Spanned<Box<Self>>,
        then_branch: Spanned<Box<Self>>,
        else_branch: Option<Spanned<Box<Self>>>,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum PrefixOp {
    Pos,
    Neg,
    Not,
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

impl fmt::Display for PrefixOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pos => "+",
                Self::Neg => "-",
                Self::Not => "!",
            }
        )
    }
}

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Mul => "*",
                Self::Div => "/",
                Self::Modulo => "%",
                Self::Add => "+",
                Self::Sub => "-",
                Self::LeftBitshift => "<<",
                Self::RightBitshift => ">>",
                Self::LessThan => "<",
                Self::GreaterThan => ">",
                Self::LessThanEquals => "<=",
                Self::GreaterThanEquals => ">=",
                Self::Equal => "==",
                Self::NotEqual => "!=",
                Self::BitwiseAnd => "&",
                Self::BitwiseXor => "^",
                Self::BitwiseOr => "|",
                Self::LogicalAnd => "&&",
                Self::LogicalOr => "||",
            }
        )
    }
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
                    ModuleExpression::Let { name, expr: _ } => format!("let {}", name.resolve()),
                })
            )
        }

        fn children(&self) -> Cow<'_, [Self::Child]> {
            Cow::from(vec![match &self {
                ModuleExpression::Expression(expr) | ModuleExpression::Let { name: _, expr } => {
                    &expr.inner
                }
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
                    Expression::Function { param, body: _ } => format!("fn {}", param.resolve()),
                    Expression::Let {
                        name,
                        expr: _,
                        in_: _,
                    } => format!("let {}", name.resolve()),
                    Expression::Call { callee: _, arg: _ } => "call".to_owned(),
                    Expression::Semicolon(_, _) => ";".to_owned(),
                    Expression::PrefixOp { expr: _, op } => format!("prefix {:?}", op.inner),
                    Expression::InfixOp { lhs: _, rhs: _, op } => format!("infix {:?}", op.inner),
                    Expression::IfThenElse {
                        condition: _,
                        then_branch: _,
                        else_branch: _,
                    } => "if then else".to_owned(),
                })
            )
        }

        fn children(&self) -> Cow<'_, [Self::Child]> {
            Cow::from(match self {
                Expression::Unit
                | Expression::Int(_)
                | Expression::Bool(_)
                | Expression::Ident(_) => vec![],
                Expression::Function { param: _, body } => vec![body.inner.as_ref()],
                Expression::Let { name: _, expr, in_ } => {
                    vec![expr.inner.as_ref(), in_.inner.as_ref()]
                }
                Expression::Call { callee, arg } => vec![callee.inner.as_ref(), arg.inner.as_ref()],
                Expression::Semicolon(lhs, rhs) => {
                    let mut exprs = vec![lhs.inner.as_ref()];

                    if let Some(rhs) = rhs {
                        exprs.push(rhs);
                    }

                    exprs
                }
                Expression::PrefixOp { expr, op: _ } => vec![expr.inner.as_ref()],
                Expression::InfixOp { lhs, rhs, op: _ } => {
                    vec![lhs.inner.as_ref(), rhs.inner.as_ref()]
                }
                Expression::IfThenElse {
                    condition,
                    then_branch: if_branch,
                    else_branch,
                } => {
                    let mut exprs = vec![condition.inner.as_ref(), if_branch.inner.as_ref()];

                    if let Some(else_branch) = else_branch {
                        exprs.push(else_branch);
                    }

                    exprs
                }
            })
        }
    }
}
