use anyhow::Result;

use crate::{
    controller::MemoryController,
    convert::{Transfer, Transferrable},
};

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

impl<T, X> Transferrable<COption<X>> for Option<T>
where
    T: Transferrable<X>,
    X: Transfer,
{
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<COption<X>> {
        Ok(self
            .map(|x| x.into_transferrable(controller))
            .transpose()?
            .into())
    }

    fn from_transferrable(
        value: COption<X>,
        controller: &mut dyn MemoryController,
    ) -> Result<Self> {
        let value: Option<X> = value.into();
        Ok(value
            .map(|x| T::from_transferrable(x, controller))
            .transpose()?)
    }
}
