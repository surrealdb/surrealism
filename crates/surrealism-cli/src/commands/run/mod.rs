use crate::{commands::SurrealismCommand, host::DemoHost};
use anyhow::Result;
use std::path::PathBuf;
use surrealdb::sql::Value;
use surrealism_runtime::package::SurrealismPackage;
use surrealism_types::err::PrefixError;

pub struct RunCommand {
    pub file: PathBuf,
    pub fnc: Option<String>,
    pub args: Vec<Value>,
}

impl SurrealismCommand for RunCommand {
    fn run(self) -> Result<()> {
        let package = SurrealismPackage::from_file(self.file)?;

        // Load the WASM module
        let host = DemoHost::boxed();
        let mut controller = surrealism_runtime::controller::Controller::new(package, host)
            .prefix_err(|| "Failed to load WASM module")?;

        // Invoke the function with the provided arguments
        let result = controller
            .invoke(self.fnc, self.args)
            .prefix_err(|| "Failed to invoke function")?;

        // Print the result with pretty display formatting
        println!("{result:#}");

        Ok(())
    }
}
