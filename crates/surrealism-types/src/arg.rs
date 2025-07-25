use std::{collections::{BTreeMap, HashMap}, time};
use rust_decimal::Decimal;
use surrealdb::expr;
use anyhow::Result;

pub trait Arg: Sized {
    fn is_value(value: &expr::Value) -> bool;
    fn from_value(value: expr::Value) -> Result<Self>;
    fn to_value(self) -> expr::Value;
    fn kindof() -> expr::Kind;

    fn invalid_err() -> anyhow::Error {
        anyhow::anyhow!("Expected {}, found other value", Self::kindof())
    }

    fn to_serializable(self) -> SerializableArg<Self> {
        SerializableArg(self)
    }
}

pub struct SerializableArg<T: Arg>(pub T);
impl<T: Arg> From<T> for SerializableArg<T> {
    fn from(value: T) -> Self {
        SerializableArg(value)
    }
}

impl Arg for expr::Value {
    fn is_value(_: &expr::Value) -> bool {
        true
    }

    fn from_value(value: expr::Value) -> Result<Self> {
        Ok(value)
    }

    fn to_value(self) -> expr::Value {
        self
    }

    fn kindof() -> expr::Kind {
        expr::Kind::Any
    }
}

macro_rules! impl_arg_types {
    ($(($type:ty$(as $conversion_type:ty)?, $is_fnc:ident, $into_fnc:ident, $kindof:expr)),+$(,)?) => {
        $(
            impl Arg for $type {
                fn is_value(value: &expr::Value) -> bool {
                    value.$is_fnc()
                }

                fn from_value(value: expr::Value) -> Result<Self> {
                    let v = value.$into_fnc().ok_or_else(|| Self::invalid_err())?;
                    $(let v: $conversion_type = v.into();)?
                    Ok(v.into())
                }

                fn to_value(self) -> expr::Value {
                    let v = self;
                    $(let v: $conversion_type = v.into();)?
                    expr::Value::from(v)
                }

                fn kindof() -> expr::Kind {
                    $kindof
                }
            }
        )+
    };
}

impl_arg_types! {
    (bool, is_bool, into_bool, expr::Kind::Bool),

    (expr::Number, is_number, into_number, expr::Kind::Number),
    (i64, is_int, into_int, expr::Kind::Int),
    (f64, is_float, into_float, expr::Kind::Float),
    (Decimal, is_decimal, into_decimal, expr::Kind::Decimal),

    (expr::Strand, is_strand, into_strand, expr::Kind::String),
    (String as expr::Strand, is_strand, into_strand, expr::Kind::String),

    (expr::Duration, is_duration, into_duration, expr::Kind::Duration),
    (time::Duration as expr::Duration, is_duration, into_duration, expr::Kind::Duration),

    (expr::Datetime, is_datetime, into_datetime, expr::Kind::Datetime),
    (chrono::DateTime<chrono::Utc> as expr::Datetime, is_datetime, into_datetime, expr::Kind::Datetime),

    (expr::Uuid, is_uuid, into_uuid, expr::Kind::Uuid),
    (uuid::Uuid as expr::Uuid, is_uuid, into_uuid, expr::Kind::Uuid),

    (expr::Bytes, is_bytes, into_bytes, expr::Kind::Bytes),
    (bytes::Bytes as Vec<u8>, is_bytes, into_bytes, expr::Kind::Bytes),

    (expr::Array, is_array, into_array, expr::Kind::Array(Box::new(expr::Kind::Any), None)),
    (expr::Object, is_object, into_object, expr::Kind::Object),
    (expr::Geometry, is_geometry, into_geometry, expr::Kind::Geometry(vec![])),
    (expr::Thing, is_thing, into_thing, expr::Kind::Record(vec![])),
}

impl<T: Arg> Arg for Vec<T> {
    fn is_value(value: &expr::Value) -> bool {
        let expr::Value::Array(arr) = value else {
            return false;
        };

        arr.iter().all(T::is_value)
    }

    fn from_value(value: expr::Value) -> Result<Self> {
        value.as_array().ok_or_else(|| Self::invalid_err())?
            .iter()
            .map(|v| T::from_value(v.clone()))
            .collect::<Result<Vec<_>>>()
    }

    fn to_value(self) -> expr::Value {
        let vals: Vec<expr::Value> = self
            .into_iter()
            .map(|v| v.to_value())
            .collect();

        expr::Value::from(expr::Array::from(vals))
    }

    fn kindof() -> expr::Kind {
        expr::Kind::Array(Box::new(T::kindof()), None)
    }
}

macro_rules! impl_arg_map {
    ($($type:ident),+ $(,)?) => {
        $(
            impl<K: Into<String> + From<String> + Eq + std::hash::Hash + std::cmp::Ord, V: Arg> Arg for $type<K, V> {
                fn is_value(value: &expr::Value) -> bool {
                    let expr::Value::Object(obj) = value else {
                        return false;
                    };

                    obj.iter().all(|(_, v)| {
                        V::is_value(v)
                    })
                }

                fn from_value(value: expr::Value) -> Result<Self> {
                    if let expr::Value::Object(obj) = value {
                        Ok(Self::from_iter(
                            obj
                                .into_iter()
                                .map(|(k, v)| -> Result<(K, V)> { Ok((k.into(), V::from_value(v)?)) })
                                .collect::<Result<Vec<(K, V)>>>()?
                                .into_iter(),
                        ))
                    } else {
                        Err(anyhow::anyhow!(
                            "Expected object, found: {:?}",
                            value.kind()
                        ))
                    }
                }

                fn to_value(self) -> expr::Value {
                    expr::Value::from(expr::Object::from_iter(self.into_iter()
                        .map(|(k, v)| (k.into(), v.to_value()))))
                }

                fn kindof() -> expr::Kind {
                    expr::Kind::Object
                }
            }
        )+
    }
}

