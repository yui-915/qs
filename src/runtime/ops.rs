use crate::parser::*;

pub fn add(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => Number(lhs + rhs),
        (String(lhs), String(rhs)) => String(lhs + &rhs),
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

pub fn index(value: Value, index: Value) -> Value {
    use Value::*;
    if let Number(index) = index {
        let index = index as usize;
        match value {
            Array(arr) => {
                if index >= arr.elements.len() {
                    Nil
                } else {
                    arr.elements[index].clone()
                }
            }
            _ => Nil,
        }
    } else {
        Nil
    }
}
