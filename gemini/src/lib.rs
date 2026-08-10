//! Google Gemini embeddings and text generation for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::gemini AS f"bucket:/gemini.surli";`
//! and call `mod::gemini::embed(...)` and `mod::gemini::generate(...)`. Every
//! request sends `x-goog-api-key: {api_key}`, and `model` is placed directly
//! in the URL path.

use anyhow::{Context, Result};
use serde_json::{Value, json};
use surrealism::surrealism;

fn auth_headers(api_key: &str) -> Value {
	json!({ "x-goog-api-key": api_key })
}

fn post_json_with_headers(url: &str, body: Value, headers: Value) -> Result<Value> {
	surrealism::run("http::post".to_string(), None, (url.to_string(), body, headers))
		.context("Call to host 'http::post' failed")
}

/// Embeds a piece of text with a Gemini embedding model.
#[surrealism]
fn embed(api_key: String, model: String, text: String) -> Result<Vec<f64>, String> {
	let response = post_json_with_headers(
		&format!("https://generativelanguage.googleapis.com/v1beta/models/{model}:embedContent"),
		json!({ "content": { "parts": [{ "text": text }] } }),
		auth_headers(&api_key),
	)
	.map_err(|e| e.to_string())?;

	response["embedding"]["values"]
		.as_array()
		.ok_or_else(|| "Response contained no embedding values".to_string())?
		.iter()
		.map(|v| v.as_f64().ok_or_else(|| "Embedding contains a non-numeric value".to_string()))
		.collect()
}

/// Generates text from a prompt with a Gemini model.
#[surrealism]
fn generate(api_key: String, model: String, prompt: String) -> Result<String, String> {
	let response = post_json_with_headers(
		&format!("https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"),
		json!({ "contents": [{ "parts": [{ "text": prompt }] }] }),
		auth_headers(&api_key),
	)
	.map_err(|e| e.to_string())?;

	response["candidates"][0]["content"]["parts"][0]["text"]
		.as_str()
		.map(str::to_string)
		.ok_or_else(|| "Response contained no generated text".to_string())
}
