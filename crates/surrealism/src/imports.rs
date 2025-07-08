use anyhow::Result;
use surrealdb::sql;
use surrealism_types::{
    args::Args,
    convert::{Transfer, Transferrable},
    object::KeyValuePair,
    string::Strand,
    utils::COption,
    value::Value,
};

use crate::Controller;

unsafe extern "C" {
    unsafe fn __sr_sql(sql_ptr: u32, vars_ptr: u32) -> u32;
    unsafe fn __sr_run(fnc_ptr: u32, version_ptr: u32, vars_ptr: u32) -> u32;
}

pub fn sql<S, R>(sql: S) -> Result<R>
where
    S: Into<String>,
    R: Transferrable<Value>,
{
    sql_with_vars(sql, Vec::<(String, sql::Value)>::new())
}

pub fn sql_with_vars<S, V, R>(sql: S, vars: V) -> Result<R>
where
    S: Into<String>,
    V: IntoIterator<Item = (String, sql::Value)>,
    R: Transferrable<Value>,
{
    let sql = sql.into();
    if sql.trim().is_empty() {
        anyhow::bail!("SQL query cannot be empty");
    }

    let mut controller = Controller {};
    let sql = Transferrable::<Strand>::into_transferrable(sql, &mut controller)?
        .transfer(&mut controller)?;

    let vars = vars
        .into_iter()
        .map(|x| x.into_transferrable(&mut controller))
        .collect::<Result<Vec<KeyValuePair>>>()?
        .into_transferrable(&mut controller)?
        .transfer(&mut controller)?;

    let result = unsafe { __sr_sql(sql.ptr(), vars.ptr()) };
    let result = Value::receive(result.into(), &mut controller)?;
    R::from_transferrable(result, &mut controller)
}

pub fn run<F, A, R>(fnc: F, version: Option<String>, args: A) -> Result<R>
where
    F: Into<String>,
    A: Args,
    R: Transferrable<Value>,
{
    let fnc = fnc.into();
    let mut controller = Controller {};
    let fnc = Transferrable::<Strand>::into_transferrable(fnc, &mut controller)?
        .transfer(&mut controller)?;

    let version = Transferrable::<COption<Strand>>::into_transferrable(version, &mut controller)?
        .transfer(&mut controller)?;

    let args = args.transfer_args(&mut controller)?;

    let result = unsafe { __sr_run(fnc.ptr(), version.ptr(), args.ptr()) };
    let result = Value::receive(result.into(), &mut controller)?;
    R::from_transferrable(result, &mut controller)
}

pub mod ml {
    use crate::Controller;
    use anyhow::Result;
    use surrealism_types::{
        array::TransferredArray,
        convert::{Transfer, Transferrable},
    };
    use surrealism_types::{string::Strand, value::Value};

    unsafe extern "C" {
        unsafe fn __sr_ml_invoke_model(model_ptr: u32, input_ptr: u32, weight_ptr: u32) -> u32;
        unsafe fn __sr_ml_tokenize(tokenizer_ptr: u32, input_ptr: u32) -> u32;
    }

    pub fn invoke_model<M, I, R>(model: M, input: I, weight: i64) -> Result<R>
    where
        M: Into<String>,
        I: Transferrable<Value>,
        R: Transferrable<Value>,
    {
        let model = model.into();
        let mut controller = Controller {};
        let model = Transferrable::<Strand>::into_transferrable(model, &mut controller)?
            .transfer(&mut controller)?;
        let input = input
            .into_transferrable(&mut controller)?
            .transfer(&mut controller)?;
        let weight = weight.transfer(&mut controller)?;

        let result = unsafe { __sr_ml_invoke_model(model.ptr(), input.ptr(), weight.ptr()) };
        let result = Value::receive(result.into(), &mut controller)?;
        R::from_transferrable(result, &mut controller)
    }

    pub fn tokenize<T, I>(tokenizer: T, input: I) -> Result<Vec<f64>>
    where
        T: Into<String>,
        I: Transferrable<Value>,
    {
        let tokenizer = tokenizer.into();
        let mut controller = Controller {};
        let tokenizer = Transferrable::<Strand>::into_transferrable(tokenizer, &mut controller)?
            .transfer(&mut controller)?;
        let input = input
            .into_transferrable(&mut controller)?
            .transfer(&mut controller)?;

        let result = unsafe { __sr_ml_tokenize(tokenizer.ptr(), input.ptr()) };
        let result = TransferredArray::<f64>::receive(result.into(), &mut controller)?;
        Vec::<f64>::from_transferrable(result, &mut controller)
    }
}
