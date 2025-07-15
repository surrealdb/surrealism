use anyhow::Result;
use std::io::BufRead;
use surrealdb::sql;
use surrealism_runtime::{
    host::Host,
    kv::{BTreeMapStore, KVStore},
};
use std::path::PathBuf;
use surrealml_llms::{
    interface::{load_model::load_model, run_model::run_model},
    models::model_spec::{model_spec_trait::ModelSpec, models::gemma::Gemma},
};
use candle_core::DType;


use crate::parse_value;

pub struct DemoHost {
    kv: BTreeMapStore,
}

impl DemoHost {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {
            kv: BTreeMapStore::new(),
        })
    }
}

impl Host for DemoHost {
    fn kv(&mut self) -> &mut dyn KVStore {
        &mut self.kv
    }

    fn sql(&self, query: String, vars: sql::Object) -> Result<sql::Value> {
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
        fnc: String,
        version: Option<String>,
        args: Vec<sql::Value>,
    ) -> Result<sql::Value> {
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
    fn ml_invoke_model(&self, model: String, input: sql::Value, weight_dir: sql::Value) -> Result<sql::Value> {
        let sql::Value::Strand(input) = input else {
            anyhow::bail!("Expected string input")
        };
        let sql::Value::Strand(weight_dir) = weight_dir else {
            anyhow::bail!("Expected string input")
        };
        let home = std::env::var("HOME").unwrap();
        // For HF cached weights at to be loaded but we can store the weights somewhere for all
        // later and reference them.
        // let weight_path = "google--gemma-7b";
        let base = PathBuf::from(home).join(
            format!(".cache/huggingface/hub/models--{}/snapshots", &weight_dir)
        );
        
        let snapshot = std::fs::read_dir(&base)
            .unwrap()
            .next()
            .unwrap()?
            .path();

        let names = Gemma.return_tensor_filenames();
        let paths: Vec<PathBuf> = names.into_iter().map(|f| snapshot.join(f)).collect();
        let mut wrapper = load_model(&model, DType::F16, Some(paths), None)
            .expect("Gemma should load from local cache");
        let input = input.to_string();
        Ok(run_model(&mut wrapper, input, 20).expect("run_model should succeed").into())
    }

    fn ml_tokenize(&self, model: String, input: sql::Value) -> Result<Vec<f64>> {
        println!("The module is running a ML tokenizer:");
        println!("Model: {model}");
        println!("Input: {input:}");
        println!("Please enter the result:");

        loop {
            match parse_value(&mut std::io::stdin().lock().lines().next().unwrap().unwrap()) {
                Ok(x) => {
                    if let sql::Value::Array(x) = x {
                        let arr = x
                            .into_iter()
                            .map(|x| -> Result<f64> {
                                if let sql::Value::Number(sql::Number::Float(x)) = x {
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
