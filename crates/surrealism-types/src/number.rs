use std::ffi::{c_double, c_float, c_int};

use surrealdb::sql;

use super::value::Value;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum Number {
    SR_NUMBER_INT(i64),
    SR_NUMBER_FLOAT(f64),
}

impl From<c_int> for Number {
    fn from(value: c_int) -> Self {
        Number::SR_NUMBER_INT(value as i64)
    }
}

impl From<c_float> for Number {
    fn from(value: c_float) -> Self {
        Number::SR_NUMBER_FLOAT(value as f64)
    }
}

impl From<c_double> for Number {
    fn from(value: c_double) -> Self {
        Number::SR_NUMBER_FLOAT(value)
    }
}

impl From<Number> for sql::Number {
    fn from(value: Number) -> Self {
        match value {
            Number::SR_NUMBER_INT(i) => sql::Number::Int(i),
            Number::SR_NUMBER_FLOAT(f) => sql::Number::Float(f),
        }
    }
}

impl From<sql::Number> for Number {
    fn from(value: sql::Number) -> Self {
        match value {
            sql::Number::Int(i) => Self::SR_NUMBER_INT(i),
            sql::Number::Float(i) => Self::SR_NUMBER_FLOAT(i),
            _ => todo!(),
        }
    }
}

impl From<Number> for Value {
    fn from(value: Number) -> Self {
        Value::SR_VALUE_NUMBER(value)
    }
}
