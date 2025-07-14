use anyhow::Result;
use std::fmt::Debug;
use std::marker::PhantomData;
use surrealdb::sql;
use surrealism_types::args::Args;
use surrealism_types::array::TransferredArray;
use surrealism_types::controller::MemoryController;
use surrealism_types::convert::{Transfer, Transferrable, Transferred};
use surrealism_types::kind::{Kind, KindOf};
use surrealism_types::utils::CResult;
use surrealism_types::value::Value;

pub struct SurrealismFunction<A, R, F>
where
    A: 'static + Send + Sync + Args + Debug,
    R: 'static + Send + Sync + Transferrable<CResult<Value>> + Debug,
    F: 'static + Send + Sync + Fn(A) -> R,
{
    function: F,
    _phantom: PhantomData<(A, R)>,
}

impl<A, R, F> SurrealismFunction<A, R, F>
where
    A: 'static + Send + Sync + Args + Debug,
    R: 'static + Send + Sync + Transferrable<CResult<Value>> + KindOf + Debug,
    F: 'static + Send + Sync + Fn(A) -> R,
{
    pub fn from(function: F) -> Self {
        Self {
            function,
            _phantom: Default::default(),
        }
    }

    pub fn args(&self) -> Vec<sql::Kind> {
        A::kinds()
    }

    pub fn returns(&self) -> sql::Kind {
        R::kindof()
    }

    pub fn invoke(&self, args: A) -> Result<R> {
        Ok((self.function)(args))
    }

    pub fn args_raw(
        &self,
        controller: &mut dyn MemoryController,
    ) -> Result<Transferred<TransferredArray<Kind>>> {
        self.args()
            // Map them into transferrable types
            .into_iter()
            .map(|x| sql::Kind::into_transferrable(x, controller))
            .collect::<Result<Vec<Kind>>>()?
            // Transfer the value
            .into_transferrable(controller)?
            .transfer(controller)
    }

    pub fn returns_raw(&self, controller: &mut dyn MemoryController) -> Result<Transferred<Kind>> {
        self.returns()
            .into_transferrable(controller)?
            .transfer(controller)
    }

    pub fn invoke_raw(
        &self,
        controller: &mut dyn MemoryController,
        args: Transferred<TransferredArray<Value>>,
    ) -> Result<Transferred<CResult<Value>>> {
        let args = A::accept_args(args, controller)?;
        self.invoke(args)?
            .into_transferrable(controller)?
            .transfer(controller)
    }
}
