//! Phone number parsing, validation and formatting for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::phonenumber AS f"bucket:/phonenumber.surli";`
//! and call `mod::phonenumber::is_valid("6502530000", "US")`,
//! `mod::phonenumber::format_e164("6502530000", "US")`,
//! `mod::phonenumber::parse("6502530000", "US")`, etc.
//! `country` is a 2-letter ISO 3166-1 alpha-2 region code (e.g. "US", "GB")
//! used as the default region for numbers not already in international
//! (+...) format.

use serde_json::{Value, json};
use surrealism::surrealism;

fn parse_number(number: &str, country: &str) -> Result<phonenumber::PhoneNumber, String> {
	if number.trim_start().starts_with('+') {
		return phonenumber::parse(None, number).map_err(|e| e.to_string());
	}
	let region = country.parse::<phonenumber::country::Id>().map_err(|e| e.to_string())?;
	phonenumber::parse(Some(region), number).map_err(|e| e.to_string())
}

/// Parses a phone number and checks whether it is valid; false for anything that fails to parse.
#[surrealism]
fn is_valid(number: String, country: String) -> bool {
	parse_number(&number, &country).map(|n| n.is_valid()).unwrap_or(false)
}

/// Parses a phone number and formats it in E.164 form (e.g. "+14155552671").
#[surrealism]
fn format_e164(number: String, country: String) -> Result<String, String> {
	let parsed = parse_number(&number, &country)?;
	Ok(parsed.format().mode(phonenumber::Mode::E164).to_string())
}

/// Parses a phone number and returns its validity, E.164 form, calling code and national number.
#[surrealism]
fn parse(number: String, country: String) -> Result<Value, String> {
	let parsed = parse_number(&number, &country)?;
	Ok(json!({
		"valid": parsed.is_valid(),
		"e164": parsed.format().mode(phonenumber::Mode::E164).to_string(),
		"country_code": parsed.code().value(),
		"national_number": parsed.national().to_string(),
	}))
}
