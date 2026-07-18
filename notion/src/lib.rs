//! Notion API integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::notion AS f"bucket:/notion.surli";`
//! and call `mod::notion::create_page(...)`, `mod::notion::get_page(...)`,
//! and `mod::notion::query_database(...)`. Every request sends `Authorization:
//! Bearer {token}` and a `Notion-Version` header.

use anyhow::{Context, Result};
use serde_json::{Value, json};
use surrealism::surrealism;

fn auth_headers(token: &str) -> Value {
	json!({
		"Authorization": format!("Bearer {token}"),
		"Notion-Version": "2022-06-28",
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

/// Creates a page in a database. `properties` must match the target database's property schema.
#[surrealism]
fn create_page(token: String, parent_database_id: String, properties: Value) -> Result<Value, String> {
	post_json_with_headers(
		"https://api.notion.com/v1/pages",
		json!({ "parent": { "database_id": parent_database_id }, "properties": properties }),
		auth_headers(&token),
	)
	.map_err(|e| e.to_string())
}

/// Fetches a page by id.
#[surrealism]
fn get_page(token: String, page_id: String) -> Result<Value, String> {
	get_json_with_headers(&format!("https://api.notion.com/v1/pages/{page_id}"), auth_headers(&token))
		.map_err(|e| e.to_string())
}

/// Queries all rows in a database, unfiltered.
#[surrealism]
fn query_database(token: String, database_id: String) -> Result<Value, String> {
	post_json_with_headers(
		&format!("https://api.notion.com/v1/databases/{database_id}/query"),
		json!({}),
		auth_headers(&token),
	)
	.map_err(|e| e.to_string())
}
