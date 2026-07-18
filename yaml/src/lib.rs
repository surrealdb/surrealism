//! YAML/JSON conversion functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::yaml AS f"bucket:/yaml.surli";`
//! and call `mod::yaml::to_json("a: 1\nb: [1, 2]")`,
//! `mod::yaml::from_json({ a: 1, b: [1, 2] })`, etc.

use serde_json::Value;
use surrealism::surrealism;

/// Parses a YAML document into a value.
#[surrealism]
fn to_json(yaml: String) -> Result<Value, String> {
	serde_yaml::from_str::<Value>(&yaml).map_err(|e| e.to_string())
}

/// Serializes a value into a YAML document.
#[surrealism]
fn from_json(value: Value) -> Result<String, String> {
	serde_yaml::to_string(&value).map_err(|e| e.to_string())
}
