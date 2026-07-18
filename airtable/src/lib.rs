//! Airtable integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::airtable AS f"bucket:/airtable.surli";`
//! and call `mod::airtable::create_record(...)`, `mod::airtable::list_records(...)`,
//! and `mod::airtable::get_record(...)`. Every request sends `Authorization:
//! Bearer {api_key}`. Table names are percent-encoded before being placed in
//! the URL path; `base_id` and `record_id` are Airtable's own URL-safe ID
//! format and are used as-is.

use anyhow::{Context, Result};
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use serde_json::{Value, json};
use surrealism::surrealism;

const PATH_SEGMENT: &AsciiSet = &CONTROLS
	.add(b' ')
	.add(b'"')
	.add(b'#')
	.add(b'%')
	.add(b'/')
	.add(b'<')
	.add(b'>')
	.add(b'?')
	.add(b'`')
	.add(b'{')
	.add(b'}');

fn encode_table(table: &str) -> String {
	utf8_percent_encode(table, PATH_SEGMENT).to_string()
}

fn auth_headers(api_key: &str) -> Value {
	json!({ "Authorization": format!("Bearer {api_key}") })
}

fn post_json_with_headers(url: &str, body: Value, headers: Value) -> Result<Value> {
	surrealism::run("http::post".to_string(), None, (url.to_string(), body, headers))
		.context("Call to host 'http::post' failed")
}

fn get_json_with_headers(url: &str, headers: Value) -> Result<Value> {
	surrealism::run("http::get".to_string(), None, (url.to_string(), headers))
		.context("Call to host 'http::get' failed")
}

/// Creates a record in a table.
#[surrealism]
fn create_record(api_key: String, base_id: String, table: String, fields: Value) -> Result<Value, String> {
	post_json_with_headers(
		&format!("https://api.airtable.com/v0/{base_id}/{}", encode_table(&table)),
		json!({ "fields": fields }),
		auth_headers(&api_key),
	)
	.map_err(|e| e.to_string())
}

/// Lists records in a table.
#[surrealism]
fn list_records(api_key: String, base_id: String, table: String) -> Result<Value, String> {
	get_json_with_headers(
		&format!("https://api.airtable.com/v0/{base_id}/{}", encode_table(&table)),
		auth_headers(&api_key),
	)
	.map_err(|e| e.to_string())
}

/// Fetches a single record from a table.
#[surrealism]
fn get_record(api_key: String, base_id: String, table: String, record_id: String) -> Result<Value, String> {
	get_json_with_headers(
		&format!(
			"https://api.airtable.com/v0/{base_id}/{}/{record_id}",
			encode_table(&table)
		),
		auth_headers(&api_key),
	)
	.map_err(|e| e.to_string())
}
