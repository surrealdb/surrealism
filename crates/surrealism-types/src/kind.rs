use std::collections::BTreeMap;
use surrealdb::sql;
use crate::{controller::MemoryController, err::Error, string::Strand};
use super::{array::TransferredArray, convert::{FromTransferrable, IntoTransferrable, Transfer, Transferred}, duration::Duration, number::Number, object::KeyValuePair, utils::COption};
use anyhow::Result;

#[repr(C)]
#[derive(Clone, Debug)]
pub enum Kind {
	Any,
	Null,
	Bool,
	Bytes,
	Datetime,
	Decimal,
	Duration,
	Float,
	Int,
	Number,
	Object,
	Point,
	String,
	Uuid,
	Regex,
	Record(TransferredArray),
	Geometry(TransferredArray),
	Option(Transferred),
	Either(TransferredArray),
	Set(Transferred, COption<u64>),
	Array(Transferred, COption<u64>),
	Function(COption<TransferredArray>, COption<Transferred>),
	Range,
	Literal(Literal),
}

impl IntoTransferrable<Kind> for sql::Kind {
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Kind> {
        match self {
            Self::Any => Ok(Kind::Any),
            Self::Null => Ok(Kind::Null),
            Self::Bool => Ok(Kind::Bool),
            Self::Bytes => Ok(Kind::Bytes),
            Self::Datetime => Ok(Kind::Datetime),
            Self::Decimal => Ok(Kind::Decimal),
            Self::Duration => Ok(Kind::Duration),
            Self::Float => Ok(Kind::Float),
            Self::Int => Ok(Kind::Int),
            Self::Number => Ok(Kind::Number),
            Self::Object => Ok(Kind::Object),
            Self::Point => Ok(Kind::Point),
            Self::String => Ok(Kind::String),
            Self::Uuid => Ok(Kind::Uuid),
            Self::Regex => Ok(Kind::Regex),
            Self::Record(x) => Ok(Kind::Record(x
                .into_iter()
                .map(|x| x.0.into_transferrable(controller))
                .collect::<Result<Vec<Strand>>>()?
                .into_transferrable(controller)?
            )),
            Self::Geometry(x) => Ok(Kind::Geometry(x
                .into_iter()
                .map(|x| x.into_transferrable(controller))
                .collect::<Result<Vec<Strand>>>()?
                .into_transferrable(controller)?
            )),
            Self::Option(x) => Ok(Kind::Option(x.into_transferrable(controller)?.transfer(controller)?)),
            Self::Either(x) => Ok(Kind::Either(x
                .into_iter()
                .map(|x| x.into_transferrable(controller))
                .collect::<Result<Vec<Kind>>>()?
                .into_transferrable(controller)?
            )),
            Self::Set(x, len) => Ok(Kind::Set(
                x.into_transferrable(controller)?.transfer(controller)?,
                len.into(),
            )),
            Self::Array(x, len) => Ok(Kind::Array(
                x.into_transferrable(controller)?.transfer(controller)?,
                len.into(),
            )),
            Self::Function(args, returns) => Ok(Kind::Function(
                args
                    .map(|args| -> Result<TransferredArray> { 
                        args
                            .into_iter()
                            .map(|x| x.into_transferrable(controller))
                            .collect::<Result<Vec<Kind>>>()?
                            .into_transferrable(controller)
                    })
                    .transpose()?
                    .into(),
                returns
                    .map(|x| -> Result<Transferred> { 
                        Ok(x.into_transferrable(controller)?.transfer(controller)?)
                    })
                    .transpose()?
                    .into(),
            )),
            Self::Range => Ok(Kind::Range),
            Self::Literal(x) => Ok(Kind::Literal(x.into_transferrable(controller)?)),
            _ => Err(Error::UnsupportedKind.into()),
        }
    }
}

