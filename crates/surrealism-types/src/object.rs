use std::{collections::BTreeMap, marker::PhantomData};

use surrealdb::sql;
use crate::{controller::MemoryController, string::Strand};
use super::{array::TransferredArray, convert::{FromTransferrable, IntoTransferrable}, value::Value};
use anyhow::Result;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Object(TransferredArray<KeyValuePair>);

impl IntoTransferrable<Object> for sql::Object {
	fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Object> {
		Ok(Object(self.0
			.into_iter()
			.map(|x| x.into_transferrable(controller))
			.collect::<Result<Vec<KeyValuePair>>>()?
			.into_transferrable(controller)?
		))
	}
}

impl FromTransferrable<Object> for sql::Object {
	fn from_transferrable(value: Object, controller: &mut dyn MemoryController) -> Result<Self> {
		let entries = Vec::<KeyValuePair>::from_transferrable(value.0, controller)?
			.into_iter()
			.map(|x| FromTransferrable::from_transferrable(x, controller))
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
pub struct KeyValuePair<X = sql::Value, T = Value>
where
	X: IntoTransferrable<T> + FromTransferrable<T>
{
	pub key: Strand,
	pub value: T,
	_phantom: PhantomData<X>,
}

impl<X, T> IntoTransferrable<KeyValuePair<X, T>> for (String, X)
where
	X: IntoTransferrable<T> + FromTransferrable<T>
{
	fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<KeyValuePair<X, T>> {
		let key = self.0.into_transferrable(controller)?;
		let value = self.1.into_transferrable(controller)?;
		Ok(KeyValuePair {
			key,
			value,
			_phantom: Default::default(),
		})
	}
}

impl<X, T> FromTransferrable<KeyValuePair<X, T>> for (String, X)
where
	X: IntoTransferrable<T> + FromTransferrable<T>
{
	fn from_transferrable(KeyValuePair { key, value, .. }: KeyValuePair<X, T>, controller: &mut dyn MemoryController) -> Result<Self> {
		Ok((
			String::from_transferrable(key, controller)?,
			X::from_transferrable(value, controller)?,
		))
	}
}