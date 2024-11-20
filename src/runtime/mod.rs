#![allow(unused)]

mod formater;
mod ops;
mod storage;

use crate::parser::*;
pub use formater::*;
pub use storage::*;

pub struct Runtime {
    pub storage: Storage,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            storage: Storage::new(),
        }
    }

    pub fn run(&mut self, program: Program) -> Value {
        for statement in program.statements {
            let value = match statement {
                Statement::Expression(expression) => expression.eval(&mut self.storage),
            };
            self.storage.set("_", value);
        }
        self.storage.get("_")
    }
}

trait Evaluate {
    fn eval(&self, storage: &mut Storage) -> Value;
}

impl Evaluate for Value {
    fn eval(&self, _storage: &mut Storage) -> Value {
        self.clone()
    }
}

impl Evaluate for Expression {
    fn eval(&self, storage: &mut Storage) -> Value {
        match self {
            Expression::Value(value) => value.eval(storage),
            Expression::Infixed(operation) => operation.eval(storage),
            Expression::Prefixed(prefixed) => prefixed.eval(storage),
            Expression::Postfixed(postfixed) => postfixed.eval(storage),
            Expression::Identifier(identifier) => storage.get(identifier),
        }
    }
}

impl Evaluate for Operation {
    fn eval(&self, storage: &mut Storage) -> Value {
        let lhs = self.lhs.eval(storage);
        let rhs = self.rhs.eval(storage);
        match self.infix {
            Operator::Add => ops::add(lhs, rhs),
            Operator::Sub => ops::sub(lhs, rhs),
            Operator::Mul => ops::mul(lhs, rhs),
            Operator::Div => ops::div(lhs, rhs),
        }
    }
}

impl Evaluate for PrefixedExpression {
    fn eval(&self, storage: &mut Storage) -> Value {
        match self {
            PrefixedExpression::Negative(expression) => ops::negate(expression.eval(storage)),
        }
    }
}

impl Evaluate for PostfixedExpression {
    fn eval(&self, storage: &mut Storage) -> Value {
        match self {
            PostfixedExpression::Debug(expression) => {
                let value = expression.eval(storage);
                println!("{}", value.fmt_debug());
                value
            }
            PostfixedExpression::Print(expression) => {
                let value = expression.eval(storage);
                println!("{}", value.fmt_print());
                value
            }
        }
    }
}
