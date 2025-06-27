use std::marker::PhantomData;
use crate::{controller::MemoryController, err::Error};
use super::{array::Array, bytes::Bytes, datetime::Datetime, duration::Duration, thing::Thing, uuid::Uuid, value::{Number, Object, Value}};
use surrealdb::sql;
use surrealdb::sql::Kind;
use anyhow::Result;

pub trait Transfer {
    /// Transfers the value into WASM memory, returns a `Transferred` handle
    fn transfer(self, controller: &mut dyn MemoryController) -> Result<Transferred<Self>>
    where
        Self: Sized;

    /// Default implementation of `accept`, does nothing unless overridden
    fn receive(transferred: Transferred<Self>, controller: &mut dyn MemoryController) -> Result<Self>
    where
        Self: Sized;
}

impl<T: Clone> Transfer for T {
    fn transfer(self, controller: &mut dyn MemoryController) -> Result<Transferred<T>> {
        let len = std::mem::size_of::<T>() as u32;
		let align = std::mem::align_of::<T>() as u32;
        let ptr = controller.alloc(len, align)?;
        let memory = controller.mut_mem(ptr, len);

        unsafe {
            let src_ptr = &self as *const T as *const u8;
            let src = std::slice::from_raw_parts(src_ptr, len as usize);
            memory.copy_from_slice(src);
        }

        std::mem::forget(self);

        Ok(Transferred::from_ptr(ptr))
    }

    fn receive(transferred: Transferred<T>, controller: &mut dyn MemoryController) -> Result<Self> {
		let ptr = transferred.ptr();
		let len = transferred.len();
        let memory = controller.mut_mem(ptr, len);

        let val = unsafe {
            let typed_ptr = memory.as_ptr() as *const T;
            (*typed_ptr).clone()
        };

        controller.free(ptr, len)?;

        Ok(val)
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Transferred<T>(u32, PhantomData<T>);

impl<T> Transferred<T> {
	pub fn from_ptr(ptr: u32) -> Self {
		Self(ptr, Default::default())
	}

	pub fn ptr(&self) -> u32 {
		self.0
	}

	pub fn len(&self) -> u32 {
		std::mem::size_of::<T>() as u32
	}
}

impl<T> From<Transferred<T>> for u32 {
	fn from(value: Transferred<T>) -> Self {
		value.0
	}
}

impl<T> From<u32> for Transferred<T> {
	fn from(ptr: u32) -> Self {
		Transferred::from_ptr(ptr)
	}
}

//////////////////////////
/// INTO TRANSFERRABLE ///
//////////////////////////

pub trait IntoTransferrable<R> {
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<R>;
}

impl IntoTransferrable<Value> for Value {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(self)
	}
}

impl IntoTransferrable<Value> for bool {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_BOOL(self))
	}
}

impl IntoTransferrable<Value> for i64 {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_NUMBER(Number::SR_NUMBER_INT(self)))
	}
}

impl IntoTransferrable<Value> for f64 {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_NUMBER(Number::SR_NUMBER_FLOAT(self)))
	}
}

impl IntoTransferrable<Value> for String {
	fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_STRAND(self.into_transferrable(controller)?))
	}
}

impl IntoTransferrable<Value> for Duration {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_DURATION(self))
	}
}

impl IntoTransferrable<Value> for sql::Duration {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_DURATION(self.into()))
	}
}

impl IntoTransferrable<Value> for Datetime {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_DATETIME(self))
	}
}

impl IntoTransferrable<Value> for sql::Datetime {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_DATETIME(self.into()))
	}
}

impl IntoTransferrable<Value> for Uuid {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_UUID(self))
	}
}

impl IntoTransferrable<Value> for sql::Uuid {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_UUID(self.into()))
	}
}

impl IntoTransferrable<Value> for Array {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_ARRAY(self))
	}
}

impl IntoTransferrable<Value> for sql::Array {
	fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_ARRAY(self.into_transferrable(controller)?))
	}
}

impl IntoTransferrable<Value> for Object {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_OBJECT(self))
	}
}

