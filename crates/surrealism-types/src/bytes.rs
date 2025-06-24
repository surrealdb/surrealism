use surrealdb::sql;
use crate::controller::MemoryController;
use super::{array::TransferredArray, convert::{FromTransferrable, IntoTransferrable}, value::Value};
use anyhow::Result;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Bytes(TransferredArray);

impl IntoTransferrable<Bytes> for sql::Bytes {
	fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Bytes> {
		Ok(Bytes(self.into_inner().into_transferrable(controller)?))
	}
}

impl FromTransferrable<Bytes> for sql::Bytes {
	fn from_transferrable(value: Bytes, controller: &mut dyn MemoryController) -> Result<Self> {
		Ok(Vec::<u8>::from_transferrable(value.0, controller)?.into())
	}
}

impl From<Bytes> for Value {
    fn from(value: Bytes) -> Self {
        Value::SR_VALUE_BYTES(value)
    }
}