use std::ops::{Bound, Deref, DerefMut};

use anyhow::Result;
use surrealdb::sql;
use surrealism_types::{
    array::TransferredArray,
    controller::MemoryController,
    convert::{Transfer, Transferrable, TransferrableArray},
    err::PrefixError,
    object::KeyValuePair,
    string::Strand,
    utils::{COption, CRange, CResult},
    value::{Object, Value},
};
use wasmtime::{Caller, Linker};

use crate::{controller::StoreData, kv::KVStore};

macro_rules! host_try_or_return {
    ($error:expr,$expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{}: {}", $error, e);
                return -1;
            }
        }
    };
}

/// Macro to register a host function with automatic argument conversion and error handling.
/// Returns -1 on error (logged to stderr), positive values are valid pointers.
#[macro_export]
macro_rules! register_host_function {
    // Sync version with dynamic arguments
    ($linker:expr, $name:expr, |$controller:ident : $controller_ty:ty, $($arg:ident : $arg_ty:ty),*| -> Result<$ret:ty> $body:tt) => {{
        $linker
            .func_wrap(
                "env",
                $name,
                |caller: Caller<StoreData>, $($arg: u32),*| -> i32 {
                    let mut $controller: $controller_ty = HostController::from(caller);

                    // Handle argument receiving errors gracefully
                    $(let $arg = host_try_or_return!("Failed to receive argument", <$arg_ty>::receive($arg.into(), &mut $controller));)*

                    // Execute the main function body and handle errors gracefully
                    let result = match (|| -> Result<$ret> $body)() {
                        Ok(x) => CResult::Ok(x),
                        Err(e) => CResult::Err(host_try_or_return!("Failed to transfer error", e.to_string().into_transferrable(&mut $controller))),
                    };

                    host_try_or_return!("Transfer error", CResult::<$ret>::transfer(result, &mut $controller)).ptr() as i32
                }
            )
            .prefix_err(|| "failed to register host function")?
    }};
}

pub trait Host: Send {
    fn sql(&self, query: String, vars: sql::Object) -> Result<sql::Value>;
    fn run(
        &self,
        fnc: String,
        version: Option<String>,
        args: Vec<sql::Value>,
    ) -> Result<sql::Value>;

    fn kv(&mut self) -> &mut dyn KVStore;

    fn ml_invoke_model(
        &self,
        model: String,
        input: sql::Value,
        weight: i64,
        weight_dir: String,
    ) -> Result<sql::Value>;
    fn ml_tokenize(&self, model: String, input: sql::Value) -> Result<Vec<f64>>;

    /// Handle stdout output from the WASM module
    ///
    /// This method is called whenever the WASM module writes to stdout (e.g., via println!).
    /// The default implementation prints to standard output.
    ///
    /// # Example
    /// ```rust
    /// use surrealism_runtime::host::Host;
    /// use std::sync::{Arc, Mutex};
    ///
    /// struct CapturingHost {
    ///     stdout: Arc<Mutex<String>>,
    /// }
    ///
    /// impl Host for CapturingHost {
    ///     // ... implement other required methods ...
    ///     
    ///     fn stdout(&self, output: &str) -> Result<()> {
    ///         // Capture stdout to our string
    ///         self.stdout.lock().unwrap().push_str(output);
    ///         Ok(())
    ///     }
    /// }
    /// ```
    fn stdout(&self, output: &str) -> Result<()> {
        // Default implementation: print to standard output
        print!("{}", output);
        Ok(())
    }

    /// Handle stderr output from the WASM module
    ///
    /// This method is called whenever the WASM module writes to stderr (e.g., via eprintln!).
    /// The default implementation prints to standard error.
    ///
    /// # Example
    /// ```rust
    /// use surrealism_runtime::host::Host;
    /// use std::sync::{Arc, Mutex};
    ///
    /// struct CapturingHost {
    ///     stderr: Arc<Mutex<String>>,
    /// }
    ///
    /// impl Host for CapturingHost {
    ///     // ... implement other required methods ...
    ///     
    ///     fn stderr(&self, output: &str) -> Result<()> {
    ///         // Capture stderr to our string
    ///         self.stderr.lock().unwrap().push_str(output);
    ///         Ok(())
    ///     }
    /// }
    /// ```
    fn stderr(&self, output: &str) -> Result<()> {
        // Default implementation: print to standard error
        eprint!("{}", output);
        Ok(())
    }
}