impl FromTransferrable<Kind> for sql::Kind {
    fn from_transferrable(value: Kind, controller: &mut dyn MemoryController) -> Result<Self> {
        match value {
            Kind::Any => Ok(Self::Any),
            Kind::Null => Ok(Self::Null),
            Kind::Bool => Ok(Self::Bool),
            Kind::Bytes => Ok(Self::Bytes),
            Kind::Datetime => Ok(Self::Datetime),
            Kind::Decimal => Ok(Self::Decimal),
            Kind::Duration => Ok(Self::Duration),
            Kind::Float => Ok(Self::Float),
            Kind::Int => Ok(Self::Int),
            Kind::Number => Ok(Self::Number),
            Kind::Object => Ok(Self::Object),
            Kind::Point => Ok(Self::Point),
            Kind::String => Ok(Self::String),
            Kind::Uuid => Ok(Self::Uuid),
            Kind::Regex => Ok(Self::Regex),
            Kind::Record(x) => Ok(Self::Record(
                Vec::<Strand>::from_transferrable(x, controller)?
                    .into_iter()
                    .map(|x| String::from_transferrable(x, controller).map(Into::into))
                    .collect::<Result<Vec<sql::Table>>>()?
            )),
            Kind::Geometry(x) => Ok(Self::Geometry(
                Vec::<Strand>::from_transferrable(x, controller)?
                    .into_iter()
                    .map(|x| String::from_transferrable(x, controller))
                    .collect::<Result<Vec<String>>>()?
            )),
            Kind::Option(x) => Ok(Self::Option(Box::new(sql::Kind::from_transferrable(Kind::receive(x, controller)?, controller)?))),
            Kind::Either(x) => Ok(Self::Either(
                Vec::<Kind>::from_transferrable(x, controller)?
                    .into_iter()
                    .map(|x| sql::Kind::from_transferrable(x, controller))
                    .collect::<Result<Vec<sql::Kind>>>()?
            )),
            Kind::Set(x, len) => Ok(Self::Set(
                Box::new(sql::Kind::from_transferrable(Kind::receive(x, controller)?, controller)?),
                len.into(),
            )),
            Kind::Array(x, len) => Ok(Self::Array(
                Box::new(sql::Kind::from_transferrable(Kind::receive(x, controller)?, controller)?),
                len.into(),
            )),
            Kind::Function(args, returns) => Ok(Self::Function(
                Option::<TransferredArray>::from(args)
                    .map(|x| -> Result<Vec<sql::Kind>> {
                        Vec::<Kind>::from_transferrable(x, controller)?
                            .into_iter()
                            .map(|x| sql::Kind::from_transferrable(x, controller))
                            .collect::<Result<Vec<sql::Kind>>>()
                    })
                    .transpose()?,
                Option::<Transferred>::from(returns)
                    .map(|x| -> Result<Box<sql::Kind>> {
                        Ok(Box::new(sql::Kind::from_transferrable(Kind::receive(x, controller)?, controller)?))
                    })
                    .transpose()?,
            )),
            Kind::Range => Ok(Self::Range),
            Kind::Literal(x) => Ok(Self::Literal(sql::Literal::from_transferrable(x, controller)?))
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub enum Literal {
	String(Strand),
	Number(Number),
	Duration(Duration),
	Array(TransferredArray),
	Object(TransferredArray),
	DiscriminatedObject(Strand, TransferredArray),
	Bool(bool),
}

impl IntoTransferrable<Literal> for sql::Literal {
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Literal> {
        match self {
            Self::String(x) => Ok(Literal::String(x.0.into_transferrable(controller)?)),
            Self::Number(x) => Ok(Literal::Number(x.into())),
            Self::Duration(x) => Ok(Literal::Duration(x.into())),
            Self::Bool(x) => Ok(Literal::Bool(x)),
            Self::Array(x) => Ok(Literal::Array(x
                .into_iter()
                .map(|x| x.into_transferrable(controller))
                .collect::<Result<Vec<Kind>>>()?
                .into_transferrable(controller)?
            )),
            Self::Object(x) => Ok(Literal::Object(x.into_transferrable(controller)?)),
            Self::DiscriminatedObject(key, variants) => Ok(Literal::DiscriminatedObject(
                key.into_transferrable(controller)?,
                variants
                    .into_iter()
                    .map(|x| x.into_transferrable(controller))
                    .collect::<Result<Vec<TransferredArray>>>()?
                    .into_transferrable(controller)?
            )),
            _ => Err(Error::UnsupportedKind.into())
        }
    }
}

impl FromTransferrable<Literal> for sql::Literal {
    fn from_transferrable(value: Literal, controller: &mut dyn MemoryController) -> Result<Self> {
        match value {
            Literal::String(x) => Ok(Self::String(String::from_transferrable(x, controller)?.into())),
            Literal::Number(x) => Ok(Self::Number(x.into())),
            Literal::Duration(x) => Ok(Self::Duration(x.into())),
            Literal::Bool(x) => Ok(Self::Bool(x)),
            Literal::Array(x) => Ok(Self::Array(
                Vec::<Kind>::from_transferrable(x, controller)?
                    .into_iter()
                    .map(|x| sql::Kind::from_transferrable(x, controller))
                    .collect::<Result<Vec<sql::Kind>>>()?
            )),
            Literal::Object(x) => Ok(Self::Object(BTreeMap::<String, sql::Kind>::from_transferrable(x, controller)?)),
            Literal::DiscriminatedObject(key, x) => Ok(Self::DiscriminatedObject(
                String::from_transferrable(key, controller)?,
                Vec::<TransferredArray>::from_transferrable(x, controller)?
                    .into_iter()
                    .map(|x| BTreeMap::<String, sql::Kind>::from_transferrable(x, controller))
                    .collect::<Result<Vec<BTreeMap<String, sql::Kind>>>>()?
            )),
        }
    }
}

impl IntoTransferrable<TransferredArray> for BTreeMap<String, sql::Kind> {
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<TransferredArray> {
        self
            .into_iter()
            .map(|x| x.into_transferrable(controller))
            .collect::<Result<Vec<KeyValuePair<sql::Kind, Kind>>>>()?
            .into_transferrable(controller)
    }
}

impl FromTransferrable<TransferredArray> for BTreeMap<String, sql::Kind> {
    fn from_transferrable(value: TransferredArray, controller: &mut dyn MemoryController) -> Result<Self> {
        Ok(BTreeMap::from_iter(Vec::<KeyValuePair<sql::Kind, Kind>>::from_transferrable(value, controller)?
            .into_iter()
            .map(|x| FromTransferrable::from_transferrable(x, controller))
            .collect::<Result<Vec<(String, sql::Kind)>>>()?))
    }
}

pub trait KindOf {
    fn kindof() -> sql::Kind;
}

macro_rules! impl_kindof {
    ($($ty:ty => $kind:expr),+ $(,)?) => {
        $(
            impl KindOf for $ty {
                fn kindof() -> sql::Kind {
                    $kind
                }
            }
        )+
    };
}

impl_kindof! {
    sql::Value => sql::Kind::Any,
    bool => sql::Kind::Bool,
    sql::Bytes => sql::Kind::Bytes,
    sql::Datetime => sql::Kind::Datetime,
    // Decimal => sql::Kind::Decimal,
    sql::Duration => sql::Kind::Duration,
    f64 => sql::Kind::Float,
    i64 => sql::Kind::Int,
    sql::Number => sql::Kind::Number,
    sql::Object => sql::Kind::Object,
    String => sql::Kind::String,
    sql::Regex => sql::Kind::Regex,
    sql::Thing => sql::Kind::Record(vec![]),
    sql::Geometry => sql::Kind::Geometry(vec![]),
    sql::Range => sql::Kind::Range,
    sql::Array => sql::Kind::Array(Box::new(sql::Kind::Any), None),
}

impl<T: KindOf> KindOf for Option<T> {
    fn kindof() -> sql::Kind {
        sql::Kind::Option(Box::new(T::kindof()))
    }
}