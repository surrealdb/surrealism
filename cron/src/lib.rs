//! Cron expression parsing and next-occurrence computation for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::cron AS f"bucket:/cron.surli";` and
//! call `mod::cron::is_valid("0 0 * * *")` or
//! `mod::cron::next("0 0 * * *", "2026-08-01T09:00:00Z")`.
//! `from` is an RFC3339 datetime string.

use chrono::{DateTime, Utc};
use croner::Cron;
use surrealism::surrealism;

fn parse_cron(expression: &str) -> Result<Cron, String> {
	Cron::new(expression).parse().map_err(|e| e.to_string())
}

/// Checks whether a cron expression parses successfully.
#[surrealism]
fn is_valid(expression: String) -> bool {
	parse_cron(&expression).is_ok()
}

/// Finds the next occurrence of a cron expression strictly after the given RFC3339 datetime.
#[surrealism]
fn next(expression: String, from: String) -> Result<String, String> {
	let cron = parse_cron(&expression)?;
	let from = DateTime::parse_from_rfc3339(&from)
		.map(|dt| dt.with_timezone(&Utc))
		.map_err(|e| e.to_string())?;
	let next = cron.find_next_occurrence(&from, false).map_err(|e| e.to_string())?;
	Ok(next.to_rfc3339())
}
