use crate::parser::*;

pub fn add(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => Number(lhs + rhs),
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
        _ => Nil,
    }
}

pub fn neq(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => Boolean(lhs != rhs),
        _ => Nil,
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
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => Boolean(lhs >= rhs),
        _ => Nil,
    }
}

pub fn lte(lhs: Value, rhs: Value) -> Value {
    use Value::*;
    match (lhs, rhs) {
        (Number(lhs), Number(rhs)) => Boolean(lhs <= rhs),
        _ => Nil,
    }
}

pub fn as_bool(value: Value) -> bool {
    match value {
        Value::Boolean(value) => value,
        _ => false,
    }
}
