#![allow(unused)]

use crate::parser::*;

pub trait Printable {
    fn fmt_print(&self) -> String;
    fn fmt_debug(&self) -> String;
}

impl Printable for Value {
    fn fmt_print(&self) -> String {
        match self {
            Value::Array(values_array) => {
                format!(
                    "[{}]",
                    values_array
                        .elements
                        .iter()
                        .map(|value| value.fmt_print())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Value::Number(value) => value.to_string(),
            Value::String(value) => format!("\"{}\"", value),
            Value::Boolean(value) => value.to_string(),
            Value::Nil => "nil".to_string(),
            Value::Closure(closure) => match closure {
                Closure::Normal(normal_closure) => {
                    format!("|{}| {{ ... }}", normal_closure.arguments.join(", "))
                }
                Closure::Native(native_closure) => "|...| { NativeCode }".to_string(),
            },
        }
    }

    fn fmt_debug(&self) -> String {
        self.fmt_print()
    }
}
