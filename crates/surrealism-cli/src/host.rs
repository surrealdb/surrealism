use anyhow::Result;
use std::io::BufRead;
use surrealdb::sql;
use surrealism_runtime::host::Host;

use crate::parse_value;

pub struct DemoHost {}

impl DemoHost {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Host for DemoHost {
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

    fn ml_invoke_model(&self, model: String, input: sql::Value, weight: i64) -> Result<sql::Value> {
        println!("The module is running a ML model:");
        println!("Model: {model}");
        println!("Input: {input:}");
        println!("Weight: {weight}");
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
}


