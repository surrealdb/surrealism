use std::{collections::BTreeMap, marker::PhantomData};

use super::{array::TransferredArray, convert::Transferrable, value::Value};
use crate::{controller::MemoryController, string::Strand};
use anyhow::Result;
use surrealdb::sql;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Object(TransferredArray<KeyValuePair>);

impl Transferrable<Object> for sql::Object {
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Object> {
        Ok(Object(
            self.0
                .into_iter()
                .map(|x| x.into_transferrable(controller))
                .collect::<Result<Vec<KeyValuePair>>>()?
                .into_transferrable(controller)?,
        ))
    }

    fn from_transferrable(value: Object, controller: &mut dyn MemoryController) -> Result<Self> {
        let entries = Vec::<KeyValuePair>::from_transferrable(value.0, controller)?
            .into_iter()
            .map(|x| Transferrable::from_transferrable(x, controller))
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
    X: Transferrable<T>,
{
    pub key: Strand,
    pub value: T,
    _phantom: PhantomData<X>,
}

impl<X, T> Transferrable<KeyValuePair<X, T>> for (String, X)
where
    X: Transferrable<T>,
{
    fn into_transferrable(
        self,
        controller: &mut dyn MemoryController,
    ) -> Result<KeyValuePair<X, T>> {
        let key = self.0.into_transferrable(controller)?;
        let value = self.1.into_transferrable(controller)?;
        Ok(KeyValuePair {
            key,
            value,
            _phantom: Default::default(),
        })
    }

    fn from_transferrable(
        KeyValuePair { key, value, .. }: KeyValuePair<X, T>,
        controller: &mut dyn MemoryController,
    ) -> Result<Self> {
        Ok((
            String::from_transferrable(key, controller)?,
            X::from_transferrable(value, controller)?,
        ))
    }
}
