use crate::controller::MemoryController;
use super::{convert::{FromTransferrable, IntoTransferrable}, value::Value};
use surrealdb::sql;
use anyhow::Result;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TransferredArray {
	pub ptr: u32,
	pub len: u32,
}

impl<T: Clone> IntoTransferrable<TransferredArray> for Vec<T>
{
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<TransferredArray> {
        let len = self.len();
        let byte_len = (len * std::mem::size_of::<T>()) as u32;
        let align = std::mem::align_of::<T>() as u32;
        let wasm_ptr = controller.alloc(byte_len, align)?;
        let memory = controller.mut_mem(wasm_ptr, byte_len);

        unsafe {
            let wasm_typed_slice: &mut [T] = std::slice::from_raw_parts_mut(
                memory.as_mut_ptr() as *mut T,
                len,
            );
            for (i, item) in self.into_iter().enumerate() {
                wasm_typed_slice[i] = item;
            }
        }

        Ok(TransferredArray {
            ptr: wasm_ptr,
            len: len as u32,
        })
    }
}

impl<T: Clone> FromTransferrable<TransferredArray> for Vec<T> {
	fn from_transferrable(value: TransferredArray, controller: &mut dyn MemoryController) -> Result<Self> {
		let ptr = value.ptr as usize;
		let len = value.len as usize;
		let byte_len = len * std::mem::size_of::<T>();

		let memory = controller.mut_mem(ptr as u32, byte_len as u32);

		let vec = unsafe {
			let typed_slice: &[T] = std::slice::from_raw_parts(
				memory.as_ptr() as *const T,
				len,
			);
			typed_slice.to_vec()
		};

		// Free the original memory in WASM after copying
		controller.free(value.ptr, byte_len as u32)?;

		Ok(vec)
	}
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Array(pub TransferredArray);

impl IntoTransferrable<Array> for sql::Array {
	fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Array> {
		Ok(Array(
            self
                .into_iter()
                .map(|v| v.into_transferrable(controller))
                .collect::<Result<Vec<Value>>>()?
                .into_transferrable(controller)?
        ))
	}
}

impl FromTransferrable<Array> for sql::Array {
	fn from_transferrable(value: Array, controller: &mut dyn MemoryController) -> Result<Self> {
        Ok(
            Vec::<Value>::from_transferrable(value.0, controller)?
                .into_iter()
                .map(|value| sql::Value::from_transferrable(value, controller))
                .collect::<Result<Vec<sql::Value>>>()?
                .into()
        )
    }
}

impl From<Array> for Value {
    fn from(value: Array) -> Self {
        Value::SR_VALUE_ARRAY(value)
    }
}