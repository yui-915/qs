#![allow(unused)]

use crate::parser::*;

pub trait Printable {
    fn fmt_print(&self) -> String;
    fn fmt_debug(&self) -> String;
}

impl Printable for Value {
    fn fmt_print(&self) -> String {
        match self {
            Value::Number(value) => value.to_string(),
            Value::String(value) => value.to_string(),
            Value::Boolean(value) => value.to_string(),
            Value::Nil => "nil".to_string(),
        }
    }

    fn fmt_debug(&self) -> String {
        match self {
            Value::Number(value) => value.to_string(),
            Value::String(value) => format!("\"{}\"", value),
            Value::Boolean(value) => value.to_string(),
            Value::Nil => "nil".to_string(),
        }
    }
}
