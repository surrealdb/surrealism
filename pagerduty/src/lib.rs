//! PagerDuty Events API v2 integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::pagerduty AS f"bucket:/pagerduty.surli";`
//! and call `mod::pagerduty::trigger(...)` / `mod::pagerduty::resolve(...)`.
//! The `routing_key` in the request body is the authentication; no separate
//! auth header is sent.

use anyhow::{Context, Result};
use serde_json::Value;
use surrealism::surrealism;

const ENQUEUE_URL: &str = "https://events.pagerduty.com/v2/enqueue";

fn post_json(url: &str, body: Value) -> Result<Value> {
	surrealism::run("http::post".to_string(), None, (url.to_string(), body))
		.context("Call to host 'http::post' failed")
}

/// Triggers a new PagerDuty incident. `severity` must be one of "critical",
/// "error", "warning", or "info".
#[surrealism]
fn trigger(
	routing_key: String,
	summary: String,
	source: String,
	severity: String,
) -> Result<Value, String> {
	post_json(
		ENQUEUE_URL,
		serde_json::json!({
			"routing_key": routing_key,
			"event_action": "trigger",
			"payload": {
				"summary": summary,
				"source": source,
				"severity": severity,
			},
		}),
	)
	.map_err(|e| e.to_string())
}

/// Resolves an existing PagerDuty incident by its dedup key.
#[surrealism]
fn resolve(routing_key: String, dedup_key: String) -> Result<Value, String> {
	post_json(
		ENQUEUE_URL,
		serde_json::json!({
			"routing_key": routing_key,
			"event_action": "resolve",
			"dedup_key": dedup_key,
		}),
	)
	.map_err(|e| e.to_string())
}
