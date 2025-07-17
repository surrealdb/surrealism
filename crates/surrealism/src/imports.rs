use anyhow::Result;
use surrealdb::sql;
use surrealism_types::{
    args::Args,
    convert::{Transfer, Transferrable},
    object::KeyValuePair,
    string::Strand,
    utils::{COption, CResult},
    value::Value,
};

use crate::Controller;

unsafe extern "C" {
    unsafe fn __sr_sql(sql_ptr: u32, vars_ptr: u32) -> i32;
    unsafe fn __sr_run(fnc_ptr: u32, version_ptr: u32, vars_ptr: u32) -> i32;
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
    let result = CResult::<Value>::receive(result.try_into()?, &mut controller)?;
    Result::<R>::from_transferrable(result, &mut controller)?
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
    let result = CResult::<Value>::receive(result.try_into()?, &mut controller)?;
    Result::<R>::from_transferrable(result, &mut controller)?
}

pub mod kv {
    use crate::Controller;
    use anyhow::Result;
    use std::ops::RangeBounds;
    use surrealism_types::{
        array::TransferredArray,
        convert::{Transfer, Transferrable, TransferrableArray, Transferred},
        object::KeyValuePair,
        string::Strand,
        utils::{COption, CRange, CResult},
        value::Value,
    };

    unsafe extern "C" {
        unsafe fn __sr_kv_get(key_ptr: u32) -> i32;
        unsafe fn __sr_kv_set(key_ptr: u32, value_ptr: u32) -> i32;
        unsafe fn __sr_kv_del(key_ptr: u32) -> i32;
        unsafe fn __sr_kv_exists(key_ptr: u32) -> i32;

        unsafe fn __sr_kv_del_rng(range_ptr: u32) -> i32;

        unsafe fn __sr_kv_get_batch(keys_ptr: u32) -> i32;
        unsafe fn __sr_kv_set_batch(entries_ptr: u32) -> i32;
        unsafe fn __sr_kv_del_batch(keys_ptr: u32) -> i32;

        unsafe fn __sr_kv_keys(range_ptr: u32) -> i32;
        unsafe fn __sr_kv_values(range_ptr: u32) -> i32;
        unsafe fn __sr_kv_entries(range_ptr: u32) -> i32;
        unsafe fn __sr_kv_count(range_ptr: u32) -> i32;
    }

    pub fn get<K: Into<String>, R: Transferrable>(key: K) -> Result<Option<R>> {
        let mut controller = Controller {};
        let key = Transferrable::<Strand>::into_transferrable(key.into(), &mut controller)?
            .transfer(&mut controller)?;
        let result = unsafe { __sr_kv_get(key.ptr()) };
        let result = CResult::<COption<Value>>::receive(result.try_into()?, &mut controller)?;
        Result::<Option<R>>::from_transferrable(result, &mut controller)?
    }

    pub fn set<K: Into<String>, V: Transferrable>(key: K, value: V) -> Result<()> {
        let mut controller = Controller {};
        let key = Transferrable::<Strand>::into_transferrable(key.into(), &mut controller)?
            .transfer(&mut controller)?;
        let value: Value = value.into_transferrable(&mut controller)?;
        let value = value.transfer(&mut controller)?;
        let result = unsafe { __sr_kv_set(key.ptr(), value.ptr()) };
        CResult::<()>::receive(result.try_into()?, &mut controller)?.try_ok(&mut controller)
    }

    pub fn del<K: Into<String>>(key: K) -> Result<()> {
        let mut controller = Controller {};
        let key = Transferrable::<Strand>::into_transferrable(key.into(), &mut controller)?
            .transfer(&mut controller)?;
        let result = unsafe { __sr_kv_del(key.ptr()) };
        CResult::<()>::receive(result.try_into()?, &mut controller)?;
        Ok(())
    }

    pub fn exists<K: Into<String>>(key: K) -> Result<bool> {
        let mut controller = Controller {};
        let key = Transferrable::<Strand>::into_transferrable(key.into(), &mut controller)?
            .transfer(&mut controller)?;
        let result = unsafe { __sr_kv_exists(key.ptr()) };
        CResult::<bool>::receive(result.try_into()?, &mut controller)?.try_ok(&mut controller)
    }

    pub fn del_rng<R: RangeBounds<String>>(range: R) -> Result<()> {
        let mut controller = Controller {};
        let range = CRange::<Strand>::from_range_bounds(range, &mut controller)?
            .transfer(&mut controller)?;
        let result = unsafe { __sr_kv_del_rng(range.ptr()) };
        CResult::<()>::receive(result.try_into()?, &mut controller)?;
        Ok(())
    }

    pub fn get_batch<K, I, R>(keys: I) -> Result<Vec<Option<R>>>
    where
        I: IntoIterator<Item = K>,
        K: Into<String>,
        R: Transferrable,
    {
        let mut controller = Controller {};
        let keys: Transferred<TransferredArray<Strand>> = keys
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<String>>()
            .transfer_array(&mut controller)?
            .transfer(&mut controller)?;

        let result = unsafe { __sr_kv_get_batch(keys.ptr()) };
        let result = CResult::<TransferredArray<COption<Value>>>::receive(
            result.try_into()?,
            &mut controller,
        )?
        .try_ok(&mut controller)?;
        Vec::<Option<R>>::from_transferred_array(result, &mut controller)
    }

