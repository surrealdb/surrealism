//! Mastodon API integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::mastodon AS f"bucket:/mastodon.surli";`
//! and call `mod::mastodon::post_status("https://mastodon.social", "token", "Hello, fediverse!")`.

use anyhow::{Context, Result};
use serde_json::{Value, json};
use surrealism::surrealism;

fn post_json_with_headers(url: &str, body: Value, headers: Value) -> Result<Value> {
	surrealism::run("http::post".to_string(), None, (url.to_string(), body, headers))
		.context("Call to host 'http::post' failed")
}

/// Publishes a status on the given Mastodon instance.
#[surrealism]
fn post_status(
	instance_url: String,
	access_token: String,
	status: String,
) -> Result<Value, String> {
	post_json_with_headers(
		&format!("{instance_url}/api/v1/statuses"),
		json!({ "status": status }),
		json!({ "Authorization": format!("Bearer {access_token}") }),
	)
	.map_err(|e| e.to_string())
}
