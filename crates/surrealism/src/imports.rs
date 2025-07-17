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

// Declares external C functions for interacting with the underlying runtime.
//
// These functions are unsafe and represent FFI (Foreign Function Interface) calls
// to external implementations, likely in a WASM host environment. They handle
// operations like executing SQL queries, running functions, and machine learning
// tasks by passing pointers to data structures in linear memory.
//
// # Safety
// These declarations assume the external functions are correctly implemented and
// that pointers passed are valid. Incorrect usage may lead to undefined behavior,
// such as memory corruption or crashes.
unsafe extern "C" {
    /// Executes a SQL query using pointers to the query string and variables.
    unsafe fn __sr_sql(sql_ptr: u32, vars_ptr: u32) -> i32;
    /// Runs a named function with optional version and arguments via pointers.
    unsafe fn __sr_run(fnc_ptr: u32, version_ptr: u32, vars_ptr: u32) -> i32;
}

/// Executes a SurrealDB SQL query without variables.
///
/// This is a convenience wrapper around `sql_with_vars` that passes an empty
/// vector for variables.
///
/// # Type Parameters
/// - `S`: A type that can be converted into a `String` (e.g., `String`, `&str`).
/// - `R`: A type that implements `Transferrable<Value>`, representing the expected
///   return type after deserialization from the raw `Value`.
///
/// # Parameters
/// - `sql`: The SQL query to execute.
///
/// # Returns
/// A `Result` containing the deserialized return value `R` on success, or an error.
///
/// # Errors
/// - If the SQL query is empty after trimming.
/// - If data transfer or reception fails.
/// - If the underlying FFI call or deserialization encounters an issue.
pub fn sql<S, R>(sql: S) -> Result<R>
where
    S: Into<String>,
    R: Transferrable<Value>,
{
    sql_with_vars(sql, Vec::<(String, sql::Value)>::new())
}

/// Executes a SurrealDB SQL query with optional variables.
///
/// This function prepares the SQL query and variables, transfers them to the
/// runtime via FFI, executes the query, and deserializes the result.
///
/// # Type Parameters
/// - `S`: A type that can be converted into a `String` (e.g., `String`, `&str`).
/// - `V`: An iterator yielding pairs of `(String, sql::Value)` for query variables.
/// - `R`: A type that implements `Transferrable<Value>`, representing the expected
///   return type after deserialization from the raw `Value`.
///
/// # Parameters
/// - `sql`: The SQL query to execute.
/// - `vars`: An iterator of key-value pairs for query variables.
///
/// # Returns
/// A `Result` containing the deserialized return value `R` on success, or an error.
///
/// # Errors
/// - If the SQL query is empty after trimming.
/// - If converting or transferring data fails.
/// - If the FFI call or result reception encounters an issue.
/// - If deserializing the result into `R` fails.
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

/// Runs a named function in the SurrealDB runtime with optional version and arguments.
///
/// This function prepares the function name, version, and arguments, transfers them
/// via FFI, executes the function, and deserializes the result.
///
/// # Type Parameters
/// - `F`: A type that can be converted into a `String` (e.g., function name).
/// - `A`: A type that implements `Args`, providing arguments for the function.
/// - `R`: A type that implements `Transferrable<Value>`, representing the expected
///   return type after deserialization from the raw `Value`.
///
/// # Parameters
/// - `fnc`: The name of the function to run.
/// - `version`: An optional version string for the function.
/// - `args`: Arguments to pass to the function.
///
/// # Returns
/// A `Result` containing the deserialized return value `R` on success, or an error.
///
/// # Errors
/// - If transferring data fails.
/// - If the FFI call or result reception encounters an issue.
/// - If deserializing the result into `R` fails.
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

