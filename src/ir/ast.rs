use crate::ir::{Ident, Spanned};
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct Ast {
    pub statements: Spanned<Vec<Spanned<Statement>>>,
}

impl Ast {
    pub const fn new(statements: Spanned<Vec<Spanned<Statement>>>) -> Self {
        Self { statements }
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Print(Spanned<Expression>),
}

#[derive(Clone, Debug)]
pub enum Expression {
    Int(BigUint),
    Bool(bool),
    Ident(Ident),
    PrefixOp {
        expr: Spanned<Box<Self>>,
        op: Spanned<PrefixOp>,
    },
    PostfixOp {
        expr: Spanned<Box<Self>>,
        op: Spanned<PostfixOp>,
    },
    BinaryOp {
        lhs: Spanned<Box<Self>>,
        rhs: Spanned<Box<Self>>,
        op: Spanned<BinaryOp>,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum PrefixOp {
    Inc,
    Dec,
    Pos,
    Neg,
    BitwiseNot,
}

#[derive(Clone, Copy, Debug)]
pub enum PostfixOp {
    Inc,
    Dec,
    FieldAccess(Ident),
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    LeftBitshift,
    RightBitshift,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
}

mod pretty_print {
    use crate::ir::ast::{Ast, BinaryOp, Expression, PostfixOp, PrefixOp, Statement};
    use ptree::TreeItem;

    impl<'a> TreeItem for &'a Ast {
        type Child = &'a Statement;

        fn write_self<W: std::io::Write>(
            &self,
            f: &mut W,
            style: &ptree::Style,
        ) -> std::io::Result<()> {
            write!(f, "{}", style.paint("ast"))
        }

        fn children(&self) -> std::borrow::Cow<'_, [Self::Child]> {
            self.statements.inner.iter().map(|s| &s.inner).collect()
        }
    }

    impl<'a> TreeItem for &'a Statement {
        type Child = &'a Expression;

        fn write_self<W: std::io::Write>(
            &self,
            f: &mut W,
            style: &ptree::Style,
        ) -> std::io::Result<()> {
            write!(
                f,
                "{}",
                style.paint(match self {
                    Statement::Print(_) => "print",
                })
            )
        }

        fn children(&self) -> std::borrow::Cow<'_, [Self::Child]> {
            match self {
                Statement::Print(expr) => vec![&expr.inner].into(),
            }
        }
    }

    impl TreeItem for &Expression {
        type Child = Self;

        fn write_self<W: std::io::Write>(
            &self,
            f: &mut W,
            style: &ptree::Style,
        ) -> std::io::Result<()> {
            write!(
                f,
                "{}",
                style.paint(match self {
                    Expression::Int(value) => format!("int {value}"),
                    Expression::Bool(value) => format!("bool {value}"),
                    Expression::Ident(value) => format!("ident '{}'", value.resolve()),
                    Expression::PrefixOp { expr: _, op } => format!(
                        "prefix {}",
                        match op.inner {
                            PrefixOp::Inc => "++",
                            PrefixOp::Dec => "--",
                            PrefixOp::Pos => "+",
                            PrefixOp::Neg => "-",
                            PrefixOp::BitwiseNot => "~",
                        }
                    ),
                    Expression::PostfixOp { expr: _, op } => format!(
                        "postfix {}",
                        match op.inner {
                            PostfixOp::Inc => "++".to_string(),
                            PostfixOp::Dec => "--".to_string(),
                            PostfixOp::FieldAccess(ident) => format!(".'{}'", ident.resolve()),
                        }
                    ),
                    Expression::BinaryOp { lhs: _, rhs: _, op } => format!(
                        "binary {}",
                        match op.inner {
                            BinaryOp::Add => "+",
                            BinaryOp::Sub => "-",
                            BinaryOp::Mul => "*",
                            BinaryOp::Div => "/",
                            BinaryOp::Modulo => "%",
                            BinaryOp::LeftBitshift => "<<",
                            BinaryOp::RightBitshift => ">>",
                            BinaryOp::BitwiseAnd => "&",
                            BinaryOp::BitwiseXor => "^",
                            BinaryOp::BitwiseOr => "|",
                        }
                    ),
                })
            )
        }

        fn children(&self) -> std::borrow::Cow<'_, [Self::Child]> {
            match self {
                Expression::Int(_) | Expression::Bool(_) | Expression::Ident(_) => vec![].into(),
                Expression::PrefixOp { expr, op: _ } | Expression::PostfixOp { expr, op: _ } => {
                    vec![expr.inner.as_ref()].into()
                }
                Expression::BinaryOp { lhs, rhs, op: _ } => {
                    vec![lhs.inner.as_ref(), rhs.inner.as_ref()].into()
                }
            }
        }
    }
}
