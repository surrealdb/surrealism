use std::ffi::{CStr, CString, c_char};

use super::string::string_t;

pub trait CStringExt {
	fn to_raw_char_ptr(self) -> *mut c_char;
}

pub trait CStringExt2 {
	fn to_string_t(self) -> string_t;
}

impl<T> CStringExt2 for T
where
	T: CStringExt,
{
	fn to_string_t(self) -> string_t {
		string_t(self.to_raw_char_ptr())
	}
}

impl CStringExt for String {
	fn to_raw_char_ptr(self) -> *mut c_char {
		let cstring = CString::new(self).unwrap();
		cstring.into_raw()
	}
}

impl CStringExt for &String {
	fn to_raw_char_ptr(self) -> *mut c_char {
		self.as_str().to_raw_char_ptr()
	}
}

impl CStringExt for &str {
	fn to_raw_char_ptr(self) -> *mut c_char {
		let cstring = CString::new(self).unwrap();
		cstring.into_raw()
	}
}

impl CStringExt for *const c_char {
	fn to_raw_char_ptr(self) -> *mut c_char {
		let cstr = unsafe { CStr::from_ptr(self) };
		let cstring = CString::from(cstr);
		cstring.into_raw()
	}
}

#[repr(C)]
#[derive(Clone, Debug)]
pub enum COption<T> {
    None,
    Some(T),
}

impl<T> From<Option<T>> for COption<T> {
	fn from(value: Option<T>) -> Self {
		if let Some(x) = value {
			COption::Some(x)
		} else {
			COption::None
		}
	}
}

impl<T> From<COption<T>> for Option<T> {
	fn from(value: COption<T>) -> Self {
		if let COption::Some(x) = value {
			Some(x)
		} else {
			None
		}
	}
}