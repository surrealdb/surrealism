//! ntfy.sh push notification integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::ntfy AS f"bucket:/ntfy.surli";`
//! and call `mod::ntfy::send("mytopic", "Hello")` or
//! `mod::ntfy::send_titled("mytopic", "Alert", "Hello")`.

use anyhow::{Context, Result};
use serde_json::{Value, json};
use surrealism::surrealism;

fn post(topic: &str, body: String, headers: Value) -> Result<Value> {
	let url = format!("https://ntfy.sh/{topic}");
	surrealism::run("http::post".to_string(), None, (url, body, headers))
		.context("Call to host 'http::post' failed")
}

/// Sends a plain-text push notification to an ntfy.sh topic.
#[surrealism]
fn send(topic: String, message: String) -> Result<Value> {
	post(&topic, message, json!({}))
}

/// Sends a titled plain-text push notification to an ntfy.sh topic.
#[surrealism]
fn send_titled(topic: String, title: String, message: String) -> Result<Value> {
	post(&topic, message, json!({ "Title": title }))
}
