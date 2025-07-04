use anyhow::Result;
use surrealdb::sql;
use surrealism_types::{array::TransferredArray, convert::{Transfer, Transferrable}, object::KeyValuePair, string::Strand, utils::COption, value::Value};

use crate::Controller;

unsafe extern "C" {
    unsafe fn __sr_sql(sql_ptr: u32, vars_ptr: u32) -> u32;
    unsafe fn __sr_run(fnc_ptr: u32, version_ptr: u32, vars_ptr: u32) -> u32;
}

pub fn sql<V, R>(
    sql: String,
    vars: Option<V>,
) -> Result<R> 
where 
    V: IntoIterator<Item = (String, sql::Value)>,
    R: Transferrable<Value>,
{
    if sql.trim().is_empty() {
        anyhow::bail!("SQL query cannot be empty");
    }

    let mut controller = Controller {};
    let sql = Transferrable::<Strand>::into_transferrable(sql, &mut controller)?.transfer(&mut controller)?;
    
    let vars: COption<TransferredArray<KeyValuePair>> = vars
        .map(|x| -> Result<TransferredArray<KeyValuePair>> {
            x.into_iter()
                .map(|x| x.into_transferrable(&mut controller))
                .collect::<Result<Vec<KeyValuePair>>>()?
                .into_transferrable(&mut controller)
        })
        .transpose()?
        .into();
    let vars = vars.transfer(&mut controller)?;

    let result = unsafe { __sr_sql(sql.ptr(), vars.ptr()) };
    let result = Value::receive(result.into(), &mut controller)?;
    R::from_transferrable(result, &mut controller)
}

pub fn run<V, R>(
    fnc: String,
    version: Option<String>,
    vars: Option<V>,
) -> Result<R> 
where 
    V: IntoIterator<Item = (String, sql::Value)>,
    R: Transferrable<Value>,
{
    let mut controller = Controller {};
    let fnc = Transferrable::<Strand>::into_transferrable(fnc, &mut controller)?.transfer(&mut controller)?;
    let version: COption<Strand> = version
        .map(|x| -> Result<Strand> {
            Transferrable::<Strand>::into_transferrable(x, &mut controller)
        })
        .transpose()?
        .into();
    let version = version.transfer(&mut controller)?;

    
    let vars: COption<TransferredArray<KeyValuePair>> = vars
        .map(|x| -> Result<TransferredArray<KeyValuePair>> {
            x.into_iter()
                .map(|x| x.into_transferrable(&mut controller))
                .collect::<Result<Vec<KeyValuePair>>>()?
                .into_transferrable(&mut controller)
        })
        .transpose()?
        .into();
    let vars = vars.transfer(&mut controller)?;

    let result = unsafe { __sr_run(fnc.ptr(), version.ptr(), vars.ptr()) };
    let result = Value::receive(result.into(), &mut controller)?;
    R::from_transferrable(result, &mut controller)
}

pub mod ml {
    use surrealism_types::{array::TransferredArray, convert::{Transfer, Transferrable}};
    use crate::Controller;
    use surrealism_types::{value::Value, string::Strand};
    use anyhow::Result;

    unsafe extern "C" {
        unsafe fn __sr_ml_invoke_model(model_ptr: u32, input_ptr: u32, weight_ptr: u32) -> u32;
        unsafe fn __sr_ml_tokenize(tokenizer_ptr: u32, input_ptr: u32) -> u32;
    }

    pub fn invoke_model<I, R>(model: String, input: I, weight: i64) -> Result<R>
    where
        I: Transferrable<Value>,
        R: Transferrable<Value> 
    {
        let mut controller = Controller {};
        let model = Transferrable::<Strand>::into_transferrable(model, &mut controller)?.transfer(&mut controller)?;
        let input = input.into_transferrable(&mut controller)?.transfer(&mut controller)?;
        let weight = weight.transfer(&mut controller)?;

        let result = unsafe { __sr_ml_invoke_model(model.ptr(), input.ptr(), weight.ptr()) };
        let result = Value::receive(result.into(), &mut controller)?;
        R::from_transferrable(result, &mut controller)
    }

    pub fn tokenize<I>(tokenizer: String, input: I) -> Result<Vec<f64>>
    where
        I: Transferrable<Value>
    {
        let mut controller = Controller {};
        let tokenizer = Transferrable::<Strand>::into_transferrable(tokenizer, &mut controller)?.transfer(&mut controller)?;
        let input = input.into_transferrable(&mut controller)?.transfer(&mut controller)?;

        let result = unsafe { __sr_ml_tokenize(tokenizer.ptr(), input.ptr()) };
        let result = TransferredArray::<f64>::receive(result.into(), &mut controller)?;
        Vec::<f64>::from_transferrable(result, &mut controller)
    }
}