//! Anthropic Claude message generation for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::anthropic AS f"bucket:/anthropic.surli";`
//! and call `mod::anthropic::chat(...)` and `mod::anthropic::generate(...)`.
//! Every request POSTs to `https://api.anthropic.com/v1/messages` with an
//! `x-api-key` header and an `anthropic-version` header, and carries the
//! required `max_tokens` body field.

use anyhow::{Context, Result};
use serde_json::{Value, json};
use surrealism::surrealism;

fn auth_headers(api_key: &str) -> Value {
	json!({
		"x-api-key": api_key,
		"anthropic-version": "2023-06-01",
	})
}

fn post_messages(api_key: &str, body: Value) -> Result<Value> {
	surrealism::run(
		"http::post".to_string(),
		None,
		(
			"https://api.anthropic.com/v1/messages".to_string(),
			body,
			auth_headers(api_key),
		),
	)
	.context("Call to host 'http::post' failed")
}

/// Returns the text of the first content block whose type is `text`.
fn first_text(response: &Value) -> Result<String, String> {
	response["content"]
		.as_array()
		.and_then(|blocks| blocks.iter().find(|block| block["type"] == "text"))
		.and_then(|block| block["text"].as_str())
		.map(str::to_string)
		.ok_or_else(|| "Anthropic response contains no text content block".to_string())
}

/// Sends an array of `{ role, content }` messages to a model and returns the generated text.
#[surrealism]
fn chat(api_key: String, model: String, messages: Value, max_tokens: i64) -> Result<String, String> {
	let response = post_messages(
		&api_key,
		json!({ "model": model, "max_tokens": max_tokens, "messages": messages }),
	)
	.map_err(|e| e.to_string())?;
	first_text(&response)
}

/// Sends a single user prompt to a model and returns the generated text.
#[surrealism]
fn generate(api_key: String, model: String, prompt: String, max_tokens: i64) -> Result<String, String> {
	let response = post_messages(
		&api_key,
		json!({
			"model": model,
			"max_tokens": max_tokens,
			"messages": [{ "role": "user", "content": prompt }],
		}),
	)
	.map_err(|e| e.to_string())?;
	first_text(&response)
}
