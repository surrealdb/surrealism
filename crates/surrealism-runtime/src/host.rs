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
                    let result = host_try_or_return!("Host function error", (|| -> Result<$ret> $body)());
                    host_try_or_return!("Transfer error", <$ret>::transfer(result, &mut $controller)).ptr() as i32
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

    fn ml_invoke_model(&self, model: String, input: sql::Value, weight: i64) -> Result<sql::Value>;
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
    register_host_function!(linker, "__sr_sql", |controller: HostController,
                                                 sql: Strand,
                                                 vars: Object|
     -> Result<Value> {
        let sql = String::from_transferrable(sql, &mut controller)?;
        let vars = sql::Object::from_transferrable(vars, &mut controller)?;
        controller
            .host()
            .sql(sql, vars)?
            .into_transferrable(&mut controller)
    });

    register_host_function!(linker, "__sr_run", |controller: HostController,
                                                 fnc: Strand,
                                                 version: COption<Strand>,
                                                 args: TransferredArray<Value>|
     -> Result<Value> {
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

    register_host_function!(
        linker,
        "__sr_ml_invoke_model",
        |controller: HostController, model: Strand, input: Value, weight: i64| -> Result<Value> {
            let model = String::from_transferrable(model, &mut controller)?;
            let input = sql::Value::from_transferrable(input, &mut controller)?;
            controller
                .host()
                .ml_invoke_model(model, input, weight)?
                .into_transferrable(&mut controller)
        }
    );

    register_host_function!(linker, "__sr_ml_tokenize", |controller: HostController,
                                                         model: Strand,
                                                         input: Value|
     -> Result<
        TransferredArray<f64>
    > {
        let model = String::from_transferrable(model, &mut controller)?;
        let input = sql::Value::from_transferrable(input, &mut controller)?;
        controller
            .host()
            .ml_tokenize(model, input)?
            .into_transferrable(&mut controller)
    });

    // Custom stdout handler
    linker
        .func_wrap(
            "env",
            "__sr_stdout",
            |caller: Caller<StoreData>, ptr: u32, len: u32| -> i32 {
                let mut controller = HostController::from(caller);
                let output = host_try_or_return!("Failed to read stdout", {
                    let memory = controller.mut_mem(ptr, len);
                    String::from_utf8(memory.to_vec())
                });
                
                if let Err(e) = controller.host().stdout(&output) {
                    eprintln!("Failed to handle stdout: {}", e);
                    return -1;
                }
                0
            }
        )
        .prefix_err(|| "failed to register stdout function")?;

    // Custom stderr handler
    linker
        .func_wrap(
            "env",
            "__sr_stderr",
            |caller: Caller<StoreData>, ptr: u32, len: u32| -> i32 {
                let mut controller = HostController::from(caller);
                let output = host_try_or_return!("Failed to read stderr", {
                    let memory = controller.mut_mem(ptr, len);
                    String::from_utf8(memory.to_vec())
                });
                
                if let Err(e) = controller.host().stderr(&output) {
                    eprintln!("Failed to handle stderr: {}", e);
                    return -1;
                }
                0
            }
        )
        .prefix_err(|| "failed to register stderr function")?;

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
