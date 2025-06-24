use anyhow::Result;
use wasmtime::*;
use surrealdb::sql;
use surrealism_types::{args::Args, controller::MemoryController, convert::{FromTransferrable, Transfer, Transferred}, kind::Kind, utils::CStringExt2, value::Value};

pub struct Controller {
    pub store: Store<()>,
    pub instance: Instance,
    pub memory: Memory,
}

impl Controller {
    pub fn from_file(file: &str) -> Self {
        let engine = Engine::default();
        let module = Module::from_file(&engine, file).unwrap();
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[]).unwrap();
        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("wasm module must export memory");

        Self {
            store,
            instance,
            memory
        }
    }

    pub fn alloc(&mut self, len: u32) -> Result<u32> {
        let alloc = self.instance.get_typed_func::<(u32,), u32>(&mut self.store, "alloc")?;
        alloc.call(&mut self.store, (len,))
    }

    pub fn free(&mut self, ptr: u32, len: u32) -> Result<()> {
        let alloc = self.instance.get_typed_func::<(u32, u32), ()>(&mut self.store, "free")?;
        alloc.call(&mut self.store, (ptr, len))
    }

    pub fn invoke<A: Args>(&mut self, name: String, args: A) -> Result<sql::Value> {
        let name = name.to_string_t().transfer(self)?;
        let args = args.transfer_args(self)?;
        let invoke = self.instance.get_typed_func::<(u32, u32, u32, u32), (u32, u32)>(&mut self.store, "invoke")?;
        let (ptr, len) = invoke.call(&mut self.store, (name.ptr, name.len, args.ptr, args.len))?;
        let value = Value::receive(Transferred { ptr, len }, self)?;
        sql::Value::from_transferrable(value, self)
    }

    pub fn args(&mut self, name: String) -> Result<Vec<sql::Kind>> {
        let name = name.to_string_t().transfer(self)?;
        let args = self.instance.get_typed_func::<(u32, u32), (u32, u32)>(&mut self.store, "args")?;
        let (ptr, len) = args.call(&mut self.store, (name.ptr, name.len))?;
        Vec::<Kind>::receive(Transferred { ptr, len }, self)?
            .into_iter()
            .map(|x| sql::Kind::from_transferrable(x, self))
            .collect()
    }
}

impl MemoryController for Controller {
    fn alloc(&mut self, len: u32) -> Result<u32> {
        self.alloc(len)
    }

    fn free(&mut self, ptr: u32, len: u32) -> Result<()> {
        self.free(ptr, len)
    }

    fn mut_mem<'a>(&'a mut self, ptr: u32, len: u32) -> &'a mut [u8] {
        let mem = self.memory.data_mut(&mut self.store);
        &mut mem[(ptr as usize)..(ptr as usize) + (len as usize)]
    }
}

fn main() -> Result<()> {
    // Load the wasm binary
    let engine = Engine::default();
    let module = Module::from_file(&engine, "target/wasm32-unknown-unknown/release/your_wasm.wasm")?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    // === 1. Call `add(a, b)` ===
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let result = add.call(&mut store, (3, 4))?;
    println!("add(3, 4) = {}", result);

    // === 2. Read from memory ===
    // Access the memory export
    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("wasm module must export memory");

    // Get pointer + length
    let get_data_ptr = instance.get_typed_func::<(), i32>(&mut store, "get_data_ptr")?;
    let get_data_len = instance.get_typed_func::<(), i32>(&mut store, "get_data_len")?;
    let ptr = get_data_ptr.call(&mut store, ())? as usize;
    let len = get_data_len.call(&mut store, ())? as usize;

    // Read data from WASM memory
    let data = memory.data(&store)[ptr..ptr + len].to_vec();
    println!("WASM buffer: {:?}", data);

    Ok(())
}