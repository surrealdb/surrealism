//! Slack Incoming Webhook integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::slack AS f"bucket:/slack.surli";`
//! and call `mod::slack::send(...)`.

use anyhow::{Context, Result};
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