    pub fn set_batch<K, V, I>(entries: I) -> Result<()>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Transferrable + Clone,
    {
        let mut controller = Controller {};
        let entries: Vec<(String, V)> = entries
            .into_iter()
            .map(|(k, v)| (k.into(), v))
            .collect::<Vec<_>>();
        let entries = entries
            .transfer_array(&mut controller)?
            .transfer(&mut controller)?;

        let result = unsafe { __sr_kv_set_batch(entries.ptr()) };
        CResult::<()>::receive(result.try_into()?, &mut controller)?;
        Ok(())
    }

    pub fn del_batch<K, I>(keys: I) -> Result<()>
    where
        I: IntoIterator<Item = K>,
        K: Into<String>,
    {
        let mut controller = Controller {};
        let keys: Transferred<TransferredArray<Strand>> = keys
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<String>>()
            .transfer_array(&mut controller)?
            .transfer(&mut controller)?;

        let result = unsafe { __sr_kv_del_batch(keys.ptr()) };
        CResult::<()>::receive(result.try_into()?, &mut controller)?;
        Ok(())
    }

    pub fn keys<R: RangeBounds<String>>(range: R) -> Result<Vec<String>> {
        let mut controller = Controller {};
        let range = CRange::<Strand>::from_range_bounds(range, &mut controller)?
            .transfer(&mut controller)?;
        let result = unsafe { __sr_kv_keys(range.ptr()) };
        let result =
            CResult::<TransferredArray<Strand>>::receive(result.try_into()?, &mut controller)?
                .try_ok(&mut controller)?;
        Vec::<String>::from_transferred_array(result, &mut controller)
    }

    pub fn values<R: RangeBounds<String>, T: Transferrable + Clone>(range: R) -> Result<Vec<T>> {
        let mut controller = Controller {};
        let range = CRange::<Strand>::from_range_bounds(range, &mut controller)?
            .transfer(&mut controller)?;
        let result = unsafe { __sr_kv_values(range.ptr()) };
        let result =
            CResult::<TransferredArray<Value>>::receive(result.try_into()?, &mut controller)?
                .try_ok(&mut controller)?;
        Vec::<T>::from_transferred_array(result, &mut controller)
    }

    pub fn entries<R: RangeBounds<String>, T: Transferrable + Clone>(
        range: R,
    ) -> Result<Vec<(String, T)>> {
        let mut controller = Controller {};
        let range = CRange::<Strand>::from_range_bounds(range, &mut controller)?
            .transfer(&mut controller)?;
        let result = unsafe { __sr_kv_entries(range.ptr()) };
        let result = CResult::<TransferredArray<KeyValuePair<T>>>::receive(
            result.try_into()?,
            &mut controller,
        )?
        .try_ok(&mut controller)?;
        Vec::<(String, T)>::from_transferred_array(result, &mut controller)
    }

    pub fn count<R: RangeBounds<String>>(range: R) -> Result<u64> {
        let mut controller = Controller {};
        let range = CRange::<Strand>::from_range_bounds(range, &mut controller)?
            .transfer(&mut controller)?;
        let result = unsafe { __sr_kv_count(range.ptr()) };
        CResult::<u64>::receive(result.try_into()?, &mut controller)?.try_ok(&mut controller)
    }
}

pub mod ml {
    use crate::Controller;
    use anyhow::Result;
    use surrealism_types::{
        array::TransferredArray,
        convert::{Transfer, Transferrable},
        utils::CResult,
    };
    use surrealism_types::{string::Strand, value::Value};

    unsafe extern "C" {
        unsafe fn __sr_ml_invoke_model(
            model_ptr: u32,
            input_ptr: u32,
            weight_ptr: u32,
            weight_dir_ptr: u32,
        ) -> i32;
        unsafe fn __sr_ml_tokenize(tokenizer_ptr: u32, input_ptr: u32) -> i32;
    }

    pub fn invoke_model<M, D, I, R>(model: M, input: I, weight: i64, weight_dir: D) -> Result<R>
    where
        M: Into<String>,
        D: Into<String>,
        I: Transferrable<Value>,
        R: Transferrable<Value>,
    {
        let model = model.into();
        let weight_dir = weight_dir.into();
        let mut controller = Controller {};
        let model = Transferrable::<Strand>::into_transferrable(model, &mut controller)?
            .transfer(&mut controller)?;
        let input = input
            .into_transferrable(&mut controller)?
            .transfer(&mut controller)?;
        let weight = weight.transfer(&mut controller)?;
        let weight_dir = Transferrable::<Strand>::into_transferrable(weight_dir, &mut controller)?
            .transfer(&mut controller)?;

        let result = unsafe {
            __sr_ml_invoke_model(model.ptr(), input.ptr(), weight.ptr(), weight_dir.ptr())
        };
        let result = CResult::<Value>::receive(result.try_into()?, &mut controller)?;
        Result::<R>::from_transferrable(result, &mut controller)?
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
        let result =
            CResult::<TransferredArray<f64>>::receive(result.try_into()?, &mut controller)?;
        Result::<Vec<f64>>::from_transferrable(result, &mut controller)?
    }
}
