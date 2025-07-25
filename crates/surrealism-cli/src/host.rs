use anyhow::{Context, Result};
use candle_core::DType;
use std::{io::BufRead, sync::Arc};
use std::path::PathBuf;
use surrealdb::expr;
use surrealism_runtime::{
    config::SurrealismConfig,
    host::Host,
    kv::{BTreeMapStore, KVStore},
};
use surrealml_llms::{
    interface::{load_model::load_model, run_model::run_model},
    models::model_spec::{model_spec_trait::ModelSpec, models::gemma::Gemma},
};

use crate::parse_value;

pub struct DemoHost {
    kv: BTreeMapStore,
}

impl DemoHost {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            kv: BTreeMapStore::new(),
        })
    }
}

impl Host for DemoHost {
    fn kv(&self) -> &dyn KVStore {
        &self.kv
    }

    fn sql(&self, _config: &SurrealismConfig, query: String, vars: expr::Object) -> Result<expr::Value> {
        println!("The module is running a SQL query:");
        println!("SQL: {query}");
        println!("Vars: {vars:#}");
        println!("Please enter the result:");

        loop {
            match parse_value(&mut std::io::stdin().lock().lines().next().unwrap().unwrap()) {
                Ok(x) => {
                    println!(" ");
                    return Ok(x);
                }
                Err(e) => {
                    println!("Failed to parse value: {e}");
                    println!("Please try again");
                }
            }
        }
    }

    fn run(
        &self,
        _config: &SurrealismConfig,
        fnc: String,
        version: Option<String>,
        args: Vec<expr::Value>,
    ) -> Result<expr::Value> {
        let version = version.map(|x| format!("<{x}>")).unwrap_or_default();
        println!("The module is running a function:");
        println!(
            " - {fnc}{version}({})",
            args.iter()
                .map(|x| format!("{x:}"))
                .collect::<Vec<String>>()
                .join(", ")
        );
        println!("\nPlease enter the result:");

        loop {
            match parse_value(&mut std::io::stdin().lock().lines().next().unwrap().unwrap()) {
                Ok(x) => {
                    println!(" ");
                    return Ok(x);
                }
                Err(e) => {
                    println!("Failed to parse value: {e}");
                    println!("Please try again");
                }
            }
        }
    }

    // "google/gemma-7b"
    fn ml_invoke_model(
        &self,
        _config: &SurrealismConfig,
        model: String,
        input: expr::Value,
        weight: i64,
        weight_dir: String,
    ) -> Result<expr::Value> {
        let expr::Value::Strand(input) = input else {
            anyhow::bail!("Expected string input")
        };
        let home = std::env::var("HOME")?;
        // For HF cached weights at to be loaded but we can store the weights somewhere for all
        // later and reference them.
        // let weight_path = "google--gemma-7b";
        let base = PathBuf::from(home).join(
            format!(".cache/huggingface/hub/models--{}/snapshots", &weight_dir).replace("'", ""),
        );

        let snapshot = std::fs::read_dir(&base)?
            .next()
            .ok_or_else(|| anyhow::anyhow!("No snapshot found"))??
            .path();

        let names = Gemma.return_tensor_filenames();
        let paths: Vec<PathBuf> = names.into_iter().map(|f| snapshot.join(f)).collect();
        let mut wrapper = load_model(&model, DType::F16, Some(paths), None)
            .context("Gemma should load from local cache")?;
        let input = input.to_string();
        Ok(run_model(&mut wrapper, input, 20)
            .context("run_model should succeed")?
            .into())
    }

    fn ml_tokenize(&self, _config: &SurrealismConfig, model: String, input: expr::Value) -> Result<Vec<f64>> {
        println!("The module is running a ML tokenizer:");
        println!("Model: {model}");
        println!("Input: {input:}");
        println!("Please enter the result:");

        loop {
            match parse_value(&mut std::io::stdin().lock().lines().next().unwrap().unwrap()) {
                Ok(x) => {
                    if let expr::Value::Array(x) = x {
                        let arr = x
                            .into_iter()
                            .map(|x| -> Result<f64> {
                                if let expr::Value::Number(expr::Number::Float(x)) = x {
                                    Ok(x)
                                } else {
                                    Err(anyhow::anyhow!("Expected array of f64"))
                                }
                            })
                            .collect::<Result<Vec<f64>>>()?;

                        println!(" ");
                        return Ok(arr);
                    }
                    return Err(anyhow::anyhow!("Expected array of f64"));
                }
                Err(e) => {
                    println!("Failed to parse value: {e}");
                    println!("Please try again");
                }
            }
        }
    }

    fn stdout(&self, output: &str) -> Result<()> {
        println!("[surli::out] {}", output);
        Ok(())
    }

    fn stderr(&self, output: &str) -> Result<()> {
        eprintln!("[surli::err] {}", output);
        Ok(())
    }
}
