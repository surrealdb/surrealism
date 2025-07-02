use super::{array::Array, convert::Transferrable, object::Object, value::Value};
use crate::{controller::MemoryController, string::Strand};
use anyhow::Result;
use std::fmt::Debug;
use surrealdb::sql;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Thing {
    tb: Strand,
    id: Id,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone)]
pub enum Id {
    SR_ID_NUMBER(i64),
    SR_ID_STRING(Strand),
    // unnesessary Box, but breaks header gen
    SR_ID_ARRAY(Array),
    SR_ID_OBJECT(Object),
    // Generate(Gen),
}

impl Transferrable<Thing> for sql::Thing {
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Thing> {
        let tb = self.tb.into_transferrable(controller)?;
        let id = match self.id {
            sql::Id::Number(i) => Id::SR_ID_NUMBER(i),
            sql::Id::String(s) => Id::SR_ID_STRING(s.into_transferrable(controller)?),
            sql::Id::Array(a) => Id::SR_ID_ARRAY(a.into_transferrable(controller)?),
            sql::Id::Object(o) => Id::SR_ID_OBJECT(o.into_transferrable(controller)?),
            sql::Id::Generate(_) => todo!(),
            _ => todo!(),
        };

        Ok(Thing { tb, id })
    }

    fn from_transferrable(value: Thing, controller: &mut dyn MemoryController) -> Result<Self> {
        let tb = String::from_transferrable(value.tb, controller)?;
        let id = match value.id {
            Id::SR_ID_NUMBER(x) => sql::Id::Number(x),
            Id::SR_ID_STRING(x) => sql::Id::String(String::from_transferrable(x, controller)?),
            Id::SR_ID_ARRAY(x) => sql::Id::Array(sql::Array::from_transferrable(x, controller)?),
            Id::SR_ID_OBJECT(x) => sql::Id::Object(sql::Object::from_transferrable(x, controller)?),
        };

        Ok(sql::Thing::from((tb, id)))
    }
}

impl From<Thing> for Value {
    fn from(value: Thing) -> Self {
        Value::SR_VALUE_THING(value)
    }
}
