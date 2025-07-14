use crate::{
    controller::MemoryController,
    convert::{Transfer, Transferrable},
    string::Strand,
    value::Value,
};
use anyhow::Result;

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

#[repr(C)]
#[derive(Clone, Debug)]
pub enum CResult<T> {
    Ok(T),
    Err(Strand),
}

impl<T> From<Result<T, Strand>> for CResult<T> {
    fn from(value: Result<T, Strand>) -> Self {
        match value {
            Ok(x) => CResult::Ok(x),
            Err(e) => CResult::Err(e),
        }
    }
}

impl<T> From<CResult<T>> for Result<T, Strand> {
    fn from(value: CResult<T>) -> Self {
        match value {
            CResult::Ok(x) => Ok(x),
            CResult::Err(e) => Err(e),
        }
    }
}

impl<T, X> Transferrable<CResult<X>> for Result<T, String>
where
    T: Transferrable<X>,
    X: Transfer,
{
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<CResult<X>> {
        Ok(match self {
            Ok(x) => {
                let x = x.into_transferrable(controller)?;
                CResult::Ok(x)
            }
            Err(e) => {
                let e = e.to_string().into_transferrable(controller)?;
                CResult::Err(e)
            }
        })
    }

    fn from_transferrable(
        value: CResult<X>,
        controller: &mut dyn MemoryController,
    ) -> Result<Self> {
        Ok(match value {
            CResult::Ok(x) => {
                let x = T::from_transferrable(x, controller)?;
                Ok(x)
            }
            CResult::Err(e) => {
                let e: String = Transferrable::from_transferrable(e, controller)?;
                Err(e.into())
            }
        })
    }
}

impl<T, X> Transferrable<CResult<X>> for anyhow::Result<T>
where
    T: Transferrable<X>,
    X: Transfer,
{
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<CResult<X>> {
        Ok(match self {
            Ok(x) => {
                let x = x.into_transferrable(controller)?;
                CResult::Ok(x)
            }
            Err(e) => {
                let e = e.to_string().into_transferrable(controller)?;
                CResult::Err(e)
            }
        })
    }

    fn from_transferrable(
        value: CResult<X>,
        controller: &mut dyn MemoryController,
    ) -> Result<Self> {
        Ok(match value {
            CResult::Ok(x) => {
                let x = T::from_transferrable(x, controller)?;
                Ok(x)
            }
            CResult::Err(e) => {
                let e: String = Transferrable::from_transferrable(e, controller)?;
                Err(anyhow::Error::msg(e))
            }
        })
    }
}

impl<T> Transferrable<CResult<Value>> for T
where
    T: Transferrable<Value>,
{
    fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<CResult<Value>> {
        Ok(CResult::Ok(self.into_transferrable(controller)?))
    }

    fn from_transferrable(
        value: CResult<Value>,
        controller: &mut dyn MemoryController,
    ) -> Result<Self> {
        match value {
            CResult::Ok(x) => Ok(T::from_transferrable(x, controller)?),
            CResult::Err(e) => {
                anyhow::bail!(String::from_transferrable(e, controller)?)
            }
        }
    }
}
