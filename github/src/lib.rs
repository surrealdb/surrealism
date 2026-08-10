//! GitHub REST API integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::github AS f"bucket:/github.surli";`
//! and call `mod::github::create_issue(...)`, `mod::github::create_comment(...)`,
//! and `mod::github::get_issue(...)`. Every request sends `Authorization: Bearer
//! {token}`, a `User-Agent` header (GitHub rejects requests without one with a
//! 403), and `Accept: application/vnd.github+json`.

use anyhow::{Context, Result};
use serde_json::{Value, json};
use surrealism::surrealism;

fn auth_headers(token: &str) -> Value {
	json!({
		"Authorization": format!("Bearer {token}"),
		"User-Agent": "surrealism-github",
		"Accept": "application/vnd.github+json",
	})
}

fn post_json_with_headers(url: &str, body: Value, headers: Value) -> Result<Value> {
	surrealism::run("http::post".to_string(), None, (url.to_string(), body, headers))
		.context("Call to host 'http::post' failed")
}

fn get_json_with_headers(url: &str, headers: Value) -> Result<Value> {
	surrealism::run("http::get".to_string(), None, (url.to_string(), headers))
		.context("Call to host 'http::get' failed")
}

/// Creates an issue in a repository.
#[surrealism]
fn create_issue(
	token: String,
	owner: String,
	repo: String,
	title: String,
	body: String,
) -> Result<Value, String> {
	post_json_with_headers(
		&format!("https://api.github.com/repos/{owner}/{repo}/issues"),
		json!({ "title": title, "body": body }),
		auth_headers(&token),
	)
	.map_err(|e| e.to_string())
}

/// Creates a comment on an issue.
#[surrealism]
fn create_comment(
	token: String,
	owner: String,
	repo: String,
	issue_number: i64,
	body: String,
) -> Result<Value, String> {
	post_json_with_headers(
		&format!("https://api.github.com/repos/{owner}/{repo}/issues/{issue_number}/comments"),
		json!({ "body": body }),
		auth_headers(&token),
	)
	.map_err(|e| e.to_string())
}

/// Fetches an issue from a repository.
#[surrealism]
fn get_issue(
	token: String,
	owner: String,
	repo: String,
	issue_number: i64,
) -> Result<Value, String> {
	get_json_with_headers(
		&format!("https://api.github.com/repos/{owner}/{repo}/issues/{issue_number}"),
		auth_headers(&token),
	)
	.map_err(|e| e.to_string())
}
