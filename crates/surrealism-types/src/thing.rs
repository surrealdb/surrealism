use std::{ffi::CString, fmt::Debug};
use surrealdb::sql;
use crate::controller::MemoryController;
use super::{array::Array, convert::{FromTransferrable, IntoTransferrable}, object::Object, string::string_t, utils::CStringExt2, value::Value};
use anyhow::Result;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Thing {
	table: string_t,
	id: Id,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone)]
pub enum Id {
	SR_ID_NUMBER(i64),
	SR_ID_STRING(string_t),
	// unnesessary Box, but breaks header gen
	SR_ID_ARRAY(Array),
	SR_ID_OBJECT(Object),
	// Generate(Gen),
}

impl IntoTransferrable<Thing> for sql::Thing {
	fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Thing> {
		let str_ptr = CString::new(self.tb).unwrap().into_raw();
		let id = match self.id {
			sql::Id::Number(i) => Id::SR_ID_NUMBER(i),
			sql::Id::String(s) => Id::SR_ID_STRING(s.to_string_t()),
			sql::Id::Array(a) => Id::SR_ID_ARRAY(a.into_transferrable(controller)?),
			sql::Id::Object(o) => Id::SR_ID_OBJECT(o.into_transferrable(controller)?),
			sql::Id::Generate(_) => todo!(),
			_ => todo!(),
		};

		Ok(Thing {
			table: string_t(str_ptr),
			id,
		})
	}
}

impl FromTransferrable<Thing> for sql::Thing {
	fn from_transferrable(value: Thing, controller: &mut dyn MemoryController) -> Result<Self> {
		let tb: String = value.table.into();
		let id = match value.id {
			Id::SR_ID_NUMBER(x) => sql::Id::Number(x),
			Id::SR_ID_STRING(x) => sql::Id::String(x.into()),
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