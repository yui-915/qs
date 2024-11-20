#![allow(unused)]

use crate::parser::Value;
use std::collections::HashMap;

pub struct Scope {
    pub data: HashMap<String, Value>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get<S>(&self, name: S) -> Value
    where
        S: AsRef<str>,
    {
        self.get_optional(name).unwrap_or(Value::Nil)
    }

    pub fn get_optional<S>(&self, name: S) -> Option<Value>
    where
        S: AsRef<str>,
    {
        self.data.get(name.as_ref()).cloned()
    }

    pub fn set<S>(&mut self, name: S, value: Value)
    where
        S: AsRef<str>,
    {
        self.data.insert(name.as_ref().to_string(), value);
    }

    pub fn has<S>(&self, name: S) -> bool
    where
        S: AsRef<str>,
    {
        self.data.contains_key(name.as_ref())
    }
}

pub struct Storage {
    pub scopes: Vec<Scope>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new())
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define<S>(&mut self, name: S, value: Value)
    where
        S: AsRef<str>,
    {
        self.scopes.last_mut().unwrap().set(name, value);
    }

    pub fn get<S>(&self, name: S) -> Value
    where
        S: AsRef<str>,
    {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get_optional(&name) {
                return value;
            }
        }
        Value::Nil
    }

    pub fn get_optional<S>(&self, name: S) -> Option<Value>
    where
        S: AsRef<str>,
    {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get_optional(&name) {
                return Some(value);
            }
        }
        None
    }

    pub fn set<S>(&mut self, name: S, value: Value)
    where
        S: AsRef<str>,
    {
        for scope in self.scopes.iter_mut().rev() {
            if scope.has(&name) {
                scope.set(name, value);
                return;
            }
        }
        self.scopes.first_mut().unwrap().set(name, value);
    }

    pub fn has<S>(&self, name: S) -> bool
    where
        S: AsRef<str>,
    {
        for scope in self.scopes.iter().rev() {
            if scope.has(&name) {
                return true;
            }
        }
        false
    }

    pub fn current_scope(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    pub fn current_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    pub fn global_scope(&self) -> &Scope {
        self.scopes.first().unwrap()
    }

    pub fn global_scope_mut(&mut self) -> &mut Scope {
        self.scopes.first_mut().unwrap()
    }
}
