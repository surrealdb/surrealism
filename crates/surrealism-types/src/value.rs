use surrealdb::sql;
use crate::controller::MemoryController;
use crate::string::Strand;
use super::convert::{FromTransferrable, IntoTransferrable};
use super::datetime::Datetime;
pub use super::{array::Array, number::Number, object::Object};
use super::{bytes::Bytes, thing::Thing, uuid::Uuid};
use super::duration::Duration;
use surrealdb::sql::Kind;
use anyhow::Result;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub enum Value {
	#[default]
	SR_VALUE_NONE,
	SR_VALUE_NULL,
	SR_VALUE_BOOL(bool),
	SR_VALUE_NUMBER(Number),
	SR_VALUE_STRAND(Strand),
	SR_VALUE_DURATION(Duration),
	SR_VALUE_DATETIME(Datetime),
	SR_VALUE_UUID(Uuid),
	SR_VALUE_ARRAY(Array),
	SR_VALUE_OBJECT(Object),
	// Geometry(Geometry),
	SR_VALUE_BYTES(Bytes),
	SR_VALUE_THING(Thing),
}

impl IntoTransferrable<Value> for sql::Value {
	fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Value> {
		match self {
			Self::None => Ok(Value::SR_VALUE_NONE),
			Self::Null => Ok(Value::SR_VALUE_NULL),
			Self::Bool(x) => Ok(Value::SR_VALUE_BOOL(x)),
			Self::Number(n) => Ok(Value::SR_VALUE_NUMBER(n.into())),
			Self::Strand(s) => Ok(Value::SR_VALUE_STRAND(s.0.into_transferrable(controller)?)),
			Self::Duration(d) => Ok(Value::SR_VALUE_DURATION(d.into())),
			Self::Datetime(dt) => Ok(Value::SR_VALUE_DATETIME(dt.into())),
			Self::Uuid(u) => Ok(Value::SR_VALUE_UUID(u.into())),
			Self::Array(x) => Ok(Value::SR_VALUE_ARRAY(x.into_transferrable(controller)?)),
			Self::Object(x) => Ok(Value::SR_VALUE_OBJECT(x.into_transferrable(controller)?)),
			Self::Bytes(x) => Ok(Value::SR_VALUE_BYTES(x.into_transferrable(controller)?)),
			Self::Thing(x) => Ok(Value::SR_VALUE_THING(x.into_transferrable(controller)?)),
			Self::Geometry(_) => todo!(),
			_ => unimplemented!("other variants shouldn't be returned"),
		}
	}
}

impl FromTransferrable<Value> for sql::Value {
	fn from_transferrable(value: Value, controller: &mut dyn MemoryController) -> Result<Self> {
		match value {
			Value::SR_VALUE_NONE => Ok(Self::None),
			Value::SR_VALUE_NULL => Ok(Self::Null),
			Value::SR_VALUE_BOOL(x) => Ok(Self::Bool(x)),
			Value::SR_VALUE_NUMBER(n) => Ok(Self::Number(n.into())),
			Value::SR_VALUE_STRAND(s) => Ok(String::from_transferrable(s, controller)?.into()),
			Value::SR_VALUE_DURATION(d) => Ok(Self::Duration(d.into())),
			Value::SR_VALUE_DATETIME(d) => Ok(Self::Datetime(d.try_into()?)),
			Value::SR_VALUE_UUID(u) => Ok(Self::Uuid(u.into())),
			Value::SR_VALUE_ARRAY(x) => Ok(Self::Array(sql::Array::from_transferrable(x, controller)?)),
			Value::SR_VALUE_OBJECT(x) => Ok(Self::Object(sql::Object::from_transferrable(x, controller)?)),
			Value::SR_VALUE_BYTES(x) => Ok(Self::Bytes(sql::Bytes::from_transferrable(x, controller)?)),
			Value::SR_VALUE_THING(x) => Ok(Self::Thing(sql::Thing::from_transferrable(x, controller)?)),
		}
	}
}

impl Value {
	pub fn kindof(&self) -> Kind {
		match self {
			Self::SR_VALUE_NONE => Kind::Any,
			Self::SR_VALUE_NULL => Kind::Null,
			Self::SR_VALUE_BOOL(_) => Kind::Bool,
			Self::SR_VALUE_NUMBER(_) => Kind::Number,
			Self::SR_VALUE_STRAND(_) => Kind::String,
			Self::SR_VALUE_DURATION(_) => Kind::Duration,
			Self::SR_VALUE_DATETIME(_) => Kind::Datetime,
			Self::SR_VALUE_UUID(_) => Kind::Uuid,
			Self::SR_VALUE_ARRAY(_) => Kind::Array(Box::new(Kind::Any), None),
			Self::SR_VALUE_OBJECT(_) => Kind::Object,
			Self::SR_VALUE_BYTES(_) => Kind::Bytes,
			Self::SR_VALUE_THING(_) => Kind::Record(vec![]),
		}
	}
}