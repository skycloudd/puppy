use crate::{
    diagnostics::Diagnostic,
    interpreter::{env::Env, value::Value},
    ir::{
        Ident,
        ast::{Ast, Expression, InfixOp, ModuleExpression, PrefixOp},
    },
};
use chumsky::span::SpanWrap as _;
use std::rc::Rc;

mod env;
pub mod value;

pub fn interpret(ast: Ast) -> Result<(), Diagnostic> {
    Interpreter::default().ast(ast)
}

#[derive(Default)]
struct Interpreter {
    env: Env,
}

impl Interpreter {
    fn ast(&mut self, ast: Ast) -> Result<(), Diagnostic> {
        self.env.push(
            Ident::get_or_intern("print"),
            Value::Builtin(Rc::new(|v| {
                println!("{v}");
                Ok(Value::Unit)
            })),
        );

        for mod_expression in ast.0 {
            self.mod_expression(mod_expression.inner)?;
        }

        Ok(())
    }

    fn mod_expression(&mut self, mod_expression: ModuleExpression) -> Result<(), Diagnostic> {
        match mod_expression {
            ModuleExpression::Expression(expr) => {
                let _ = self.expression(expr.inner)?;
            }
            ModuleExpression::Let { name, expr } => {
                let expr = self.expression(expr.inner)?;

                self.env.push(name.inner, expr);
            }
        }

        Ok(())
    }

