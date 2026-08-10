//! Google Sheets API v4 integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::googlesheets AS f"bucket:/googlesheets.surli";`
//! and call `mod::googlesheets::append_row(...)` and
//! `mod::googlesheets::get_values(...)`. This module does not perform Google's
//! OAuth2 flow; the caller supplies an already-valid OAuth2 access token.
//! Every request sends `Authorization: Bearer {access_token}`. `range` is
//! Google's A1 notation (e.g. `Sheet1!A1:D1`) and is percent-encoded before
//! being placed in the URL path.

use anyhow::{Context, Result};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde_json::{Value, json};
use surrealism::surrealism;

fn encode_range(range: &str) -> String {
	utf8_percent_encode(range, NON_ALPHANUMERIC).to_string()
}

fn auth_headers(access_token: &str) -> Value {
	json!({ "Authorization": format!("Bearer {access_token}") })
}

fn post_json_with_headers(url: &str, body: Value, headers: Value) -> Result<Value> {
	surrealism::run("http::post".to_string(), None, (url.to_string(), body, headers))
		.context("Call to host 'http::post' failed")
}

fn get_json_with_headers(url: &str, headers: Value) -> Result<Value> {
	surrealism::run("http::get".to_string(), None, (url.to_string(), headers))
		.context("Call to host 'http::get' failed")
}

/// Appends a row of values to a sheet.
#[surrealism]
fn append_row(
	access_token: String,
	spreadsheet_id: String,
	range: String,
	values: Vec<String>,
) -> Result<Value, String> {
	post_json_with_headers(
		&format!(
			"https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values/{}:append?valueInputOption=USER_ENTERED",
			encode_range(&range)
		),
		json!({ "values": [values] }),
		auth_headers(&access_token),
	)
	.map_err(|e| e.to_string())
}

/// Fetches the values in a range of a sheet.
#[surrealism]
fn get_values(
	access_token: String,
	spreadsheet_id: String,
	range: String,
) -> Result<Value, String> {
	get_json_with_headers(
		&format!(
			"https://sheets.googleapis.com/v4/spreadsheets/{spreadsheet_id}/values/{}",
			encode_range(&range)
		),
		auth_headers(&access_token),
	)
	.map_err(|e| e.to_string())
}
