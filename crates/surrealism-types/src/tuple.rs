use crate::kind::KindOf;
use surrealdb::sql;
use surrealdb::sql::Kind;
use crate::value::Value;
use crate::array::Array;
use crate::convert::Transferrable;
use crate::controller::MemoryController;
use anyhow::Result;
use crate::err::Error;

macro_rules! impl_args {
    ($($len:literal => ($($name:ident),+)),+ $(,)?) => {
        $(
            impl<$($name),+> KindOf for ($($name,)+)
            where
                $($name: KindOf),+
            {
                fn kindof() -> sql::Kind {
                    sql::Kind::Literal(sql::Literal::Array(vec![
                        $($name::kindof(),)+
                    ]))
                }
            }

            impl<$($name),+> Transferrable for ($($name,)+)
            where
                $($name: Transferrable + KindOf),+
            {
                fn into_transferrable(self, controller: &mut dyn MemoryController) -> Result<Value> {
                    #[allow(non_snake_case)]
                    let ($($name,)+) = self;
                    let vals = vec![
                        $($name.into_transferrable(controller)?),+
                    ];
                    Ok(Value::SR_VALUE_ARRAY(Array(vals.into_transferrable(controller)?)))
                }
                
                fn from_transferrable(value: Value, controller: &mut dyn MemoryController) -> Result<Self> {
                    if let Value::SR_VALUE_ARRAY(x) = value {
                        let mut arr = Vec::<Value>::from_transferrable(x.0, controller)?;
                        if arr.len() != $len {
                            return Err(Error::UnexpectedType(
                                Kind::Array(Box::new(Kind::Any), Some(arr.len() as u64)), 
                                Self::kindof()
                            ).into())
                        }

                        $(
                            #[allow(non_snake_case)]
                            let $name = $name::from_transferrable(arr.remove(0), controller)?;
                        )+

                        Ok(($($name,)+))
                    } else {
                        Err(Error::UnexpectedType(value.kindof(), Kind::Array(Box::new(Kind::Any), None)).into())
                    }
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