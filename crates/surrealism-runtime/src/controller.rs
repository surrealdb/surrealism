use anyhow::Result;
use wasmtime::*;
use surrealdb::sql;
use surrealism_types::{args::Args, array::TransferredArray, controller::MemoryController, convert::{Transferrable, Transfer}, kind::Kind, value::Value};
use wasmtime_wasi::preview1::{self, WasiP1Ctx};
use wasmtime_wasi::p2::WasiCtxBuilder;

pub struct Controller {
    pub store: Store<WasiP1Ctx>,
    pub instance: Instance,
    pub memory: Memory,
}

impl Controller {
    pub fn from_file(file: &str) -> Self {
        let engine = Engine::default();
        let module = Module::from_file(&engine, file).unwrap();
    
        let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
        preview1::add_to_linker_sync(&mut linker, |t| t).unwrap();
        let pre: InstancePre<WasiP1Ctx> = linker.instantiate_pre(&module).unwrap();

        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_env()
            .build_p1();
    
        // Add any additional host functions here if needed (e.g., __sr_alloc)
    
        let mut store = Store::new(&engine, wasi_ctx);
        let instance = pre.instantiate(&mut store).unwrap();
        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("wasm module must export memory");

        Self {
            store,
            instance,
            memory,
        }
    }

    pub fn alloc(&mut self, len: u32, align: u32) -> Result<u32> {
        let alloc = self.instance.get_typed_func::<(u32, u32), u32>(&mut self.store, "__sr_alloc")?;
        alloc.call(&mut self.store, (len, align))
    }

    pub fn free(&mut self, ptr: u32, len: u32) -> Result<()> {
        let alloc = self.instance.get_typed_func::<(u32, u32), ()>(&mut self.store, "__sr_free")?;
        alloc.call(&mut self.store, (ptr, len))
    }

    pub fn invoke<A: Args>(&mut self, name: Option<String>, args: A) -> Result<sql::Value> {
        let name = format!("__sr_fnc__{}", name.unwrap_or_default());
        let args = args.transfer_args(self)?;
        let invoke = self.instance.get_typed_func::<(u32,), (u32,)>(&mut self.store, &name)?;
        let (ptr,) = invoke.call(&mut self.store, (args.ptr(),))?;
        let value = Value::receive(ptr.into(), self)?;
        sql::Value::from_transferrable(value, self)
    }

    pub fn args(&mut self, name: Option<String>) -> Result<Vec<sql::Kind>> {
        let name = format!("__sr_args__{}", name.unwrap_or_default());
        let args = self.instance.get_typed_func::<(), (u32,)>(&mut self.store, &name)?;
        let (ptr,) = args.call(&mut self.store, ())?;
        let array = TransferredArray::receive(ptr.into(), self)?;
        Vec::<Kind>::from_transferrable(array, self)?
            .into_iter()
            .map(|x| sql::Kind::from_transferrable(x, self))
            .collect()
    }

    pub fn returns(&mut self, name: Option<String>) -> Result<sql::Kind> {
        let name = format!("__sr_returns__{}", name.unwrap_or_default());
        let returns = self.instance.get_typed_func::<(), (u32,)>(&mut self.store, &name)?;
        let (ptr,) = returns.call(&mut self.store, ())?;
        let kind = Kind::receive(ptr.into(), self)?;
        sql::Kind::from_transferrable(kind, self)
    }
}

impl MemoryController for Controller {
    fn alloc(&mut self, len: u32, align: u32) -> Result<u32> {
        Controller::alloc(self, len, align)
    }

    fn free(&mut self, ptr: u32, len: u32) -> Result<()> {
        Controller::free(self, ptr, len)
    }

    fn mut_mem<'a>(&'a mut self, ptr: u32, len: u32) -> &'a mut [u8] {
        let mem = self.memory.data_mut(&mut self.store);
        &mut mem[(ptr as usize)..(ptr as usize) + (len as usize)]
    }
}