use std::collections::BTreeMap;

use surrealdb::sql;
use crate::controller::MemoryController;
use super::{array::TransferredArray, convert::{FromTransferrable, IntoTransferrable}, string::string_t, utils::CStringExt2, value::Value};
use anyhow::Result;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Object(TransferredArray);

impl IntoTransferrable<Object> for sql::Object {
	fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Object> {
		Ok(Object(self.0
			.into_iter()
			.map(|(k, v)| v.into_transferrable(controller).map(|value| KeyValuePair {
				key: k.to_string_t(),
				value
			}))
			.collect::<Result<Vec<KeyValuePair>>>()?
			.into_transferrable(controller)?
		))
	}
}

impl FromTransferrable<Object> for sql::Object {
	fn from_transferrable(value: Object, controller: &mut dyn MemoryController) -> Result<Self> {
		let entries = Vec::<KeyValuePair>::from_transferrable(value.0, controller)?
			.into_iter()
			.map(|KeyValuePair { key, value }| {
				sql::Value::from_transferrable(value, controller)
					.map(|v| (key.into(), v))
			})
			.collect::<Result<Vec<(String, sql::Value)>>>()?;

		Ok(BTreeMap::from_iter(entries).into())
    }
}

impl From<Object> for Value {
    fn from(value: Object) -> Self {
        Value::SR_VALUE_OBJECT(value)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct KeyValuePair<T = Value> {
	pub key: string_t,
	pub value: T,
}