pub fn implement_host_functions(linker: &mut Linker<StoreData>) -> Result<()> {
    // SQL function
    #[rustfmt::skip]
    register_host_function!(linker, "__sr_sql", |controller: HostController, sql: Strand, vars: Object| -> Result<Value> {
        let sql = String::from_transferrable(sql, &mut controller)?;
        let vars = sql::Object::from_transferrable(vars, &mut controller)?;
        controller
            .host()
            .sql(sql, vars)?
            .into_transferrable(&mut controller)
    });

    // Run function
    #[rustfmt::skip]
    register_host_function!(linker, "__sr_run", |controller: HostController, fnc: Strand, version: COption<Strand>, args: TransferredArray<Value>| -> Result<Value> {
        let fnc = String::from_transferrable(fnc, &mut controller)?;
        let version = Option::<String>::from_transferrable(version, &mut controller)?;
        let args_vec = Vec::<Value>::from_transferrable(args, &mut controller)?;
        let args = args_vec
            .into_iter()
            .map(|x| sql::Value::from_transferrable(x, &mut controller))
            .collect::<Result<Vec<sql::Value>>>()?;
        controller
            .host()
            .run(fnc, version, args)?
            .into_transferrable(&mut controller)
    });

    // KV functions
    #[rustfmt::skip]
    register_host_function!(linker, "__sr_kv_get", |controller: HostController, key: Strand| -> Result<COption<Value>> {
        let key = String::from_transferrable(key, &mut controller)?;
        controller
            .host_mut()
            .kv()
            .get(key)?
            .into_transferrable(&mut controller)
    });

    #[rustfmt::skip]
    register_host_function!(linker, "__sr_kv_set", |controller: HostController, key: Strand, value: Value| -> Result<()> {
        let key = String::from_transferrable(key, &mut controller)?;
        let value = sql::Value::from_transferrable(value, &mut controller)?;
        controller.host_mut().kv().set(key, value)?;
        Ok(())
    });

    #[rustfmt::skip]
    register_host_function!(linker, "__sr_kv_del", |controller: HostController, key: Strand| -> Result<()> {
        let key = String::from_transferrable(key, &mut controller)?;
        controller.host_mut().kv().del(key)?;
        Ok(())
    });

    #[rustfmt::skip]
    register_host_function!(linker, "__sr_kv_exists", |controller: HostController, key: Strand| -> Result<bool> {
        let key = String::from_transferrable(key, &mut controller)?;
        controller.host_mut().kv().exists(key)
    });

    #[rustfmt::skip]
    register_host_function!(linker, "__sr_kv_del_rng", |controller: HostController, range: CRange<Strand>| -> Result<()> {
        let start = Bound::<String>::from_transferrable(range.start, &mut controller)?;
        let end = Bound::<String>::from_transferrable(range.end, &mut controller)?;
        controller.host_mut().kv().del_rng(start, end)?;
        Ok(())
    });

    #[rustfmt::skip]
    register_host_function!(linker, "__sr_kv_get_batch", |controller: HostController, keys: TransferredArray<Strand>| -> Result<TransferredArray<COption<Value>>> {
        let keys = Vec::<String>::from_transferred_array(keys, &mut controller)?;
        let values = controller.host_mut().kv().get_batch(keys)?;
        values.transfer_array(&mut controller)
    });

    #[rustfmt::skip]
    register_host_function!(linker, "__sr_kv_set_batch", |controller: HostController, entries: TransferredArray<KeyValuePair>| -> Result<()> {
        let entries = Vec::<(String, sql::Value)>::from_transferred_array(entries, &mut controller)?;
        controller.host_mut().kv().set_batch(entries)?;
        Ok(())
    });

    #[rustfmt::skip]
    register_host_function!(linker, "__sr_kv_del_batch", |controller: HostController, keys: TransferredArray<Strand>| -> Result<()> {
        let keys = Vec::<String>::from_transferred_array(keys, &mut controller)?;
        controller.host_mut().kv().del_batch(keys)?;
        Ok(())
    });

    #[rustfmt::skip]
    register_host_function!(linker, "__sr_kv_keys", |controller: HostController, range: CRange<Strand>| -> Result<TransferredArray<Strand>> {
        let start = Bound::<String>::from_transferrable(range.start, &mut controller)?;
        let end = Bound::<String>::from_transferrable(range.end, &mut controller)?;
        let keys = controller.host_mut().kv().keys(start, end)?;
        keys.transfer_array(&mut controller)
    });

    #[rustfmt::skip]
    register_host_function!(linker, "__sr_kv_values", |controller: HostController, range: CRange<Strand>| -> Result<TransferredArray<Value>> {
        let start = Bound::<String>::from_transferrable(range.start, &mut controller)?;
        let end = Bound::<String>::from_transferrable(range.end, &mut controller)?;
        let values = controller.host_mut().kv().values(start, end)?;
        values.transfer_array(&mut controller)
    });

    #[rustfmt::skip]
    register_host_function!(linker, "__sr_kv_entries", |controller: HostController, range: CRange<Strand>| -> Result<TransferredArray<KeyValuePair>> {
        let start = Bound::<String>::from_transferrable(range.start, &mut controller)?;
        let end = Bound::<String>::from_transferrable(range.end, &mut controller)?;
        let entries = controller.host_mut().kv().entries(start, end)?;
        entries.transfer_array(&mut controller)
    });

    #[rustfmt::skip]
    register_host_function!(linker, "__sr_kv_count", |controller: HostController, range: CRange<Strand>| -> Result<u64> {
        let start = Bound::<String>::from_transferrable(range.start, &mut controller)?;
        let end = Bound::<String>::from_transferrable(range.end, &mut controller)?;
        controller.host_mut().kv().count(start, end)
    });

    // ML invoke model function
    #[rustfmt::skip]
    register_host_function!(linker, "__sr_ml_invoke_model", |controller: HostController, model: Strand, input: Value, weight: i64, weight_dir: Strand| -> Result<Value> {
        let model = String::from_transferrable(model, &mut controller)?;
        let input = sql::Value::from_transferrable(input, &mut controller)?;
        let weight_dir = String::from_transferrable(weight_dir, &mut controller)?;
        controller
            .host()
            .ml_invoke_model(model, input, weight, weight_dir)?
            .into_transferrable(&mut controller)
    });

    // ML tokenize function
    #[rustfmt::skip]
    register_host_function!(linker, "__sr_ml_tokenize", |controller: HostController, model: Strand, input: Value| -> Result<TransferredArray<f64>> {
        let model = String::from_transferrable(model, &mut controller)?;
        let input = sql::Value::from_transferrable(input, &mut controller)?;
        controller
            .host()
            .ml_tokenize(model, input)?
            .into_transferrable(&mut controller)
    });

    // Custom stdout handler (WASI-compatible)
    // linker
    //     .func_wrap(
    //         "wasi_snapshot_preview1",
    //         "fd_write",
    //         |caller: Caller<StoreData>, fd: u32, iovs_ptr: u32, iovs_len: u32, nwritten_ptr: u32| -> u32 {
    //             // Only handle stdout (fd == 1) and stderr (fd == 2)
    //             let mut controller = HostController::from(caller);
    //             if fd != 1 && fd != 2 {
    //                 return 8; // __WASI_ERRNO_BADF
    //             }

    //             // Read the iovec array from guest memory
    //             let mut output = Vec::new();
    //             for i in 0..iovs_len {
    //                 let base = iovs_ptr + i * 8;
    //                 let mem = controller.mut_mem(base, 8);
    //                 let ptr = u32::from_le_bytes([mem[0], mem[1], mem[2], mem[3]]);
    //                 let len = u32::from_le_bytes([mem[4], mem[5], mem[6], mem[7]]);
    //                 let data = controller.mut_mem(ptr, len);
    //                 output.extend_from_slice(data);
    //             }

    //             let output_str = match String::from_utf8(output) {
    //                 Ok(s) => s,
    //                 Err(_) => return 21, // __WASI_ERRNO_ILSEQ
    //             };

    //             let result = if fd == 1 {
    //                 controller.host().stdout(&output_str)
    //             } else {
    //                 controller.host().stderr(&output_str)
    //             };

    //             if let Err(e) = result {
    //                 eprintln!("Failed to handle fd_write: {}", e);
    //                 return 1; // __WASI_ERRNO_ACC
    //             }

    //             // Write the number of bytes written back to guest memory
    //             let nwritten = output_str.len() as u32;
    //             let mem = controller.mut_mem(nwritten_ptr, 4);
    //             mem.copy_from_slice(&nwritten.to_le_bytes());

    //             0 // __WASI_ERRNO_SUCCESS
    //         }
    //     )
    //     .prefix_err(|| "failed to register WASI fd_write function")?;

    Ok(())
}

