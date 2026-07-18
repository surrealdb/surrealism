//! Bluesky / AT Protocol integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::bluesky AS f"bucket:/bluesky.surli";`
//! and call `mod::bluesky::create_session("handle.bsky.social", "app-password")`
//! then `mod::bluesky::post(access_jwt, did, "Hello, Bluesky!")`.

use anyhow::{Context, Result};
use chrono::{SecondsFormat, Utc};
use serde_json::{Value, json};
use surrealism::surrealism;

fn post_json(url: &str, body: Value) -> Result<Value> {
	surrealism::run("http::post".to_string(), None, (url.to_string(), body))
		.context("Call to host 'http::post' failed")
}

fn post_json_with_headers(url: &str, body: Value, headers: Value) -> Result<Value> {
	surrealism::run("http::post".to_string(), None, (url.to_string(), body, headers))
		.context("Call to host 'http::post' failed")
}

/// Creates an AT Protocol session; the response contains `accessJwt` and `did`.
#[surrealism]
fn create_session(identifier: String, app_password: String) -> Result<Value, String> {
	post_json(
		"https://bsky.social/xrpc/com.atproto.server.createSession",
		json!({ "identifier": identifier, "password": app_password }),
	)
	.map_err(|e| e.to_string())
}

/// Publishes a text post to the authenticated account's repo.
#[surrealism]
fn post(access_jwt: String, did: String, text: String) -> Result<Value, String> {
	post_json_with_headers(
		"https://bsky.social/xrpc/com.atproto.repo.createRecord",
		json!({
			"repo": did,
			"collection": "app.bsky.feed.post",
			"record": {
				"$type": "app.bsky.feed.post",
				"text": text,
				"createdAt": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
			},
		}),
		json!({ "Authorization": format!("Bearer {access_jwt}") }),
	)
	.map_err(|e| e.to_string())
}
