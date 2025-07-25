use anyhow::Result;
use surrealdb::expr;
use crate::arg::Arg;

pub trait Args: Sized {
    fn to_values(self) -> Vec<expr::Value>;
    fn from_values(values: Vec<expr::Value>) -> Result<Self>;
    fn kinds() -> Vec<expr::Kind>;
}

macro_rules! impl_args {
    ($($len:literal => ($($name:ident),+)),+ $(,)?) => {
        $(
            impl<$($name),+> Args for ($($name,)+)
            where
                $($name: Arg),+
            {
                fn to_values(self) -> Vec<expr::Value> {
                    #[allow(non_snake_case)]
                    let ($($name,)+) = self;
                    vec![
                        $($name.to_value(),)+
                    ]
                }
                
                fn from_values(values: Vec<expr::Value>) -> Result<Self> {
                    if values.len() != $len {
                        return Err(anyhow::anyhow!("Expected ({}), found other arguments", Self::kinds().iter().map(|k| k.to_string()).collect::<Vec<String>>().join(", ")));
                    }

                    let mut values = values;
                    
                    $(#[allow(non_snake_case)] let $name = values.remove(0);)+

                    Ok(($($name::from_value($name)?,)+))
                }
                
                fn kinds() -> Vec<expr::Kind> {
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
    fn to_values(self) -> Vec<expr::Value> {
        Vec::new()
    }

    fn from_values(values: Vec<expr::Value>) -> Result<Self> {
        if !values.is_empty() {
            return Err(anyhow::anyhow!("Expected ({}), found other arguments", Self::kinds().iter().map(|k| k.to_string()).collect::<Vec<String>>().join(", ")));
        }

        Ok(())
    }

    fn kinds() -> Vec<expr::Kind> {
        Vec::new()
    }
}

impl<T> Args for Vec<T>
where
    T: Arg,
{
    fn to_values(self) -> Vec<expr::Value> {
        self.into_iter().map(|x| x.to_value()).collect()
    }

    fn from_values(values: Vec<expr::Value>) -> Result<Self> {
        Ok(values.into_iter().map(|x| T::from_value(x)).collect::<Result<Vec<T>>>()?.into())
    }

    // This implementation is only used to dynamically transfer arguments, not to annotate them
    fn kinds() -> Vec<expr::Kind> {
        vec![T::kindof()]
    }
}