struct HostController<'a>(Caller<'a, StoreData>);

impl<'a> HostController<'a> {
    pub fn host(&self) -> &Box<dyn Host> {
        &self.0.data().host
    }

    pub fn host_mut(&mut self) -> &mut Box<dyn Host> {
        &mut self.0.data_mut().host
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
        let alloc_func = self
            .get_export("__sr_alloc")
            .ok_or_else(|| anyhow::anyhow!("Export __sr_alloc not found"))?
            .into_func()
            .ok_or_else(|| anyhow::anyhow!("Export __sr_alloc is not a function"))?;
        let result = alloc_func
            .typed::<(u32, u32), i32>(&mut self.0)?
            .call(&mut self.0, (len, align))?;
        if result == -1 {
            anyhow::bail!("Memory allocation failed");
        }
        Ok(result as u32)
    }

    fn free(&mut self, ptr: u32, len: u32) -> Result<()> {
        let free_func = self
            .get_export("__sr_free")
            .ok_or_else(|| anyhow::anyhow!("Export __sr_free not found"))?
            .into_func()
            .ok_or_else(|| anyhow::anyhow!("Export __sr_free is not a function"))?;
        let result = free_func
            .typed::<(u32, u32), i32>(&mut self.0)?
            .call(&mut self.0, (ptr, len))?;
        if result == -1 {
            anyhow::bail!("Memory deallocation failed");
        }
        Ok(())
    }

    fn mut_mem(&mut self, ptr: u32, len: u32) -> &mut [u8] {
        let memory = self
            .get_export("memory")
            .ok_or_else(|| anyhow::anyhow!("Export memory not found"))
            .unwrap()
            .into_memory()
            .ok_or_else(|| anyhow::anyhow!("Export memory is not a memory"))
            .unwrap();
        let mem = memory.data_mut(&mut self.0);
        if (ptr as usize) + (len as usize) > mem.len() {
            println!(
                "[ERROR] Out of bounds: ptr + len = {} > mem.len() = {}",
                (ptr as usize) + (len as usize),
                mem.len()
            );
        }
        &mut mem[(ptr as usize)..(ptr as usize) + (len as usize)]
    }
}
