use std::fmt::Display;
use surrealdb::expr;

pub trait KindOf {
    fn kindof() -> expr::Kind;
}

macro_rules! impl_kindof {
    ($($ty:ty => $kind:expr),+ $(,)?) => {
        $(
            impl KindOf for $ty {
                fn kindof() -> expr::Kind {
                    $kind
                }
            }
        )+
    };
}

impl_kindof! {
    expr::Value => expr::Kind::Any,
    bool => expr::Kind::Bool,
    expr::Bytes => expr::Kind::Bytes,
    expr::Datetime => expr::Kind::Datetime,
    // Decimal => sql::Kind::Decimal,
    expr::Duration => expr::Kind::Duration,
    f64 => expr::Kind::Float,
    i64 => expr::Kind::Int,
    expr::Number => expr::Kind::Number,
    expr::Object => expr::Kind::Object,
    String => expr::Kind::String,
    expr::Regex => expr::Kind::Regex,
    expr::Thing => expr::Kind::Record(vec![]),
    expr::Geometry => expr::Kind::Geometry(vec![]),
    expr::Range => expr::Kind::Range,
    expr::Array => expr::Kind::Array(Box::new(expr::Kind::Any), None),
}

impl<T: KindOf> KindOf for Option<T> {
    fn kindof() -> expr::Kind {
        expr::Kind::Option(Box::new(T::kindof()))
    }
}

impl<T: KindOf, E: Display> KindOf for Result<T, E> {
    fn kindof() -> expr::Kind {
        T::kindof()
    }
}

impl KindOf for () {
    fn kindof() -> expr::Kind {
        expr::Kind::Any
    }
}