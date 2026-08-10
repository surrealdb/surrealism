//! Voyage AI text embeddings for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::voyage AS f"bucket:/voyage.surli";`
//! and call `mod::voyage::embed("pa-xxx", "voyage-4", "Hello, world!")`,
//! `mod::voyage::embed_for_query(...)`, or
//! `mod::voyage::embed_for_document(...)`. Every request sends
//! `Authorization: Bearer {api_key}`.

use anyhow::{Context, Result};
use serde_json::{Value, json};
use surrealism::surrealism;

fn post_json_with_headers(url: &str, body: Value, headers: Value) -> Result<Value> {
	surrealism::run("http::post".to_string(), None, (url.to_string(), body, headers))
		.context("Call to host 'http::post' failed")
}

fn embedding(response: &Value) -> Result<Vec<f64>, String> {
	response["data"][0]["embedding"]
		.as_array()
		.ok_or_else(|| "Response contains no 'data[0].embedding' array".to_string())?
		.iter()
		.map(|v| v.as_f64().ok_or_else(|| "Embedding contains a non-numeric value".to_string()))
		.collect()
}

fn request(api_key: &str, body: Value) -> Result<Vec<f64>, String> {
	let response = post_json_with_headers(
		"https://api.voyageai.com/v1/embeddings",
		body,
		json!({ "Authorization": format!("Bearer {api_key}") }),
	)
	.map_err(|e| e.to_string())?;
	embedding(&response)
}

/// Embeds a text string with a Voyage AI model.
#[surrealism]
fn embed(api_key: String, model: String, input: String) -> Result<Vec<f64>, String> {
	request(&api_key, json!({ "model": model, "input": input }))
}

/// Embeds a text string as the query side of a retrieval pair.
#[surrealism]
fn embed_for_query(api_key: String, model: String, input: String) -> Result<Vec<f64>, String> {
	request(&api_key, json!({ "model": model, "input": input, "input_type": "query" }))
}

/// Embeds a text string as the document side of a retrieval pair.
#[surrealism]
fn embed_for_document(api_key: String, model: String, input: String) -> Result<Vec<f64>, String> {
	request(&api_key, json!({ "model": model, "input": input, "input_type": "document" }))
}
