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