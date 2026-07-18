//! iCalendar (.ics) event generation for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::ical AS f"bucket:/ical.surli";` and
//! call `mod::ical::event("Standup", "2026-08-01T09:00:00Z", "2026-08-01T09:15:00Z")`
//! or `mod::ical::event_with_details("Standup", "2026-08-01T09:00:00Z", "2026-08-01T09:15:00Z", "Daily sync", "Room 1")`.
//! `start` and `end` are RFC3339 datetime strings.

use chrono::{DateTime, Utc};
use icalendar::{Calendar, Component, Event, EventLike};
use surrealism::surrealism;

fn parse_rfc3339(value: &str) -> Result<DateTime<Utc>, String> {
	DateTime::parse_from_rfc3339(value)
		.map(|dt| dt.with_timezone(&Utc))
		.map_err(|e| e.to_string())
}

/// Builds a single-event .ics calendar and returns the full iCalendar text.
#[surrealism]
fn event(summary: String, start: String, end: String) -> Result<String, String> {
	let starts = parse_rfc3339(&start)?;
	let ends = parse_rfc3339(&end)?;
	let event = Event::new().summary(&summary).starts(starts).ends(ends).done();
	let mut calendar = Calendar::new();
	calendar.push(event);
	Ok(calendar.done().to_string())
}

/// Builds a single-event .ics calendar with a description and location, and returns the full iCalendar text.
#[surrealism]
fn event_with_details(
	summary: String,
	start: String,
	end: String,
	description: String,
	location: String,
) -> Result<String, String> {
	let starts = parse_rfc3339(&start)?;
	let ends = parse_rfc3339(&end)?;
	let event = Event::new()
		.summary(&summary)
		.starts(starts)
		.ends(ends)
		.description(&description)
		.location(&location)
		.done();
	let mut calendar = Calendar::new();
	calendar.push(event);
	Ok(calendar.done().to_string())
}
