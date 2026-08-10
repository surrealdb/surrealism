//! OneSignal push notification integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::onesignal AS f"bucket:/onesignal.surli";`
//! and call `mod::onesignal::send(...)`. Every request sends `Authorization:
//! Key {api_key}`.

use anyhow::{Context, Result};
use serde_json::{Value, json};
use surrealism::surrealism;

fn post_json_with_headers(url: &str, body: Value, headers: Value) -> Result<Value> {
	surrealism::run("http::post".to_string(), None, (url.to_string(), body, headers))
		.context("Call to host 'http::post' failed")
}

/// Sends a push notification to a segment via the OneSignal API.
#[surrealism]
fn send(
	api_key: String,
	app_id: String,
	message: String,
	segment: String,
) -> Result<Value, String> {
	post_json_with_headers(
		"https://api.onesignal.com/notifications",
		json!({
			"app_id": app_id,
			"target_channel": "push",
			"contents": { "en": message },
			"included_segments": [segment],
		}),
		json!({ "Authorization": format!("Key {api_key}") }),
	)
	.map_err(|e| e.to_string())
}
