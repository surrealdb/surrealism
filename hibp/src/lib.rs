//! Have I Been Pwned k-anonymity password-breach check for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::hibp AS f"bucket:/hibp.surli";` and
//! call `mod::hibp::check_password("hunter2")`. No authentication is
//! required by the Have I Been Pwned API.

use anyhow::{Context, Result};
use serde_json::json;
use sha1::{Digest, Sha1};
use surrealism::surrealism;

fn get_text_with_headers(url: &str, headers: serde_json::Value) -> Result<String> {
	surrealism::run("http::get".to_string(), None, (url.to_string(), headers))
		.context("Call to host 'http::get' failed")
}

fn hex_upper(bytes: &[u8]) -> String {
	bytes.iter().map(|b| format!("{b:02X}")).collect()
}

/// Checks a password against the Have I Been Pwned range API using
/// k-anonymity, returning how many times its hash has been seen in breaches.
#[surrealism]
fn check_password(password: String) -> Result<i64, String> {
	let hash = hex_upper(&Sha1::digest(password.as_bytes()));
	let (prefix, suffix) = hash.split_at(5);
	let body = get_text_with_headers(&format!("https://api.pwnedpasswords.com/range/{prefix}"), json!({}))
		.map_err(|e| e.to_string())?;
	for line in body.lines() {
		if let Some((line_suffix, count)) = line.trim().split_once(':') {
			if line_suffix.eq_ignore_ascii_case(suffix) {
				return count.trim().parse::<i64>().map_err(|e| e.to_string());
			}
		}
	}
	Ok(0)
}