    fn expression(&mut self, expression: Expression) -> Result<Value, Diagnostic> {
        match expression {
            Expression::Unit => Ok(Value::Unit),
            Expression::Int(value) => Ok(Value::Int(value)),
            Expression::Bool(value) => Ok(Value::Bool(value)),
            Expression::Ident(ident) => self
                .env
                .get(ident.inner)
                .ok_or_else(|| Diagnostic::NameNotDefined { ident }),
            Expression::Function { param, body } => Ok(Value::Function {
                param: param.inner,
                env: self.env.clone(),
                body: *body.inner,
            }),
            Expression::Let { name, expr, in_ } => {
                let expr = self.expression(*expr.inner)?;

                self.env.push(name.inner, expr);

                let in_ = self.expression(*in_.inner);

                self.env.pop();

                in_
            }
            Expression::Call { callee, arg } => {
                let callee = self.expression(*callee.inner)?.with_span(callee.span);

                let arg = self.expression(*arg.inner)?;

                match callee.inner {
                    Value::Unit | Value::Int(_) | Value::Bool(_) => Err(Diagnostic::CantCall {
                        callee: Box::new(callee),
                    }),
                    Value::Function {
                        param,
                        mut env,
                        body,
                    } => {
                        let env_len = self.env.len();

                        self.env.append(&mut env);
                        self.env.push(param, arg);

                        let result = self.expression(body);

                        self.env.truncate(env_len);

                        result
                    }
                    Value::Builtin(func) => func(arg),
                }
            }
            Expression::Semicolon(lhs, rhs) => {
                let _ = self.expression(*lhs.inner)?;

                rhs.map_or(Ok(Value::Unit), |rhs| self.expression(*rhs.inner))
            }
            Expression::PrefixOp { expr, op } => {
                let expr = self.expression(*expr.inner)?.with_span(expr.span);

                {
                    use Value::{Bool, Int};

                    #[expect(clippy::arithmetic_side_effects)]
                    match (&expr.inner, op.inner) {
                        (Int(expr), PrefixOp::Pos) => Ok(Int(expr.clone())),
                        (Int(expr), PrefixOp::Neg) => Ok(Int(-expr)),
                        (Int(expr), PrefixOp::Not) => Ok(Int(!expr)),

                        (Bool(expr), PrefixOp::Not) => Ok(Bool(!expr)),

                        _ => Err(Diagnostic::CantApplyPrefixOp {
                            expr: Box::new(expr),
                            op,
                        }),
                    }
                }
            }
            Expression::InfixOp { lhs, rhs, op } => {
                let lhs = self.expression(*lhs.inner)?.with_span(lhs.span);
                let rhs = self.expression(*rhs.inner)?.with_span(rhs.span);

                {
                    use Value::{Bool, Int, Unit};

                    #[expect(clippy::arithmetic_side_effects)]
                    match (&lhs.inner, &rhs.inner, op.inner) {
                        (Unit, Unit, InfixOp::Equal) => Ok(Bool(true)),
                        (Unit, Unit, InfixOp::NotEqual) => Ok(Bool(false)),

                        (Int(lhs), Int(rhs), InfixOp::Mul) => Ok(Int(lhs * rhs)),
                        (Int(lhs), Int(rhs), InfixOp::Div) => Ok(Int(lhs / rhs)),
                        (Int(lhs), Int(rhs), InfixOp::Modulo) => Ok(Int(lhs % rhs)),
                        (Int(lhs), Int(rhs), InfixOp::Add) => Ok(Int(lhs + rhs)),
                        (Int(lhs), Int(rhs), InfixOp::Sub) => Ok(Int(lhs - rhs)),
                        // (Int(lhs), Int(rhs), InfixOp::LeftBitshift) => Ok(Int(lhs << rhs)),
                        // (Int(lhs), Int(rhs), InfixOp::RightBitshift) => Ok(Int(lhs >> rhs)),
                        (Int(lhs), Int(rhs), InfixOp::LessThan) => Ok(Bool(lhs < rhs)),
                        (Int(lhs), Int(rhs), InfixOp::GreaterThan) => Ok(Bool(lhs > rhs)),
                        (Int(lhs), Int(rhs), InfixOp::LessThanEquals) => Ok(Bool(lhs <= rhs)),
                        (Int(lhs), Int(rhs), InfixOp::GreaterThanEquals) => Ok(Bool(lhs >= rhs)),
                        (Int(lhs), Int(rhs), InfixOp::Equal) => Ok(Bool(lhs == rhs)),
                        (Int(lhs), Int(rhs), InfixOp::NotEqual) => Ok(Bool(lhs != rhs)),
                        (Int(lhs), Int(rhs), InfixOp::BitwiseAnd) => Ok(Int(lhs & rhs)),
                        (Int(lhs), Int(rhs), InfixOp::BitwiseXor) => Ok(Int(lhs ^ rhs)),
                        (Int(lhs), Int(rhs), InfixOp::BitwiseOr) => Ok(Int(lhs | rhs)),

                        (Bool(lhs), Bool(rhs), InfixOp::Equal) => Ok(Bool(lhs == rhs)),
                        (Bool(lhs), Bool(rhs), InfixOp::NotEqual) => Ok(Bool(lhs != rhs)),
                        (Bool(lhs), Bool(rhs), InfixOp::BitwiseAnd) => Ok(Bool(lhs & rhs)),
                        (Bool(lhs), Bool(rhs), InfixOp::BitwiseXor) => Ok(Bool(lhs ^ rhs)),
                        (Bool(lhs), Bool(rhs), InfixOp::BitwiseOr) => Ok(Bool(lhs | rhs)),
                        (Bool(lhs), Bool(rhs), InfixOp::LogicalAnd) => Ok(Bool(*lhs && *rhs)),
                        (Bool(lhs), Bool(rhs), InfixOp::LogicalOr) => Ok(Bool(*lhs || *rhs)),

                        _ => Err(Diagnostic::CantApplyInfixOp {
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                            op: Box::new(op),
                        }),
                    }
                }
            }
            Expression::IfThenElse {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.expression(*condition.inner)?.with_span(condition.span);

                match condition.inner {
                    Value::Bool(true) => self.expression(*then_branch.inner),
                    Value::Bool(false) => else_branch.map_or(Ok(Value::Unit), |else_branch| {
                        self.expression(*else_branch.inner)
                    }),
                    Value::Unit
                    | Value::Int(_)
                    | Value::Function {
                        param: _,
                        env: _,
                        body: _,
                    }
                    | Value::Builtin(_) => Err(Diagnostic::InvalidCondition {
                        condition: Box::new(condition),
                    }),
                }
            }
        }
    }
}
