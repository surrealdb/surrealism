use crate::{controller::MemoryController, err::Error, array::Array, convert::{FromTransferrable, IntoTransferrable, Transfer, Transferred}, value::Value};
use crate::kind::KindOf;
use surrealdb::sql;
use anyhow::Result;

pub trait Args {
    fn transfer_args(self, controller: &mut dyn MemoryController) -> Result<Transferred>;
    fn accept_args(transferred: Transferred, controller: &mut dyn MemoryController) -> Result<Self>
    where 
        Self: Sized;
    fn kinds() -> Vec<sql::Kind>;
}

macro_rules! impl_args {
    ($($len:literal => ($($name:ident),+)),+ $(,)?) => {
        $(
            impl<$($name),+> Args for ($($name,)+)
            where
                $($name: IntoTransferrable<Value> + FromTransferrable<Value> + KindOf),+
            {
                fn transfer_args(self, controller: &mut dyn MemoryController) -> Result<Transferred> {
                    #[allow(non_snake_case)]
                    let ($($name,)+) = self;
                    let vals = vec![
                        $($name.into_transferrable(controller)?),+
                    ];
                    Ok(Array(vals.into_transferrable(controller)?).transfer(controller)?)
                }

                fn accept_args(transferred: Transferred, controller: &mut dyn MemoryController) -> Result<Self> {
                    let mut arr = Vec::<Value>::from_transferrable(Array::receive(transferred, controller)?.0, controller)?;
                    if arr.len() != $len {
                        return Err(Error::InvalidArgs($len, arr.len()).into())
                    }

                    $(
                        #[allow(non_snake_case)]
                        let $name = $name::from_transferrable(arr.remove(0), controller)?;
                    )+

                    Ok(($($name,)+))
                }

                fn kinds() -> Vec<sql::Kind> {
                    vec![
                        $($name::kindof(),)+
                    ]
                }
            }
        )+
    };
}

impl_args! {
    1 => (A),
    2 => (A, B),
    3 => (A, B, C),
    4 => (A, B, C, D),
    5 => (A, B, C, D, E),
    6 => (A, B, C, D, E, F),
    7 => (A, B, C, D, E, F, G),
    8 => (A, B, C, D, E, F, G, H),
    9 => (A, B, C, D, E, F, G, H, I),
    10 => (A, B, C, D, E, F, G, H, I, J),
}

// Empty impl
impl Args for () {
    fn transfer_args(self, controller: &mut dyn MemoryController) -> Result<Transferred> {
        Ok(Array(Vec::<Value>::new().into_transferrable(controller)?).transfer(controller)?)
    }

    fn accept_args(transferred: Transferred, controller: &mut dyn MemoryController) -> Result<Self> {
        let arr = Vec::<Value>::from_transferrable(Array::receive(transferred, controller)?.0, controller)?;
        if !arr.is_empty() {
            return Err(Error::InvalidArgs(0, arr.len()).into())
        }

        Ok(())
    }

    fn kinds() -> Vec<sql::Kind> {
        Vec::new()
    }
}