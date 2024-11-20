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
            let value = statement.eval(&mut self.storage);
            self.storage.set("_", value);
        }
        self.storage.get("_")
    }
}

trait Evaluate {
    fn eval(&self, storage: &mut Storage) -> Value;
}

impl Evaluate for Statement {
    fn eval(&self, storage: &mut Storage) -> Value {
        match self {
            Statement::Expression(expression) => expression.eval(storage),
            Statement::Set(set) => set.eval(storage),
            Statement::Define(define) => define.eval(storage),
            Statement::DefineAndSet(define_and_set) => define_and_set.eval(storage),
            Statement::If(if_statement) => if_statement.eval(storage),
        }
    }
}

impl Evaluate for IfStatement {
    fn eval(&self, storage: &mut Storage) -> Value {
        let mut value = None;
        for (condition, statement) in &self.conditionals {
            let cond = condition.eval(storage);
            if ops::as_bool(cond) {
                value = Some(statement.eval(storage));
                break;
            }
        }

        value.unwrap_or_else(|| {
            if let Some(statement) = &self.otherwise {
                statement.eval(storage)
            } else {
                Value::Nil
            }
        })
    }
}

impl Evaluate for SetStatement {
    fn eval(&self, storage: &mut Storage) -> Value {
        let value = self.expression.eval(storage);
        storage.set(&self.identifier, value.clone());
        value
    }
}

impl Evaluate for DefineStatement {
    fn eval(&self, storage: &mut Storage) -> Value {
        storage.define(&self.identifier, Value::Nil);
        Value::Nil
    }
}

impl Evaluate for DefineAndSetStatement {
    fn eval(&self, storage: &mut Storage) -> Value {
        let value = self.expression.eval(storage);
        storage.define(&self.identifier, value.clone());
        value
    }
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
            Expression::Block(block) => block.eval(storage),
        }
    }
}

impl Evaluate for Block {
    fn eval(&self, storage: &mut Storage) -> Value {
        storage.push_scope();
        for statement in &self.statements {
            let value = statement.eval(storage);
            storage.set("_", value);
        }
        let res = storage.get("_");
        storage.pop_scope();
        res
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
            Operator::Eq => ops::eq(lhs, rhs),
            Operator::Neq => ops::neq(lhs, rhs),
            Operator::Gt => ops::gt(lhs, rhs),
            Operator::Lt => ops::lt(lhs, rhs),
            Operator::Gte => ops::gte(lhs, rhs),
            Operator::Lte => ops::lte(lhs, rhs),
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
