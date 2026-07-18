//! SendGrid transactional email integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::sendgrid AS f"bucket:/sendgrid.surli";`
//! and call `mod::sendgrid::send(...)`.

use anyhow::{Context, Result};
use serde_json::json;
use surrealism::surrealism;

fn post_json_with_headers(
	url: &str,
	body: serde_json::Value,
	headers: serde_json::Value,
) -> Result<String> {
	let value: serde_json::Value = surrealism::run(
		"http::post".to_string(),
		None,
		(url.to_string(), body, headers),
	)
	.context("Call to host 'http::post' failed")?;
	Ok(value.to_string())
}

/// Sends a plain-text email via the SendGrid API using the given API key.
#[surrealism]
fn send(api_key: String, from: String, to: String, subject: String, body: String) -> Result<String> {
	post_json_with_headers(
		"https://api.sendgrid.com/v3/mail/send",
		json!({
			"personalizations": [{ "to": [{ "email": to }] }],
			"from": { "email": from },
			"subject": subject,
			"content": [{ "type": "text/plain", "value": body }],
		}),
		json!({ "Authorization": format!("Bearer {api_key}") }),
	)
}
