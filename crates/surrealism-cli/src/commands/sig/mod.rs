use std::path::PathBuf;
use anyhow::Context;
use surrealism_runtime::package::SurrealismPackage;
use crate::commands::SurrealismCommand;

pub struct SigCommand {
    pub file: PathBuf,
    pub fnc: Option<String>,
}

impl SurrealismCommand for SigCommand {
    fn run(self) -> anyhow::Result<()> {
        let package = SurrealismPackage::from_file(self.file)
            .with_context(|| "Failed to load Surrealism package")?;

        // Load the WASM module from memory
        let mut controller = surrealism_runtime::controller::Controller::from_package(package)
            .with_context(|| "Failed to load WASM module")?;

        // Invoke the function with the provided arguments
        let args = controller.args(self.fnc.clone()).with_context(|| "Failed to collect arguments")?;
        let returns = controller.returns(self.fnc.clone()).with_context(|| "Failed to collect return type")?;

        println!("\nSignature:\n - {}({}) -> {}", 
            self.fnc.as_deref().unwrap_or("<default>"),
            args.iter().map(|arg| format!("{}", arg)).collect::<Vec<_>>().join(", "),
            returns
        );

        Ok(())
    }
}

