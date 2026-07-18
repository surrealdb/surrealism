//! Postmark transactional email integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::postmark AS f"bucket:/postmark.surli";`
//! and call `mod::postmark::send(...)`.

use anyhow::{Context, Result};
use surrealism::surrealism;

fn post_json_with_headers(
	url: &str,
	headers: serde_json::Value,
	body: serde_json::Value,
) -> Result<String> {
	let value: serde_json::Value = surrealism::run(
		"http::post".to_string(),
		None,
		(url.to_string(), body, headers),
	)
	.context("Call to host 'http::post' failed")?;
	Ok(value.to_string())
}

/// Sends a transactional email via the Postmark API.
#[surrealism]
fn send(server_token: String, from: String, to: String, subject: String, body: String) -> Result<String> {
	let headers = serde_json::json!({ "X-Postmark-Server-Token": server_token });
	let payload = serde_json::json!({
		"From": from,
		"To": to,
		"Subject": subject,
		"TextBody": body,
	});
	post_json_with_headers("https://api.postmarkapp.com/email", headers, payload)
}
