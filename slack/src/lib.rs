//! Slack Incoming Webhook and Web API integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::slack AS f"bucket:/slack.surli";`
//! and call `mod::slack::send(...)` or `mod::slack::post_message(...)`.

use anyhow::{Context, Result};
use serde_json::{Value, json};
use surrealism::surrealism;

fn post_json(url: &str, body: serde_json::Value) -> Result<String> {
	let value: serde_json::Value =
		surrealism::run("http::post".to_string(), None, (url.to_string(), body))
			.context("Call to host 'http::post' failed")?;
	Ok(value.to_string())
}

/// Sends a plain-text message via a Slack Incoming Webhook URL.
#[surrealism]
fn send(webhook_url: String, text: String) -> Result<String> {
	post_json(&webhook_url, serde_json::json!({ "text": text }))
}

/// Posts a message to a channel via the Slack Web API using a Bearer token.
#[surrealism]
fn post_message(token: String, channel: String, text: String) -> Result<Value, String> {
	let response: Value = surrealism::run(
		"http::post".to_string(),
		None,
		(
			"https://slack.com/api/chat.postMessage".to_string(),
			json!({ "channel": channel, "text": text }),
			json!({ "Authorization": format!("Bearer {token}") }),
		),
	)
	.context("Call to host 'http::post' failed")
	.map_err(|e| e.to_string())?;

	if response["ok"].as_bool() == Some(true) {
		Ok(response)
	} else {
		Err(response["error"]
			.as_str()
			.unwrap_or("unknown_error")
			.to_string())
	}
}
