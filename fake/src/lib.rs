//! Realistic fake data generation functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE fake AS f"bucket:/fake.surli";` and call
//! `fake::name()`, `fake::address()`, `fake::email()`, etc.

use fake::Fake;
use fake::faker::address::en::{
	CityName, CountryName, Latitude, Longitude, StateName, StreetName, ZipCode,
};
use fake::faker::company::en::{CompanyName, Industry, Profession};
use fake::faker::internet::en::{FreeEmail, IPv4, IPv6, Password, UserAgent, Username};
use fake::faker::lorem::en::{Paragraph, Sentence, Word};
use fake::faker::name::en::{FirstName, LastName, Name};
use fake::faker::phone_number::en::PhoneNumber;
use rand::Rng;
use surrealism::surrealism;

/// A random full name.
#[surrealism]
fn name() -> String {
	Name().fake()
}

/// A random first name.
#[surrealism]
fn first_name() -> String {
	FirstName().fake()
}

/// A random last name.
#[surrealism]
fn last_name() -> String {
	LastName().fake()
}

/// A random email address.
#[surrealism]
fn email() -> String {
	FreeEmail().fake()
}

/// A random username.
#[surrealism]
fn username() -> String {
	Username().fake()
}

/// A random password of exactly `length` characters.
#[surrealism]
fn password(length: i64) -> Result<String, String> {
	let length = usize::try_from(length)
		.map_err(|_| "length must not be negative".to_string())?
		.max(1);
	Ok(Password(length..length + 1).fake())
}

/// A random phone number.
#[surrealism]
fn phone_number() -> String {
	PhoneNumber().fake()
}

/// A random company name.
#[surrealism]
fn company() -> String {
	CompanyName().fake()
}

/// A random job title.
#[surrealism]
fn job_title() -> String {
	Profession().fake()
}

/// A random industry name.
#[surrealism]
fn industry() -> String {
	Industry().fake()
}

/// A random street name.
#[surrealism]
fn street_address() -> String {
	StreetName().fake()
}

/// A random city name.
#[surrealism]
fn city() -> String {
	CityName().fake()
}

/// A random state/province name.
#[surrealism]
fn state() -> String {
	StateName().fake()
}

/// A random country name.
#[surrealism]
fn country() -> String {
	CountryName().fake()
}

/// A random postal/zip code.
#[surrealism]
fn zip_code() -> String {
	ZipCode().fake()
}

/// A random latitude, in `[-90, 90]`.
#[surrealism]
fn latitude() -> f64 {
	Latitude().fake()
}

/// A random longitude, in `[-180, 180]`.
#[surrealism]
fn longitude() -> f64 {
	Longitude().fake()
}

/// A single random word.
#[surrealism]
fn word() -> String {
	Word().fake()
}

/// A random sentence of exactly `word_count` words.
#[surrealism]
fn sentence(word_count: i64) -> Result<String, String> {
	let n = usize::try_from(word_count)
		.map_err(|_| "word_count must not be negative".to_string())?
		.max(1);
	Ok(Sentence(n..n + 1).fake())
}

/// A random paragraph of exactly `sentence_count` sentences.
#[surrealism]
fn paragraph(sentence_count: i64) -> Result<String, String> {
	let n = usize::try_from(sentence_count)
		.map_err(|_| "sentence_count must not be negative".to_string())?
		.max(1);
	Ok(Paragraph(n..n + 1).fake())
}

/// A random UUID (version 4).
#[surrealism]
fn uuid() -> String {
	uuid::Uuid::new_v4().to_string()
}

/// A random IPv4 address.
#[surrealism]
fn ipv4() -> String {
	IPv4().fake()
}

/// A random IPv6 address.
#[surrealism]
fn ipv6() -> String {
	IPv6().fake()
}

/// A random browser user-agent string.
#[surrealism]
fn user_agent() -> String {
	UserAgent().fake()
}

/// A random `#rrggbb` hex color.
#[surrealism]
fn color_hex() -> String {
	let mut rng = rand::rng();
	format!("#{:02x}{:02x}{:02x}", rng.random::<u8>(), rng.random::<u8>(), rng.random::<u8>())
}

/// A random boolean.
#[surrealism]
fn boolean() -> bool {
	rand::rng().random_bool(0.5)
}

/// A random integer in the inclusive range `[min, max]`.
#[surrealism]
fn number(min: i64, max: i64) -> Result<i64, String> {
	if min > max {
		return Err(format!("min ({min}) must be <= max ({max})"));
	}
	Ok(rand::rng().random_range(min..=max))
}

/// A random `YYYY-MM-DD` date between 1970 and 2024.
#[surrealism]
fn date() -> String {
	let mut rng = rand::rng();
	let year = rng.random_range(1970..=2024);
	let month = rng.random_range(1..=12);
	let day = rng.random_range(1..=28);
	format!("{year:04}-{month:02}-{day:02}")
}

fn luhn_check_digit(digits: &[u32]) -> u32 {
	let sum: u32 = digits
		.iter()
		.rev()
		.enumerate()
		.map(|(i, &d)| {
			if i % 2 == 0 {
				let doubled = d * 2;
				if doubled > 9 { doubled - 9 } else { doubled }
			} else {
				d
			}
		})
		.sum();
	(10 - (sum % 10)) % 10
}

/// A random 16-digit number that passes the Luhn checksum (not a real,
/// issuable card number).
#[surrealism]
fn credit_card_number() -> String {
	let mut rng = rand::rng();
	let mut digits: Vec<u32> = (0..15).map(|_| rng.random_range(0..=9)).collect();
	digits.push(luhn_check_digit(&digits));
	digits.iter().map(u32::to_string).collect()
}