impl_arg_map! {
    HashMap,
    BTreeMap,
}

// Tuples

macro_rules! impl_args_tuples {
    ($($len:literal => ($($name:ident),+)),+ $(,)?) => {
        $(
            impl<$($name),+> Arg for ($($name,)+)
            where
                $($name: Arg),+
            {
                fn is_value(value: &expr::Value) -> bool {
                    let expr::Value::Array(arr) = value else {
                        return false;
                    };

                    arr.len() == $len && arr.iter().zip([$($name::is_value,)+]).all(|(v, is)| is(v))
                }

                fn from_value(value: expr::Value) -> Result<Self> {
                    if let expr::Value::Array(arr) = value {
                        if arr.len() != $len {
                            return Err(anyhow::anyhow!(
                                "Expected array of length {}, found: {}",
                                $len,
                                arr.len()
                            ));
                        }

                        let mut iter = arr.into_iter();
                        $(
                            #[allow(non_snake_case)]
                            let $name = $name::from_value(iter.next().ok_or_else(|| {
                                anyhow::anyhow!("Not enough elements in array for tuple")
                            })?)?;
                        )+

                        Ok(($($name,)+))
                    } else {
                        Err(anyhow::anyhow!(
                            "Expected array, found: {:?}",
                            value.kind()
                        ))
                    }
                }

                fn to_value(self) -> expr::Value {
                    #[allow(non_snake_case)]
                    let ($($name,)+) = self;
                    let vals = vec![
                        $($name.to_value()),+
                    ];
                    expr::Value::from(expr::Array::from(vals))
                }

                fn kindof() -> expr::Kind {
                    expr::Kind::Literal(expr::Literal::Array(vec![
                        $($name::kindof(),)+
                    ]))
                }
            }
        )+
    };
}

impl_args_tuples! {
    1 => (A),
    2 => (A, B),
    3 => (A, B, C),
    4 => (A, B, C, D),
    5 => (A, B, C, D, E),
    6 => (A, B, C, D, E, F),
    7 => (A, B, C, D, E, F, G),
    8 => (A, B, C, D, E, F, G, H),
    9 => (A, B, C, D, E, F, G, H, I),
    10 => (A, B, C, D, E, F, G, H, I, J),
}

// TODO: Decided to match this with tuples (literal arrays) instead of an empty unit
// Not sure if this makes sense, so open to feedback, but the annoying thing with 
// making this NONE, is there there is no NONE type... only option<T>
impl Arg for () {
    fn is_value(value: &expr::Value) -> bool {
        value.as_array().is_some_and(|a| a.is_empty())
    }

    fn from_value(value: expr::Value) -> Result<Self> {
        if value.into_array().is_some_and(|a| a.is_empty()) {
            Ok(())
        } else {
            Err(Self::invalid_err())
        }
    }

    fn to_value(self) -> expr::Value {
        expr::Value::from(expr::Array::default())
    }

    fn kindof() -> expr::Kind {
        expr::Kind::Literal(expr::Literal::Array(vec![]))
    }
}

impl<T: Arg> Arg for Option<T> {
    fn is_value(value: &expr::Value) -> bool {
        value.is_none() || T::is_value(value)
    }

    fn from_value(value: expr::Value) -> Result<Self> {
        if value.is_none() {
            Ok(None)
        } else {
            T::from_value(value).map(Some)
        }
    }

    fn to_value(self) -> expr::Value {
        match self {
            Some(val) => val.to_value(),
            None => expr::Value::None,
        }
    }

    fn kindof() -> expr::Kind {
        expr::Kind::Option(Box::new(T::kindof()))
    }
}

// Either of 2, 3, 4, 5, 6 or 7
macro_rules! impl_arg_either {
    ($($enum:ident => $len:literal => ($($name:ident),+)),+ $(,)?) => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum $enum<$($name: Arg,)+> {
                $($name($name),)+
            }

            impl<$($name: Arg),+> Arg for $enum<$($name,)+>
            {
                fn is_value(value: &expr::Value) -> bool {
                    $($name::is_value(value) ||)+ false
                }

                fn from_value(value: expr::Value) -> Result<Self> {
                    $(if $name::is_value(&value) {
                        return Ok($enum::$name($name::from_value(value)?));
                    })+

                    Err(Self::invalid_err())
                }

                fn to_value(self) -> expr::Value {
                    match self {
                        $($enum::$name(val) => val.to_value(),)+
                    }
                }

                fn kindof() -> expr::Kind {
                    expr::Kind::Either(vec![
                        $($name::kindof(),)+
                    ])
                }
            }
        )+
    };
}

impl_arg_either! {
    Either2 => 2 => (A, B),
    Either3 => 2 => (A, B, C),
    Either4 => 2 => (A, B, C, D),
    Either5 => 2 => (A, B, C, D, E),
    Either6 => 2 => (A, B, C, D, E, F),
    Either7 => 2 => (A, B, C, D, E, F, G),
}