/// Module containing key-value store operations.
///
/// This module provides utilities for interacting with a key-value store in a
/// WASM-compatible environment using FFI calls. It supports basic operations
/// like get, set, delete, and exists, as well as batch operations and range-based
/// queries for efficient data management.
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

    // Declares external C functions for key-value store operations.
    //
    // These functions are unsafe FFI calls to external implementations for
    // basic KV operations, batch operations, and range-based queries.
    //
    // # Safety
    // Assumes valid pointers and correct external implementation.
    unsafe extern "C" {
        /// Retrieves a value from the key-value store using a key pointer.
        unsafe fn __sr_kv_get(key_ptr: u32) -> i32;
        /// Sets a value in the key-value store using key and value pointers.
        unsafe fn __sr_kv_set(key_ptr: u32, value_ptr: u32) -> i32;
        /// Deletes a key-value pair from the store using a key pointer.
        unsafe fn __sr_kv_del(key_ptr: u32) -> i32;
        /// Checks if a key exists in the store using a key pointer.
        unsafe fn __sr_kv_exists(key_ptr: u32) -> i32;

        /// Deletes all key-value pairs within a specified range.
        unsafe fn __sr_kv_del_rng(range_ptr: u32) -> i32;

        /// Retrieves multiple values from the store using an array of key pointers.
        unsafe fn __sr_kv_get_batch(keys_ptr: u32) -> i32;
        /// Sets multiple key-value pairs in the store using an array of entry pointers.
        unsafe fn __sr_kv_set_batch(entries_ptr: u32) -> i32;
        /// Deletes multiple key-value pairs from the store using an array of key pointers.
        unsafe fn __sr_kv_del_batch(keys_ptr: u32) -> i32;

        /// Retrieves all keys within a specified range.
        unsafe fn __sr_kv_keys(range_ptr: u32) -> i32;
        /// Retrieves all values within a specified range.
        unsafe fn __sr_kv_values(range_ptr: u32) -> i32;
        /// Retrieves all key-value pairs within a specified range.
        unsafe fn __sr_kv_entries(range_ptr: u32) -> i32;
        /// Counts the number of key-value pairs within a specified range.
        unsafe fn __sr_kv_count(range_ptr: u32) -> i32;
    }

    /// Retrieves a value from the key-value store by key.
    ///
    /// This function transfers the key to the runtime via FFI, retrieves the
    /// associated value, and deserializes it into the specified type.
    ///
    /// # Type Parameters
    /// - `K`: A type that can be converted into a `String` (e.g., the key).
    /// - `R`: A type that implements `Transferrable<Value>`, representing the expected
    ///   return type after deserialization from the raw `Value`.
    ///
    /// # Parameters
    /// - `key`: The key to look up in the store.
    ///
    /// # Returns
    /// A `Result` containing `Some(R)` if the key exists, `None` if it doesn't,
    /// or an error if the operation fails.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
    /// - If deserializing the result into `R` fails.
    pub fn get<K: Into<String>, R: Transferrable>(key: K) -> Result<Option<R>> {
        let mut controller = Controller {};
        let key = Transferrable::<Strand>::into_transferrable(key.into(), &mut controller)?
            .transfer(&mut controller)?;
        let result = unsafe { __sr_kv_get(key.ptr()) };
        let result = CResult::<COption<Value>>::receive(result.try_into()?, &mut controller)?;
        Result::<Option<R>>::from_transferrable(result, &mut controller)?
    }

    /// Sets a value in the key-value store for the specified key.
    ///
    /// This function transfers both the key and value to the runtime via FFI
    /// and stores them in the key-value store.
    ///
    /// # Type Parameters
    /// - `K`: A type that can be converted into a `String` (e.g., the key).
    /// - `V`: A type that implements `Transferrable<Value>`, representing the value to store.
    ///
    /// # Parameters
    /// - `key`: The key under which to store the value.
    /// - `value`: The value to store.
    ///
    /// # Returns
    /// A `Result` containing `()` on success, or an error if the operation fails.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
    pub fn set<K: Into<String>, V: Transferrable>(key: K, value: V) -> Result<()> {
        let mut controller = Controller {};
        let key = Transferrable::<Strand>::into_transferrable(key.into(), &mut controller)?
            .transfer(&mut controller)?;
        let value: Value = value.into_transferrable(&mut controller)?;
        let value = value.transfer(&mut controller)?;
        let result = unsafe { __sr_kv_set(key.ptr(), value.ptr()) };
        CResult::<()>::receive(result.try_into()?, &mut controller)?.try_ok(&mut controller)
    }

    /// Deletes a key-value pair from the store by key.
    ///
    /// This function transfers the key to the runtime via FFI and removes
    /// the associated key-value pair from the store.
    ///
    /// # Type Parameters
    /// - `K`: A type that can be converted into a `String` (e.g., the key to delete).
    ///
    /// # Parameters
    /// - `key`: The key of the key-value pair to delete.
    ///
    /// # Returns
    /// A `Result` containing `()` on success, or an error if the operation fails.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
    pub fn del<K: Into<String>>(key: K) -> Result<()> {
        let mut controller = Controller {};
        let key = Transferrable::<Strand>::into_transferrable(key.into(), &mut controller)?
            .transfer(&mut controller)?;
        let result = unsafe { __sr_kv_del(key.ptr()) };
        CResult::<()>::receive(result.try_into()?, &mut controller)?;
        Ok(())
    }

    /// Checks if a key exists in the key-value store.
    ///
    /// This function transfers the key to the runtime via FFI and checks
    /// whether the key exists in the store without retrieving its value.
    ///
    /// # Type Parameters
    /// - `K`: A type that can be converted into a `String` (e.g., the key to check).
    ///
    /// # Parameters
    /// - `key`: The key to check for existence.
    ///
    /// # Returns
    /// A `Result` containing `true` if the key exists, `false` if it doesn't,
    /// or an error if the operation fails.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
    pub fn exists<K: Into<String>>(key: K) -> Result<bool> {
        let mut controller = Controller {};
        let key = Transferrable::<Strand>::into_transferrable(key.into(), &mut controller)?
            .transfer(&mut controller)?;
        let result = unsafe { __sr_kv_exists(key.ptr()) };
        CResult::<bool>::receive(result.try_into()?, &mut controller)?.try_ok(&mut controller)
    }

    /// Deletes all key-value pairs within a specified range.
    ///
    /// This function transfers the range bounds to the runtime via FFI and
    /// removes all key-value pairs whose keys fall within the specified range.
    ///
    /// # Type Parameters
    /// - `R`: A type that implements `RangeBounds<String>` for defining the key range.
    ///
    /// # Parameters
    /// - `range`: The range of keys to delete (e.g., `"a".."z"` or `.."prefix"`).
    ///
    /// # Returns
    /// A `Result` containing `()` on success, or an error if the operation fails.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
    pub fn del_rng<R: RangeBounds<String>>(range: R) -> Result<()> {
        let mut controller = Controller {};
        let range = CRange::<Strand>::from_range_bounds(range, &mut controller)?
            .transfer(&mut controller)?;
        let result = unsafe { __sr_kv_del_rng(range.ptr()) };
        CResult::<()>::receive(result.try_into()?, &mut controller)?;
        Ok(())
    }

    /// Retrieves multiple values from the key-value store in a single operation.
    ///
    /// This function transfers an array of keys to the runtime via FFI and
    /// retrieves all corresponding values, returning `None` for non-existent keys.
    ///
    /// # Type Parameters
    /// - `K`: A type that can be converted into a `String` (e.g., the keys).
    /// - `I`: An iterator yielding keys of type `K`.
    /// - `R`: A type that implements `Transferrable<Value>`, representing the expected
    ///   return type after deserialization from the raw `Value`.
    ///
    /// # Parameters
    /// - `keys`: An iterator of keys to look up in the store.
    ///
    /// # Returns
    /// A `Result` containing a `Vec<Option<R>>` where each element corresponds
    /// to the key at the same index, or an error if the operation fails.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
    /// - If deserializing any result into `R` fails.
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

    /// Sets multiple key-value pairs in the store in a single operation.
    ///
    /// This function transfers an array of key-value pairs to the runtime via FFI
    /// and stores them all atomically in the key-value store.
    ///
    /// # Type Parameters
    /// - `K`: A type that can be converted into a `String` (e.g., the keys).
    /// - `V`: A type that implements `Transferrable<Value>` and `Clone`, representing the values.
    /// - `I`: An iterator yielding key-value pairs of type `(K, V)`.
    ///
    /// # Parameters
    /// - `entries`: An iterator of key-value pairs to store.
    ///
    /// # Returns
    /// A `Result` containing `()` on success, or an error if the operation fails.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
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

    /// Deletes multiple key-value pairs from the store in a single operation.
    ///
    /// This function transfers an array of keys to the runtime via FFI and
    /// removes all corresponding key-value pairs from the store.
    ///
    /// # Type Parameters
    /// - `K`: A type that can be converted into a `String` (e.g., the keys to delete).
    /// - `I`: An iterator yielding keys of type `K`.
    ///
    /// # Parameters
    /// - `keys`: An iterator of keys to delete from the store.
    ///
    /// # Returns
    /// A `Result` containing `()` on success, or an error if the operation fails.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
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

    /// Retrieves all keys within a specified range.
    ///
    /// This function transfers the range bounds to the runtime via FFI and
    /// returns all keys that fall within the specified range.
    ///
    /// # Type Parameters
    /// - `R`: A type that implements `RangeBounds<String>` for defining the key range.
    ///
    /// # Parameters
    /// - `range`: The range of keys to retrieve (e.g., `"a".."z"` or `.."prefix"`).
    ///
    /// # Returns
    /// A `Result` containing a `Vec<String>` of all keys within the range,
    /// or an error if the operation fails.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
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

    /// Retrieves all values within a specified key range.
    ///
    /// This function transfers the range bounds to the runtime via FFI and
    /// returns all values whose keys fall within the specified range.
    ///
    /// # Type Parameters
    /// - `R`: A type that implements `RangeBounds<String>` for defining the key range.
    /// - `T`: A type that implements `Transferrable<Value>` and `Clone`, representing the expected
    ///   return type after deserialization from the raw `Value`.
    ///
    /// # Parameters
    /// - `range`: The range of keys whose values to retrieve (e.g., `"a".."z"` or `.."prefix"`).
    ///
    /// # Returns
    /// A `Result` containing a `Vec<T>` of all values within the range,
    /// or an error if the operation fails.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
    /// - If deserializing any result into `T` fails.
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

    /// Retrieves all key-value pairs within a specified key range.
    ///
    /// This function transfers the range bounds to the runtime via FFI and
    /// returns all key-value pairs whose keys fall within the specified range.
    ///
    /// # Type Parameters
    /// - `R`: A type that implements `RangeBounds<String>` for defining the key range.
    /// - `T`: A type that implements `Transferrable<Value>` and `Clone`, representing the expected
    ///   return type after deserialization from the raw `Value`.
    ///
    /// # Parameters
    /// - `range`: The range of keys whose entries to retrieve (e.g., `"a".."z"` or `.."prefix"`).
    ///
    /// # Returns
    /// A `Result` containing a `Vec<(String, T)>` of all key-value pairs within the range,
    /// or an error if the operation fails.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
    /// - If deserializing any result into `T` fails.
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

    /// Counts the number of key-value pairs within a specified key range.
    ///
    /// This function transfers the range bounds to the runtime via FFI and
    /// returns the count of all key-value pairs whose keys fall within the specified range.
    ///
    /// # Type Parameters
    /// - `R`: A type that implements `RangeBounds<String>` for defining the key range.
    ///
    /// # Parameters
    /// - `range`: The range of keys to count (e.g., `"a".."z"` or `.."prefix"`).
    ///
    /// # Returns
    /// A `Result` containing a `u64` representing the count of key-value pairs
    /// within the range, or an error if the operation fails.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
    pub fn count<R: RangeBounds<String>>(range: R) -> Result<u64> {
        let mut controller = Controller {};
        let range = CRange::<Strand>::from_range_bounds(range, &mut controller)?
            .transfer(&mut controller)?;
        let result = unsafe { __sr_kv_count(range.ptr()) };
        CResult::<u64>::receive(result.try_into()?, &mut controller)?.try_ok(&mut controller)
    }
}

