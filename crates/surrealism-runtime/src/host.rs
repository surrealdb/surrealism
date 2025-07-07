use std::ops::{Deref, DerefMut};

use anyhow::Result;
use surrealdb::sql;
use surrealism_types::{
    array::TransferredArray,
    controller::MemoryController,
    convert::{Transfer, Transferrable},
    err::PrefixError,
    string::Strand,
    utils::COption,
    value::{Object, Value},
};
use wasmtime::{Caller, Linker};

use crate::controller::StoreData;

pub trait Host: Send {
    fn sql(&self, query: String, vars: sql::Object) -> Result<sql::Value>;
    fn run(
        &self,
        fnc: String,
        version: Option<String>,
        args: Vec<sql::Value>,
    ) -> Result<sql::Value>;

    fn ml_invoke_model(&self, model: String, input: sql::Value, weight: i64) -> Result<sql::Value>;
    fn ml_tokenize(&self, model: String, input: sql::Value) -> Result<Vec<f64>>;
}

pub fn implement_host_functions(linker: &mut Linker<StoreData>) -> Result<()> {
    linker
        .func_wrap(
            "env",
            "__sr_sql",
            |caller: Caller<StoreData>, sql: u32, vars: u32| -> u32 {
                let mut controller = HostController::from(caller);

                let sql = String::from_transferrable(
                    Strand::receive(sql.into(), &mut controller).unwrap(),
                    &mut controller,
                )
                .unwrap();

                let vars = sql::Object::from_transferrable(
                    Object::receive(vars.into(), &mut controller).unwrap(),
                    &mut controller,
                )
                .unwrap();

                let result = controller
                    .host()
                    .sql(sql, vars)
                    .unwrap()
                    .into_transferrable(&mut controller)
                    .unwrap()
                    .transfer(&mut controller)
                    .unwrap();

                result.ptr()
            },
        )
        .prefix_err(|| "failed to register host function")?;

    linker
        .func_wrap(
            "env",
            "__sr_run",
            |caller: Caller<StoreData>, fnc: u32, version: u32, args: u32| -> u32 {
                let mut controller = HostController::from(caller);

                let fnc = String::from_transferrable(
                    Strand::receive(fnc.into(), &mut controller).unwrap(),
                    &mut controller,
                )
                .unwrap();

                let version = Option::<String>::from_transferrable(
                    COption::<Strand>::receive(version.into(), &mut controller).unwrap(),
                    &mut controller,
                )
                .unwrap();

                let args = Vec::<Value>::from_transferrable(
                    TransferredArray::<Value>::receive(args.into(), &mut controller).unwrap(),
                    &mut controller,
                )
                .unwrap()
                .into_iter()
                .map(|x| sql::Value::from_transferrable(x, &mut controller))
                .collect::<Result<Vec<sql::Value>>>()
                .unwrap();

                let result = controller
                    .host()
                    .run(fnc, version, args)
                    .unwrap()
                    .into_transferrable(&mut controller)
                    .unwrap()
                    .transfer(&mut controller)
                    .unwrap();

                result.ptr()
            },
        )
        .prefix_err(|| "failed to register host function")?;

    linker
        .func_wrap(
            "env",
            "__sr_ml_invoke_model",
            |caller: Caller<StoreData>, model: u32, input: u32, weight: u32| -> u32 {
                let mut controller = HostController::from(caller);

                let model = String::from_transferrable(
                    Strand::receive(model.into(), &mut controller).unwrap(),
                    &mut controller,
                )
                .unwrap();

                let input = sql::Value::from_transferrable(
                    Value::receive(input.into(), &mut controller).unwrap(),
                    &mut controller,
                )
                .unwrap();

                let weight = i64::receive(weight.into(), &mut controller).unwrap();

                let result = controller
                    .host()
                    .ml_invoke_model(model, input, weight)
                    .unwrap()
                    .into_transferrable(&mut controller)
                    .unwrap()
                    .transfer(&mut controller)
                    .unwrap();

                result.ptr()
            },
        )
        .prefix_err(|| "failed to register host function")?;

    linker
        .func_wrap(
            "env",
            "__sr_ml_tokenize",
            |caller: Caller<StoreData>, model: u32, input: u32| -> u32 {
                let mut controller = HostController::from(caller);

                let model = String::from_transferrable(
                    Strand::receive(model.into(), &mut controller).unwrap(),
                    &mut controller,
                )
                .unwrap();

                let input = sql::Value::from_transferrable(
                    Value::receive(input.into(), &mut controller).unwrap(),
                    &mut controller,
                )
                .unwrap();

                let result = Transferrable::<TransferredArray<f64>>::into_transferrable(
                    controller.host().ml_tokenize(model, input).unwrap(),
                    &mut controller,
                )
                .unwrap()
                .transfer(&mut controller)
                .unwrap();

                result.ptr()
            },
        )
        .prefix_err(|| "failed to register host function")?;
    Ok(())
}

struct HostController<'a>(Caller<'a, StoreData>);

impl<'a> HostController<'a> {
    pub fn host(&self) -> &Box<dyn Host> {
        &self.0.data().host
    }
}

impl<'a> From<Caller<'a, StoreData>> for HostController<'a> {
    fn from(caller: Caller<'a, StoreData>) -> Self {
        Self(caller)
    }
}

impl<'a> Deref for HostController<'a> {
    type Target = Caller<'a, StoreData>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for HostController<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> MemoryController for HostController<'a> {
    fn alloc(&mut self, len: u32, align: u32) -> Result<u32> {
        let alloc_func = self.get_export("__sr_alloc").unwrap().into_func().unwrap();
        let result = alloc_func
            .typed::<(u32, u32), u32>(&mut self.0)?
            .call(&mut self.0, (len, align));
        result
    }

    fn free(&mut self, ptr: u32, len: u32) -> Result<()> {
        let free_func = self.get_export("__sr_free").unwrap().into_func().unwrap();
        free_func
            .typed::<(u32, u32), ()>(&mut self.0)?
            .call(&mut self.0, (ptr, len))
    }

    fn mut_mem(&mut self, ptr: u32, len: u32) -> &mut [u8] {
        let memory = self.get_export("memory").unwrap().into_memory().unwrap();
        let mem = memory.data_mut(&mut self.0);
        &mut mem[(ptr as usize)..(ptr as usize) + (len as usize)]
    }
}