impl IntoTransferrable<Value> for sql::Object {
	fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_OBJECT(self.into_transferrable(controller)?))
	}
}

impl IntoTransferrable<Value> for Bytes {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_BYTES(self))
	}
}

impl IntoTransferrable<Value> for sql::Bytes {
	fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_BYTES(self.into_transferrable(controller)?))
	}
}

impl IntoTransferrable<Value> for Thing {
	fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_THING(self))
	}
}

impl IntoTransferrable<Value> for sql::Thing {
	fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Value> {
		Ok(Value::SR_VALUE_THING(self.into_transferrable(controller)?))
	}
}


//////////////////////////
/// FROM TRANSFERRABLE ///
//////////////////////////

pub trait FromTransferrable<T> {
    fn from_transferrable(value: T, controller: &mut dyn MemoryController) -> Result<Self>
    where Self: Sized;
}

impl FromTransferrable<Value> for bool {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_BOOL(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Bool).into())
        }
	}
}

impl FromTransferrable<Value> for i64 {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_NUMBER(Number::SR_NUMBER_INT(x)) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Int).into())
        }
	}
}

impl FromTransferrable<Value> for f64 {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_NUMBER(Number::SR_NUMBER_FLOAT(x)) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Float).into())
        }
	}
}

impl FromTransferrable<Value> for Number {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_NUMBER(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Number).into())
        }
	}
}

impl FromTransferrable<Value> for sql::Number {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_NUMBER(x) = value {
            Ok(x.into())
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Number).into())
        }
	}
}

impl FromTransferrable<Value> for String {
	fn from_transferrable(value: Value, controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_STRAND(x) = value {
            Ok(String::from_transferrable(x, controller)?)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::String).into())
        }
	}
}

impl FromTransferrable<Value> for Duration {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_DURATION(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Duration).into())
        }
	}
}

impl FromTransferrable<Value> for sql::Duration {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_DURATION(x) = value {
            Ok(x.into())
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Duration).into())
        }
	}
}

impl FromTransferrable<Value> for Datetime {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_DATETIME(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Datetime).into())
        }
	}
}

impl FromTransferrable<Value> for sql::Datetime {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_DATETIME(x) = value {
            Ok(x.try_into()?)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Datetime).into())
        }
	}
}

impl FromTransferrable<Value> for Uuid {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_UUID(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Uuid).into())
        }
	}
}

impl FromTransferrable<Value> for sql::Uuid {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_UUID(x) = value {
            Ok(x.into())
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Uuid).into())
        }
	}
}

impl FromTransferrable<Value> for Array {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_ARRAY(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Array(Box::new(Kind::Any), None)).into())
        }
	}
}

impl FromTransferrable<Value> for sql::Array {
	fn from_transferrable(value: Value, controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_ARRAY(x) = value {
            Ok(sql::Array::from_transferrable(x, controller)?)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Array(Box::new(Kind::Any), None)).into())
        }
	}
}

impl FromTransferrable<Value> for Object {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_OBJECT(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Object).into())
        }
	}
}

impl FromTransferrable<Value> for sql::Object {
	fn from_transferrable(value: Value, controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_OBJECT(x) = value {
            Ok(sql::Object::from_transferrable(x, controller)?)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Object).into())
        }
	}
}

impl FromTransferrable<Value> for Bytes {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_BYTES(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Bytes).into())
        }
	}
}

impl FromTransferrable<Value> for sql::Bytes {
	fn from_transferrable(value: Value, controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_BYTES(x) = value {
            Ok(sql::Bytes::from_transferrable(x, controller)?)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Bytes).into())
        }
	}
}

impl FromTransferrable<Value> for Thing {
	fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_THING(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Record(vec![])).into())
        }
	}
}

impl FromTransferrable<Value> for sql::Thing {
	fn from_transferrable(value: Value, controller: &mut dyn MemoryController) -> Result<Self> {
		if let Value::SR_VALUE_THING(x) = value {
            Ok(sql::Thing::from_transferrable(x, controller)?)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Record(vec![])).into())
        }
	}
}