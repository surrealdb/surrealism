//! Microsoft Teams webhook integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::teams AS f"bucket:/teams.surli";`
//! and call `mod::teams::send(...)`.

use anyhow::{Context, Result};
use serde_json::{Value, json};
use surrealism::surrealism;

fn post_json(url: &str, body: serde_json::Value) -> Result<String> {
	let value: serde_json::Value =
		surrealism::run("http::post".to_string(), None, (url.to_string(), body))
			.context("Call to host 'http::post' failed")?;
	Ok(value.to_string())
}

fn post_json_with_headers(url: &str, body: Value, headers: Value) -> Result<Value> {
	surrealism::run("http::post".to_string(), None, (url.to_string(), body, headers))
		.context("Call to host 'http::post' failed")
}

/// Sends a plain-text message via a Microsoft Teams incoming webhook URL.
#[surrealism]
fn send(webhook_url: String, text: String) -> Result<String> {
	post_json(
		&webhook_url,
		serde_json::json!({
			"@type": "MessageCard",
			"@context": "http://schema.org/extensions",
			"text": text,
		}),
	)
}

/// Posts a plain-text message to a Teams channel via the Microsoft Graph API.
#[surrealism]
fn post_channel_message(
	token: String,
	team_id: String,
	channel_id: String,
	text: String,
) -> Result<Value, String> {
	post_json_with_headers(
		&format!("https://graph.microsoft.com/v1.0/teams/{team_id}/channels/{channel_id}/messages"),
		json!({ "body": { "content": text } }),
		json!({ "Authorization": format!("Bearer {token}") }),
	)
	.map_err(|e| e.to_string())
}
