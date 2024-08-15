//! This module defines a custom value that equale to serde_json::Value, to implement
//! BorshSerialize and BorshDeserialize on it.
use std::collections::BTreeMap;

// This is required since the expansion of the procedural macros BorshSerialize and
// BorshDeserialize contains `use borsh`.
use crate as borsh;
use crate::{BorshDeserialize, BorshSerialize};
use serde_json::{Number as JsonNumber, Value as JsonValue};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
enum N {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Always finite.
    Float(f64),
}

impl From<JsonNumber> for N {
    fn from(number: JsonNumber) -> Self {
        match number.as_u64() {
            Some(n) => Self::PosInt(n),
            None => match number.as_i64() {
                Some(n) => Self::NegInt(n),
                None => Self::Float(number.as_f64().unwrap()),
            },
        }
    }
}

impl From<N> for JsonNumber {
    fn from(number: N) -> Self {
        match number {
            N::PosInt(n) => n.into(),
            N::NegInt(n) => n.into(),
            N::Float(x) => Self::from_f64(x).unwrap(),
        }
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Number {
    n: N,
}

impl From<JsonNumber> for Number {
    fn from(value: JsonNumber) -> Self {
        Self { n: value.into() }
    }
}

impl From<Number> for JsonNumber {
    fn from(value: Number) -> Self {
        value.n.into()
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl From<JsonValue> for Value {
    fn from(value: JsonValue) -> Self {
        match value {
            JsonValue::Null => Self::Null,
            JsonValue::Bool(b) => Self::Bool(b),
            JsonValue::Number(n) => Self::Number(n.into()),
            JsonValue::String(s) => Self::String(s),
            JsonValue::Array(v) => Self::Array(v.into_iter().map(|s| s.into()).collect()),
            JsonValue::Object(o) => {
                let map = o.into_iter().map(|(k, v)| (k, v.into())).collect();
                Self::Object(map)
            }
        }
    }
}

impl From<Value> for JsonValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::Bool(b) => Self::Bool(b),
            Value::Number(n) => Self::Number(n.into()),
            Value::String(s) => Self::String(s),
            Value::Array(v) => Self::Array(v.into_iter().map(|s| s.into()).collect()),
            Value::Object(o) => {
                let map = o.into_iter().map(|(k, v)| (k, v.into())).collect();
                Self::Object(map)
            }
        }
    }
}
