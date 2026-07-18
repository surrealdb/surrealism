//! Discord webhook integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::discord AS f"bucket:/discord.surli";`
//! and call `mod::discord::send(...)`.

use anyhow::{Context, Result};
use surrealism::surrealism;

fn post_json(url: &str, body: serde_json::Value) -> Result<String> {
	let value: serde_json::Value =
		surrealism::run("http::post".to_string(), None, (url.to_string(), body))
			.context("Call to host 'http::post' failed")?;
	Ok(value.to_string())
}

/// Sends a plain-text message via a Discord webhook URL.
#[surrealism]
fn send(webhook_url: String, content: String) -> Result<String> {
	post_json(&webhook_url, serde_json::json!({ "content": content }))
}

/// Sends a rich embed via a Discord webhook URL. `color` is a decimal RGB
/// integer, e.g. `16711680` for red.
#[surrealism]
fn send_embed(
	webhook_url: String,
	title: String,
	description: String,
	color: i64,
) -> Result<String> {
	post_json(
		&webhook_url,
		serde_json::json!({
			"embeds": [{
				"title": title,
				"description": description,
				"color": color,
			}],
		}),
	)
}
