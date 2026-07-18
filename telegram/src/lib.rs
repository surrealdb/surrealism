//! Telegram Bot API integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::telegram AS f"bucket:/telegram.surli";`
//! and call `mod::telegram::send(...)`.

use anyhow::{Context, Result};
use surrealism::surrealism;

fn post_json(url: &str, body: serde_json::Value) -> Result<String> {
	let value: serde_json::Value =
		surrealism::run("http::post".to_string(), None, (url.to_string(), body))
			.context("Call to host 'http::post' failed")?;
	Ok(value.to_string())
}

/// Sends a plain-text message to a Telegram chat via a bot.
#[surrealism]
fn send(bot_token: String, chat_id: String, text: String) -> Result<String> {
	let url = format!("https://api.telegram.org/bot{bot_token}/sendMessage");
	post_json(
		&url,
		serde_json::json!({ "chat_id": chat_id, "text": text }),
	)
}
