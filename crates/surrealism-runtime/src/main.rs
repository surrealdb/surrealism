mod controller;
use controller::Controller;

// fn main() {
//     let mut controller = Controller::from_file("out.wasm");
//     let res = controller.invoke(Some("can_drive".to_string()), (20,)).unwrap();
//     println!("res {res}");
// }

use clap::{Parser, Subcommand};
use surrealdb::sql::Value;
use std::path::PathBuf;

/// CLI definition
#[derive(Debug, Parser)]
#[command(name = "cli-name")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run a function with arguments
    Run {
        /// Arguments passed to function (repeatable)
        #[arg(long = "arg", value_parser = parse_value)]
        args: Vec<Value>,

        /// Required name
        #[arg(long)]
        name: Option<String>,

        /// Path to WASM file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Show the function signature
    Sig {
        /// Required name
        #[arg(long)]
        name: Option<String>,

        /// Path to WASM file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

/// Custom parser for `surrealdb::sql::Value`
fn parse_value(s: &str) -> Result<Value, String> {
    surrealdb::sql::value(s).map_err(|e| format!("Invalid value: {e}"))
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { args, name, file } => {
            println!("Run: name={}, file={file:?}, args={args:?}", name.clone().unwrap_or("<default>".to_string()));
            let mut controller = Controller::from_file(file.to_str().unwrap());
            let kinds = controller.args(name.clone()).unwrap();

            if args.len() != kinds.len() {
                panic!("Arguments len mismatch");
            }

            // coerce_to is internal :/
            // for (i, kind) in kinds.into_iter().enumerate() {
            //     let arg = args.get(i).unwrap();
            //     arg.coerce_to_i64()
            // }

            let res = controller.invoke(name, args).unwrap();
            println!("Result:\n - {res}");
        }
        Commands::Sig { name, file } => {
            println!("Sig: name={}, file={file:?}", name.clone().unwrap_or("<default>".to_string()));
            let mut controller = Controller::from_file(file.to_str().unwrap());
            let args = controller.args(name.clone()).unwrap();
            let returns = controller.returns(name).unwrap();

            println!("Arguments:");
            if args.is_empty() {
                println!(" - None");
            } else {
                for (i, arg) in args.into_iter().enumerate() {
                    println!("- {i}: {arg}");
                }
            }

            println!("\nReturns:\n - {returns}");
        }
    }
}