/// Module containing machine learning-related functions.
///
/// This module provides utilities for invoking ML models and tokenizing input
/// in a WASM-compatible environment using FFI calls.
pub mod ml {
    use crate::Controller;
    use anyhow::Result;
    use surrealism_types::{
        array::TransferredArray,
        convert::{Transfer, Transferrable},
        utils::CResult,
    };
    use surrealism_types::{string::Strand, value::Value};

    // Declares external C functions for ML operations.
    //
    // These are unsafe FFI calls to external implementations for model invocation
    // and tokenization.
    //
    // # Safety
    // Assumes valid pointers and correct external implementation.
    unsafe extern "C" {
        /// Invokes a machine learning model with input, weight, and weight directory pointers.
        unsafe fn __sr_ml_invoke_model(
            model_ptr: u32,
            input_ptr: u32,
            weight_ptr: u32,
            weight_dir_ptr: u32,
        ) -> i32;
        /// Tokenizes input using a specified tokenizer via pointers.
        unsafe fn __sr_ml_tokenize(tokenizer_ptr: u32, input_ptr: u32) -> i32;
    }

    /// Invokes a machine learning model with the given input, weight, and weight directory.
    ///
    /// Prepares and transfers parameters via FFI, executes the model, and deserializes the result.
    ///
    /// # Type Parameters
    /// - `M`: A type that can be converted into a `String` (e.g., model name).
    /// - `I`: A type that implements `Transferrable<Value>` for input and weight directory.
    /// - `R`: A type that implements `Transferrable<Value>`, representing the expected
    ///   return type after deserialization from the raw `Value`.
    ///
    /// # Parameters
    /// - `model`: The name or identifier of the ML model.
    /// - `input`: The input data for the model.
    /// - `weight`: An integer weight parameter (e.g., for model selection or scaling).
    /// - `weight_dir`: The directory or path for model weights.
    ///
    /// # Returns
    /// A `Result` containing the deserialized return value `R` on success, or an error.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
    /// - If deserializing the result into `R` fails.
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

    /// Tokenizes input using a specified tokenizer.
    ///
    /// Prepares and transfers parameters via FFI, performs tokenization, and returns
    /// a vector of floating-point values (e.g., token embeddings or scores).
    ///
    /// # Type Parameters
    /// - `T`: A type that can be converted into a `String` (e.g., tokenizer name).
    /// - `I`: A type that implements `Transferrable<Value>` for the input.
    ///
    /// # Parameters
    /// - `tokenizer`: The name or identifier of the tokenizer.
    /// - `input`: The input data to tokenize.
    ///
    /// # Returns
    /// A `Result` containing a `Vec<f64>` of tokenization results on success, or an error.
    ///
    /// # Errors
    /// - If transferring data fails.
    /// - If the FFI call or result reception encounters an issue.
    /// - If deserializing the transferred array fails.
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
