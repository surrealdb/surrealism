//! Third-party API integrations for Surrealism — one `#[surrealism] mod`
//! per platform, so calls look like `mod::api::discord::send(...)` and
//! `mod::api::slack::send(...)`.
//!
//! Requests go through the SurrealDB host's `http::post` (via
//! `surrealism::run`) rather than a raw socket from the guest, since guest
//! code can't resolve hostnames. See `surrealism.toml` for capabilities.

use anyhow::{Context, Result};
use surrealism::surrealism;

/// Posts a JSON body to `url` via the host's `http::post`, returning the
/// response re-serialized as JSON text.
fn post_json(url: &str, body: serde_json::Value) -> Result<String> {
	let value: serde_json::Value =
		surrealism::run("http::post".to_string(), None, (url.to_string(), body))
			.context("Call to host 'http::post' failed")?;
	Ok(value.to_string())
}

#[surrealism]
mod discord {
	/// Sends a plain-text message via a Discord webhook URL.
	#[surrealism]
	fn send(webhook_url: String, content: String) -> anyhow::Result<String> {
		crate::post_json(&webhook_url, serde_json::json!({ "content": content }))
	}

	/// Sends a rich embed via a Discord webhook URL. `color` is a decimal
	/// RGB integer, e.g. `16711680` for red.
	#[surrealism]
	fn send_embed(
		webhook_url: String,
		title: String,
		description: String,
		color: i64,
	) -> anyhow::Result<String> {
		crate::post_json(
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
}

#[surrealism]
mod slack {
	/// Sends a plain-text message via a Slack Incoming Webhook URL.
	#[surrealism]
	fn send(webhook_url: String, text: String) -> anyhow::Result<String> {
		crate::post_json(&webhook_url, serde_json::json!({ "text": text }))
	}
}
