use crate::{
    interpreter::{scopes::Scopes, value::Value},
    ir::{
        Ident,
        ast::{Ast, Expression, ModuleExpression},
        map,
    },
};

mod scopes;
mod value;

pub fn interpret(ast: Ast) {
    Interpreter::default().ast(ast);
}

#[derive(Debug, Default)]
struct Interpreter {
    vars: Scopes,
}

impl Interpreter {
    fn ast(&mut self, ast: Ast) {
        for mod_expression in ast.0 {
            self.mod_expression(mod_expression.inner);
        }
    }

    fn mod_expression(&mut self, mod_expression: ModuleExpression) {
        match mod_expression {
            ModuleExpression::Expression(expr) => {
                let _ = self.expression(expr.inner);
            }
            ModuleExpression::Let { name, params, expr } => {
                let value = match params.as_slice() {
                    [] => self.expression(expr.inner),
                    params => Value::Function {
                        params: params.iter().map(|i| i.inner).collect(),
                        env: self.vars.clone(),
                        body: expr.inner,
                    },
                };

                self.vars.push(map(name, Ident::resolve), value);
            }
        }
    }

    #[must_use]
    fn expression(&mut self, expression: Expression) -> Value {
        match expression {
            Expression::Unit => Value::Unit,
            Expression::Int(value) => Value::Int(value),
            Expression::Bool(value) => Value::Bool(value),
            Expression::Ident(ident) => self.vars.get(ident.resolve()).unwrap().clone(),
            Expression::Function { param, body } => todo!(),
            Expression::Let { name, expr, in_ } => todo!(),
            Expression::Call { callee, arg } => {
                let callee = self.expression(*callee.inner);

                let arg = self.expression(*arg.inner);

                self.call(callee, arg)
            }
            Expression::Semicolon(lhs, rhs) => todo!(),
            Expression::PrefixOp { expr, op } => todo!(),
            Expression::InfixOp { lhs, rhs, op } => todo!(),
            Expression::IfThenElse {
                condition,
                then_branch,
                else_branch,
            } => todo!(),
        }
    }

    fn call(&mut self, callee: Value, arg: Value) -> Value {
        match callee {
            Value::Unit | Value::Int(_) | Value::Bool(_) => panic!(),
            Value::Function { params, env, body } => todo!(),
        }
    }
}
