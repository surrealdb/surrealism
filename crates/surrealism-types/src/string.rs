use crate::controller::MemoryController;
use super::{array::TransferredArray, convert::{FromTransferrable, IntoTransferrable}, value::Value};
use anyhow::Result;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Strand(TransferredArray<u8>);

impl IntoTransferrable<Strand> for String {
	fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Strand> {
		Ok(Strand(self.as_bytes().to_vec().into_transferrable(controller)?))
	}
}

impl FromTransferrable<Strand> for String {
	fn from_transferrable(value: Strand, controller: &mut dyn MemoryController) -> Result<Self> {
		Ok(String::from_utf8(Vec::<u8>::from_transferrable(value.0, controller)?).expect("Found non UTF-8 characters while reconstructing string"))
	}
}

impl From<Strand> for Value {
    fn from(value: Strand) -> Self {
        Value::SR_VALUE_STRAND(value)
    }
}