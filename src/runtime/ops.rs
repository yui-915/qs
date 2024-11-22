use super::{Evaluate, Storage};
use crate::parser::*;

pub fn add(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => Number(lhs + rhs),
        (String(lhs), String(rhs)) => String(lhs + &rhs),
        (Array(lhs), rhs) => Array(ValuesArray {
            elements: lhs
                .elements
                .iter()
                .cloned()
                .chain(std::iter::once(rhs))
                .collect(),
        }),
        _ => Nil,
    }
}

pub fn negate(value: Value) -> Value {
    use Value::*;
    match value {
        Number(value) => Number(-value),
        _ => Nil,
    }
}

pub fn sub(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => Number(lhs - rhs),
        _ => Nil,
    }
}

pub fn mul(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => Number(lhs * rhs),
        _ => Nil,
    }
}

pub fn div(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => Number(lhs / rhs),
        _ => Nil,
    }
}

pub fn eq(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => Boolean(lhs == rhs),
        (String(lhs), String(rhs)) => Boolean(lhs == rhs),
        (Nil, Nil) => Boolean(true),
        _ => Boolean(false),
    }
}

pub fn neq(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match eq(lhs, rhs) {
        Boolean(value) => Boolean(!value),
        _ => unreachable!(),
    }
}

pub fn gt(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => Boolean(lhs > rhs),
        _ => Nil,
    }
}

pub fn lt(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => Boolean(lhs < rhs),
        _ => Nil,
    }
}

pub fn gte(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match lt(lhs, rhs) {
        Boolean(value) => Boolean(!value),
        _ => unreachable!(),
    }
}

pub fn lte(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match gt(lhs, rhs) {
        Boolean(value) => Boolean(!value),
        _ => unreachable!(),
    }
}

pub fn as_bool(value: Value) -> bool {
    match value {
        Value::Boolean(value) => value,
        _ => false,
    }
}

pub fn and(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Boolean(lhs), Boolean(rhs)) => Boolean(lhs && rhs),
        _ => Nil,
    }
}

pub fn or(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Boolean(lhs), Boolean(rhs)) => Boolean(lhs || rhs),
        _ => Nil,
    }
}

pub fn as_string(value: Value) -> String {
    use Value::*;
    match value {
        String(value) => value,
        Number(value) => value.to_string(),
        Boolean(value) => value.to_string(),
        Nil => "nil".to_string(),
        _ => "????".to_string(),
    }
}

pub fn dollar(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (String(lhs), String(rhs)) => Value::Array(ValuesArray {
            elements: lhs
                .split(&rhs)
                .map(|value| Value::String(value.to_string()))
                .collect(),
        }),
        (Array(lhs), String(rhs)) => Value::String(
            lhs.elements
                .iter()
                .cloned()
                .map(as_string)
                .collect::<Vec<_>>()
                .join(&rhs),
        ),
        _ => Nil,
    }
}

pub fn double_dollar(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (String(lhs), String(rhs)) => Value::Array(ValuesArray {
            elements: match lhs.split_once(&rhs) {
                None => vec![lhs.clone()],
                Some((lhs, rhs)) => vec![lhs.to_string(), rhs.to_string()],
            }
            .iter()
            .cloned()
            .map(Value::String)
            .collect(),
        }),
        (Array(lhs), String(rhs)) => {
            let mut arr = lhs
                .elements
                .iter()
                .cloned()
                .map(as_string)
                .collect::<Vec<_>>();
            let arr = match arr.len() {
                0 => vec![],
                1 => vec![arr[0].clone()],
                _ => {
                    let mut r = vec![arr[0].clone() + &rhs + &arr[1]];
                    r.extend(arr.into_iter().skip(2));
                    r
                }
            };
            Value::Array(ValuesArray {
                elements: arr.iter().cloned().map(Value::String).collect(),
            })
        }
        _ => Nil,
    }
}

pub fn not(value: Value) -> Value {
    Value::Boolean(!as_bool(value))
}

pub fn to_index(i: f64, len: usize) -> Option<usize> {
    let (i, len) = (i as isize, len as isize);
    if i >= 0 && i < len {
        Some(i as usize)
    } else {
        let i = len + i;
        if i >= 0 && i < len {
            Some(i as usize)
        } else {
            None
        }
    }
}

pub fn erange_to_idxs(start: f64, end: f64, len: usize) -> Option<(usize, usize)> {
    let (start, end) = (to_index(start, len), to_index(end, len));
    match (start, end) {
        (Some(start), Some(end)) => Some((start, end)),
        _ => None,
    }
}

pub fn index(value: Value, idx: Value) -> Value {
    use Value::*;
    match value {
        Array(arr) => match idx {
            Number(index) => to_index(index, arr.elements.len())
                .map(|i| arr.elements[i].clone())
                .unwrap_or(Nil),
            ExclusiveRange(start, end) => {
                let len = arr.elements.len();
                let (start, end) = (to_index(start, len), to_index(end - 1., len));
                match (start, end) {
                    (Some(start), Some(end)) => Value::Array(ValuesArray {
                        elements: arr.elements[start..=end].to_vec(),
                    }),
                    _ => Nil,
                }
            }
            InclusiveRange(start, end) => {
                let len = arr.elements.len();
                let (start, end) = (to_index(start, len), to_index(end, len));
                match (start, end) {
                    (Some(start), Some(end)) => Value::Array(ValuesArray {
                        elements: arr.elements[start..=end].to_vec(),
                    }),
                    _ => Nil,
                }
            }
            String(key) => key
                .parse::<f64>()
                .map(|key| index(Value::Array(arr), Value::Number(key)))
                .unwrap_or(Nil),
            _ => Nil,
        },
        Table(table) => match idx {
            Value::String(key) => table.map.get(&key).cloned().unwrap_or(Nil),
            _ => Nil,
        },
        _ => Nil,
    }
}

pub fn dot_index(value: Value, idx: String) -> Value {
    index(value, Value::String(idx))
}

pub fn exclusive_range(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => ExclusiveRange(lhs, rhs),
        _ => Nil,
    }
}

pub fn inclusive_range(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => InclusiveRange(lhs, rhs),
        _ => Nil,
    }
}

pub fn hash(value: Value) -> Value {
    use Value::*;
    match value {
        Table(table) => Value::Array(ValuesArray {
            elements: table.map.keys().cloned().map(Value::String).collect(),
        }),
        _ => Nil,
    }
}

pub fn double_hash(value: Value) -> Value {
    use Value::*;
    match value {
        Table(table) => Value::Array(ValuesArray {
            elements: table.map.values().cloned().collect(),
        }),
        _ => Nil,
    }
}

pub fn triple_hash(value: Value) -> Value {
    use Value::*;
    match value {
        Table(table) => Value::Array(ValuesArray {
            elements: table
                .map
                .into_iter()
                .map(|(k, v)| {
                    Value::Array(ValuesArray {
                        elements: vec![Value::String(k), v],
                    })
                })
                .collect(),
        }),
        _ => Nil,
    }
}

pub fn modulo(lhs: Value, rhs: Value, storage: &mut Storage) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => Number(lhs % rhs),
        (Array(lhs), Closure(rhs)) => Array(ValuesArray {
            elements: lhs
                .elements
                .into_iter()
                .filter(|x| as_bool(run_closure(rhs.clone(), vec![x.clone()], storage)))
                .collect(),
        }),
        _ => Nil,
    }
}

pub fn run_closure(closure: Closure, args: Vec<Value>, storage: &mut Storage) -> Value {
    match closure {
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
