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
