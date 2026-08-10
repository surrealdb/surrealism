//! OpenAI embeddings and chat completions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::openai AS f"bucket:/openai.surli";`
//! and call `mod::openai::embed(...)`, `mod::openai::chat(...)`, and
//! `mod::openai::generate(...)`. Every request sends `Authorization: Bearer
//! {api_key}`.

use anyhow::{Context, Result};
use serde_json::{Value, json};
use surrealism::surrealism;

fn auth_headers(api_key: &str) -> Value {
	json!({ "Authorization": format!("Bearer {api_key}") })
}

fn post_json_with_headers(url: &str, body: Value, headers: Value) -> Result<Value> {
	surrealism::run("http::post".to_string(), None, (url.to_string(), body, headers))
		.context("Call to host 'http::post' failed")
}

fn completion(api_key: &str, model: String, messages: Value) -> Result<String, String> {
	let response = post_json_with_headers(
		"https://api.openai.com/v1/chat/completions",
		json!({ "model": model, "messages": messages }),
		auth_headers(api_key),
	)
	.map_err(|e| e.to_string())?;

	response["choices"][0]["message"]["content"]
		.as_str()
		.map(str::to_string)
		.ok_or_else(|| "Response contained no message content".to_string())
}

/// Embeds a piece of text with an OpenAI embedding model.
#[surrealism]
fn embed(api_key: String, model: String, input: String) -> Result<Vec<f64>, String> {
	let response = post_json_with_headers(
		"https://api.openai.com/v1/embeddings",
		json!({ "model": model, "input": input }),
		auth_headers(&api_key),
	)
	.map_err(|e| e.to_string())?;

	response["data"][0]["embedding"]
		.as_array()
		.ok_or_else(|| "Response contained no embedding values".to_string())?
		.iter()
		.map(|v| v.as_f64().ok_or_else(|| "Embedding contains a non-numeric value".to_string()))
		.collect()
}

/// Runs a chat completion over an array of `{ role, content }` messages.
#[surrealism]
fn chat(api_key: String, model: String, messages: Value) -> Result<String, String> {
	completion(&api_key, model, messages)
}

/// Runs a one-shot chat completion for a single user prompt.
#[surrealism]
fn generate(api_key: String, model: String, prompt: String) -> Result<String, String> {
	completion(&api_key, model, json!([{ "role": "user", "content": prompt }]))
}
