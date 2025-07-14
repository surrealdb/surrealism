use super::{
    array::Array,
    bytes::Bytes,
    datetime::Datetime,
    duration::Duration,
    thing::Thing,
    uuid::Uuid,
    value::{Number, Object, Value},
};
use crate::{controller::MemoryController, err::Error, kind::KindOf};
use anyhow::Result;
use std::marker::PhantomData;
use surrealdb::sql;
use surrealdb::sql::Kind;

pub trait Transfer {
    /// Transfers the value into WASM memory, returns a `Transferred` handle
    fn transfer(self, controller: &mut dyn MemoryController) -> Result<Transferred<Self>>
    where
        Self: Sized;

    /// Default implementation of `accept`, does nothing unless overridden
    fn receive(
        transferred: Transferred<Self>,
        controller: &mut dyn MemoryController,
    ) -> Result<Self>
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

    #[allow(clippy::len_without_is_empty)]
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

impl<T> TryFrom<i32> for Transferred<T> {
    type Error = anyhow::Error;
    fn try_from(ptr: i32) -> Result<Self> {
        if ptr < 0 {
            Err(anyhow::anyhow!(
                "Failed to process transfer, pointer is negative"
            ))
        } else {
            Ok(Transferred::from_ptr(ptr as u32))
        }
    }
}

// Transferrable

pub trait Transferrable<T = Value> {
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<T>;
    fn from_transferrable(value: T, controller: &mut dyn MemoryController) -> Result<Self>
    where
        Self: Sized;
}

impl<T: Clone + Transferrable<Value> + KindOf> Transferrable for Vec<T> {
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_ARRAY(Array(
            self.into_iter()
                .map(|x| x.into_transferrable(controller))
                .collect::<Result<Vec<Value>>>()?
                .into_transferrable(controller)?,
        )))
    }

    fn from_transferrable(value: Value, controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_ARRAY(Array(x)) = value {
            Vec::<Value>::from_transferrable(x, controller)?
                .into_iter()
                .map(|x| T::from_transferrable(x, controller))
                .collect::<Result<Vec<T>>>()
        } else {
            Err(
                Error::UnexpectedType(value.kindof(), Kind::Array(Box::new(T::kindof()), None))
                    .into(),
            )
        }
    }
}

impl Transferrable for Value {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(self)
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        Ok(value)
    }
}

impl Transferrable for bool {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_BOOL(self))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_BOOL(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Bool).into())
        }
    }
}

impl Transferrable for i64 {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_NUMBER(Number::SR_NUMBER_INT(self)))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_NUMBER(Number::SR_NUMBER_INT(x)) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Int).into())
        }
    }
}

impl Transferrable for f64 {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_NUMBER(Number::SR_NUMBER_FLOAT(self)))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_NUMBER(Number::SR_NUMBER_FLOAT(x)) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Float).into())
        }
    }
}

impl Transferrable for Number {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_NUMBER(self))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_NUMBER(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Number).into())
        }
    }
}

impl Transferrable for sql::Number {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_NUMBER(self.into()))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_NUMBER(x) = value {
            Ok(x.into())
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Number).into())
        }
    }
}

impl Transferrable for String {
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_STRAND(self.into_transferrable(controller)?))
    }

    fn from_transferrable(value: Value, controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_STRAND(x) = value {
            Ok(String::from_transferrable(x, controller)?)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::String).into())
        }
    }
}

impl Transferrable for Duration {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_DURATION(self))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_DURATION(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Duration).into())
        }
    }
}

impl Transferrable for sql::Duration {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_DURATION(self.into()))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_DURATION(x) = value {
            Ok(x.into())
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Duration).into())
        }
    }
}

impl Transferrable for Datetime {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_DATETIME(self))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_DATETIME(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Datetime).into())
        }
    }
}

impl Transferrable for sql::Datetime {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_DATETIME(self.into()))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_DATETIME(x) = value {
            Ok(x.try_into()?)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Datetime).into())
        }
    }
}

impl Transferrable for Uuid {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_UUID(self))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_UUID(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Uuid).into())
        }
    }
}

impl Transferrable for sql::Uuid {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_UUID(self.into()))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_UUID(x) = value {
            Ok(x.into())
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Uuid).into())
        }
    }
}

impl Transferrable for Array {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_ARRAY(self))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_ARRAY(x) = value {
            Ok(x)
        } else {
            Err(
                Error::UnexpectedType(value.kindof(), Kind::Array(Box::new(Kind::Any), None))
                    .into(),
            )
        }
    }
}

impl Transferrable for sql::Array {
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_ARRAY(self.into_transferrable(controller)?))
    }

    fn from_transferrable(value: Value, controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_ARRAY(x) = value {
            Ok(sql::Array::from_transferrable(x, controller)?)
        } else {
            Err(
                Error::UnexpectedType(value.kindof(), Kind::Array(Box::new(Kind::Any), None))
                    .into(),
            )
        }
    }
}

impl Transferrable for Object {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_OBJECT(self))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_OBJECT(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Object).into())
        }
    }
}

impl Transferrable for sql::Object {
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_OBJECT(self.into_transferrable(controller)?))
    }

    fn from_transferrable(value: Value, controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_OBJECT(x) = value {
            Ok(sql::Object::from_transferrable(x, controller)?)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Object).into())
        }
    }
}

impl Transferrable for Bytes {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_BYTES(self))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_BYTES(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Bytes).into())
        }
    }
}

impl Transferrable for sql::Bytes {
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_BYTES(self.into_transferrable(controller)?))
    }

    fn from_transferrable(value: Value, controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_BYTES(x) = value {
            Ok(sql::Bytes::from_transferrable(x, controller)?)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Bytes).into())
        }
    }
}

impl Transferrable for Thing {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_THING(self))
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_THING(x) = value {
            Ok(x)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Record(vec![])).into())
        }
    }
}

impl Transferrable for () {
    fn into_transferrable(self, _controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_NONE)
    }

    fn from_transferrable(value: Value, _controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_NONE = value {
            Ok(())
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Any).into())
        }
    }
}

impl Transferrable for sql::Thing {
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Value> {
        Ok(Value::SR_VALUE_THING(self.into_transferrable(controller)?))
    }

    fn from_transferrable(value: Value, controller: &mut dyn MemoryController) -> Result<Self> {
        if let Value::SR_VALUE_THING(x) = value {
            Ok(sql::Thing::from_transferrable(x, controller)?)
        } else {
            Err(Error::UnexpectedType(value.kindof(), Kind::Record(vec![])).into())
        }
    }
}
