use crate::commands::SurrealismCommand;
use anyhow::{Context, Result};
use std::path::PathBuf;
use surrealdb::sql::Value;
use surrealism_runtime::package::SurrealismPackage;

pub struct RunCommand {
    pub file: PathBuf,
    pub fnc: Option<String>,
    pub args: Vec<Value>,
}

impl SurrealismCommand for RunCommand {
    fn run(self) -> Result<()> {
        let package = SurrealismPackage::from_file(self.file)?;

        // Load the WASM module
        let mut controller = surrealism_runtime::controller::Controller::from_package(package)
            .with_context(|| "Failed to load WASM module")?;

        // Invoke the function with the provided arguments
        let result = controller
            .invoke(self.fnc, self.args)
            .with_context(|| "Failed to invoke function")?;

        // Print the result with pretty display formatting
        println!("{:#}", result);

        Ok(())
    }
}
