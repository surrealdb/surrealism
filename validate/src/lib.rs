//! Format validation functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::validate AS f"bucket:/validate.surli";`
//! and call `mod::validate::email("a@b.com")`, `mod::validate::iban("GB...")`, etc.
//!
//! These are lightweight, dependency-light format checks (not full RFC
//! parsers) intended for everyday input validation.

use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;
use surrealism::surrealism;

static EMAIL_RE: LazyLock<Regex> = LazyLock::new(|| {
	Regex::new(
		r"(?i)^[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)+$",
	)
	.expect("valid regex")
});

/// Loose E.164-shaped check: optional leading `+`, 7-15 digits, no leading
/// zero. Not a substitute for full phone-number-metadata validation.
static PHONE_RE: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"^\+?[1-9]\d{6,14}$").expect("valid regex"));

static UUID_RE: LazyLock<Regex> = LazyLock::new(|| {
	Regex::new(r"(?i)^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$")
		.expect("valid regex")
});

static HEX_COLOR_RE: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"(?i)^#?([0-9a-f]{3}|[0-9a-f]{6})$").expect("valid regex"));

/// Checks whether `input` looks like a valid email address.
#[surrealism]
fn email(input: String) -> bool {
	EMAIL_RE.is_match(&input)
}

/// Checks whether `input` is a valid `http(s)://` URL.
#[surrealism]
fn url(input: String) -> bool {
	::url::Url::parse(&input).is_ok_and(|u| matches!(u.scheme(), "http" | "https"))
}

/// Checks whether `input` is a valid IPv4 address.
#[surrealism]
fn ipv4(input: String) -> bool {
	Ipv4Addr::from_str(&input).is_ok()
}

/// Checks whether `input` is a valid IPv6 address.
#[surrealism]
fn ipv6(input: String) -> bool {
	Ipv6Addr::from_str(&input).is_ok()
}

fn luhn_valid(input: &str) -> bool {
	let cleaned: String = input.chars().filter(|c| !c.is_whitespace() && *c != '-').collect();
	if cleaned.len() < 8 || !cleaned.chars().all(|c| c.is_ascii_digit()) {
		return false;
	}
	let sum: u32 = cleaned
		.chars()
		.rev()
		.enumerate()
		.map(|(i, c)| {
			let d = c.to_digit(10).expect("already validated as ascii digit");
			if i % 2 == 1 {
				let doubled = d * 2;
				if doubled > 9 {
					doubled - 9
				} else {
					doubled
				}
			} else {
				d
			}
		})
		.sum();
	sum.is_multiple_of(10)
}

/// Checks whether `input` is a Luhn-valid credit card number.
#[surrealism]
fn credit_card(input: String) -> bool {
	luhn_valid(&input)
}

fn iban_valid(input: &str) -> bool {
	let cleaned: String =
		input.chars().filter(|c| !c.is_whitespace()).map(|c| c.to_ascii_uppercase()).collect();
	if cleaned.len() < 15
		|| cleaned.len() > 34
		|| !cleaned.chars().all(|c| c.is_ascii_alphanumeric())
	{
		return false;
	}
	if !cleaned[..2].chars().all(|c| c.is_ascii_alphabetic()) {
		return false;
	}
	let rearranged = format!("{}{}", &cleaned[4..], &cleaned[..4]);
	let mut numeric = String::with_capacity(rearranged.len() * 2);
	for c in rearranged.chars() {
		if c.is_ascii_digit() {
			numeric.push(c);
		} else {
			numeric.push_str(&(c as u32 - 'A' as u32 + 10).to_string());
		}
	}
	let remainder =
		numeric.chars().fold(0u32, |acc, c| (acc * 10 + c.to_digit(10).expect("digit")) % 97);
	remainder == 1
}

/// Checks whether `input` is a structurally valid IBAN (ISO 7064 mod-97 check).
#[surrealism]
fn iban(input: String) -> bool {
	iban_valid(&input)
}

/// Checks whether `input` is a valid UUID (any version).
#[surrealism]
fn uuid(input: String) -> bool {
	UUID_RE.is_match(&input)
}

/// Loose phone number format check (E.164-shaped). Not full
/// libphonenumber-grade validation.
#[surrealism]
fn phone(input: String) -> bool {
	PHONE_RE.is_match(input.trim())
}

/// Checks whether `input` is a valid `#rgb` or `#rrggbb` hex color.
#[surrealism]
fn hex_color(input: String) -> bool {
	HEX_COLOR_RE.is_match(&input)
}
