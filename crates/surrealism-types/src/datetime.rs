use chrono::Utc;
use surrealdb::sql;
use crate::err::Error;

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Datetime {
	secs: i64,
	nanos: u32,
}

impl From<chrono::DateTime<Utc>> for Datetime {
	fn from(value: chrono::DateTime<Utc>) -> Self {
		Self {
			secs: value.timestamp(),
			nanos: value.timestamp_subsec_nanos(),
		}
	}
}

impl From<surrealdb::sql::Datetime> for Datetime {
	fn from(value: surrealdb::sql::Datetime) -> Self {
		value.0.into()
	}
}

impl TryFrom<Datetime> for sql::Datetime {
	type Error = Error;
	fn try_from(value: Datetime) -> Result<Self, Error> {
		if let Some(dt) = chrono::DateTime::<Utc>::from_timestamp(value.secs, value.nanos) {
			Ok(dt.into())
		} else {
			Err(Error::InvalidDatetime)
		}
	}
}