//! TOML/JSON conversion functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::toml AS f"bucket:/toml.surli";`
//! and call `mod::toml::to_json("a = 1\n[b]\nc = 2\n")`,
//! `mod::toml::from_json({ a: 1, b: { c: 2 } })`, etc.

use serde_json::Value;
use surrealism::surrealism;

/// Parses a TOML document into a value.
#[surrealism]
fn to_json(toml: String) -> Result<Value, String> {
	toml::from_str::<Value>(&toml).map_err(|e| e.to_string())
}

/// Serializes a value into a TOML document; the value must be an object and must not contain nulls.
#[surrealism]
fn from_json(value: Value) -> Result<String, String> {
	toml::to_string(&value).map_err(|e| e.to_string())
}
