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

    pub fn register_fn<S>(&mut self, name: S, func: fn(Vec: Vec<Value>) -> Value)
    where
        S: AsRef<str>,
    {
        self.storage.global_scope_mut().set(
            name,
            Value::Closure(Closure::Native(NativeClosure { function: func })),
        )
    }

    pub fn run(&mut self, program: Program) -> Value {
        Block {
            statements: program.statements,
            functions: program.functions,
        }
        .eval(&mut self.storage)
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
            Statement::While(while_statement) => while_statement.eval(storage),
            Statement::For(for_statement) => for_statement.eval(storage),
        }
    }
}

impl Evaluate for ForStatement {
    fn eval(&self, storage: &mut Storage) -> Value {
        storage.push_scope();
        self.initializer.eval(storage);
        loop {
            let cond = self.condition.eval(storage);
            if !ops::as_bool(cond) {
                break;
            }
            // maybe clear _ ?
            self.statement.eval(storage);
            self.increment.eval(storage);
        }
        storage.pop_scope();
        Value::Nil // TODO: return break value
    }
}

impl Evaluate for WhileStatement {
    fn eval(&self, storage: &mut Storage) -> Value {
        storage.push_scope();
        let mut value = Value::Nil;
        loop {
            let cond = self.expression.eval(storage);
            if !ops::as_bool(cond) {
                break;
            }
            value = self.statement.eval(storage);
        }
        storage.pop_scope();
        value
    }
}

impl Evaluate for IfStatement {
    fn eval(&self, storage: &mut Storage) -> Value {
        storage.push_scope();
        let mut value = None;
        for (condition, statement) in &self.conditionals {
            let cond = condition.eval(storage);
            if ops::as_bool(cond) {
                value = Some(statement.eval(storage));
                break;
            }
        }

        storage.pop_scope();
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
        let new = self.expression.eval(storage);

        let value = match self.op {
            SetOp::Set => new,
            _ => {
                let curr = storage.get(&self.identifier).clone();
                match self.op {
                    SetOp::Set => unreachable!(),
                    SetOp::Increment => ops::add(curr, new),
                    SetOp::Decrement => ops::sub(curr, new),
                }
            }
        };

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
            Expression::Map(map) => map.eval(storage),
            Expression::FunctionCall(call) => call.eval(storage),
            Expression::Array(array) => array.eval(storage),
        }
    }
}

impl Evaluate for ExpressionsArray {
    fn eval(&self, storage: &mut Storage) -> Value {
        Value::Array(ValuesArray {
            elements: self
                .elements
                .iter()
                .map(|element| element.eval(storage))
                .collect::<Vec<_>>(),
        })
    }
}

impl Evaluate for FunctionCall {
    fn eval(&self, storage: &mut Storage) -> Value {
        if let Some(func) = storage.get_optional(&self.name) {
            match func {
                Value::Closure(func) => {
                    let args = self
                        .arguments
                        .iter()
                        .map(|arg| arg.eval(storage))
                        .collect::<Vec<_>>();
                    match func {
                        Closure::Normal(closure) => {
                            storage.push_scope();
                            for (name, value) in closure.arguments.iter().zip(args.iter()) {
                                storage.set(name, value.clone());
                            }
                            closure.body.eval(storage)
                        }
                        Closure::Native(closure) => (closure.function)(args),
                    }
                }
                _ => Value::Nil,
            }
        } else {
            Value::Nil
        }
    }
}

impl Evaluate for MapExpression {
    fn eval(&self, storage: &mut Storage) -> Value {
        let input = self.input.eval(storage);
        let mut fallback = Value::Nil;

        for (cases, value) in &self.map {
            for case in cases {
                if let Expression::Identifier(ident) = case {
                    if ident == "_" {
                        fallback = value.eval(storage);
                        continue;
                    }
                }

                let case = case.eval(storage);
                let eq = ops::eq(input.clone(), case);
                if ops::as_bool(eq) {
                    return value.eval(storage);
                }
            }
        }

        fallback
    }
}

impl Evaluate for Block {
    fn eval(&self, storage: &mut Storage) -> Value {
        storage.push_scope();
        for function in &self.functions {
            storage.set(
                &function.name,
                Value::Closure(Closure::Normal(function.closure.clone())),
            );
        }
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
            Operator::And => ops::and(lhs, rhs),
            Operator::Or => ops::or(lhs, rhs),
            Operator::Dollar => ops::dollar(lhs, rhs),
            Operator::DoubleDollar => ops::double_dollar(lhs, rhs),
            Operator::ExclusiveRange => ops::exclusive_range(lhs, rhs),
            Operator::InclusiveRange => ops::inclusive_range(lhs, rhs),
        }
    }
}

impl Evaluate for PrefixedExpression {
    fn eval(&self, storage: &mut Storage) -> Value {
        match self {
            PrefixedExpression::Negative(expression) => ops::negate(expression.eval(storage)),
            PrefixedExpression::Not(expression) => ops::not(expression.eval(storage)),
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
            PostfixedExpression::Index(expression, index) => {
                let value = expression.eval(storage);
                let index = index.eval(storage);
                ops::index(value, index)
            }
        }
    }
}
