//! String and text manipulation functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE text AS f"bucket:/text.surli";` and call
//! `text::slugify("Hello, World!")`, `text::snake_case("myVarName")`, etc.

use std::sync::LazyLock;

use deunicode::deunicode;
use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase, ToTitleCase, ToUpperCamelCase};
use regex::Regex;
use surrealism::surrealism;

/// Converts `input` into a URL-friendly slug: transliterated to ASCII,
/// lowercased, with runs of non-alphanumeric characters collapsed to a
/// single `-`.
#[surrealism]
fn slugify(input: String) -> String {
	let ascii = deunicode(&input);
	let mut slug = String::with_capacity(ascii.len());
	let mut last_was_dash = true;
	for c in ascii.chars() {
		if c.is_ascii_alphanumeric() {
			slug.push(c.to_ascii_lowercase());
			last_was_dash = false;
		} else if !last_was_dash {
			slug.push('-');
			last_was_dash = true;
		}
	}
	slug.trim_end_matches('-').to_string()
}

/// Truncates `input` to at most `max_len` characters (not bytes).
#[surrealism]
fn truncate(input: String, max_len: i64) -> Result<String, String> {
	let max_len =
		usize::try_from(max_len).map_err(|_| "max_len must not be negative".to_string())?;
	Ok(input.chars().take(max_len).collect())
}

/// Converts `input` to `snake_case`.
#[surrealism]
fn snake_case(input: String) -> String {
	input.to_snake_case()
}

/// Converts `input` to `camelCase`.
#[surrealism]
fn camel_case(input: String) -> String {
	input.to_lower_camel_case()
}

/// Converts `input` to `kebab-case`.
#[surrealism]
fn kebab_case(input: String) -> String {
	input.to_kebab_case()
}

/// Converts `input` to `PascalCase`.
#[surrealism]
fn pascal_case(input: String) -> String {
	input.to_upper_camel_case()
}

/// Converts `input` to Title Case (each word capitalized).
#[surrealism]
fn title_case(input: String) -> String {
	input.to_title_case()
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
	let a: Vec<char> = a.chars().collect();
	let b: Vec<char> = b.chars().collect();
	let mut prev: Vec<usize> = (0..=b.len()).collect();
	let mut curr = vec![0usize; b.len() + 1];
	for (i, &ca) in a.iter().enumerate() {
		curr[0] = i + 1;
		for (j, &cb) in b.iter().enumerate() {
			let cost = usize::from(ca != cb);
			curr[j + 1] = (prev[j + 1] + 1).min(curr[j] + 1).min(prev[j] + cost);
		}
		std::mem::swap(&mut prev, &mut curr);
	}
	prev[b.len()]
}

/// Levenshtein (edit) distance between `a` and `b`.
#[surrealism]
fn levenshtein(a: String, b: String) -> i64 {
	levenshtein_distance(&a, &b) as i64
}

/// Counts whitespace-separated words in `input`.
#[surrealism]
fn word_count(input: String) -> i64 {
	input.split_whitespace().count() as i64
}

static HTML_TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<[^>]*>").expect("valid regex"));

/// Strips HTML tags from `input`, leaving the text content.
#[surrealism]
fn strip_html(input: String) -> String {
	HTML_TAG.replace_all(&input, "").into_owned()
}

/// Uppercases the first character of `input`, leaving the rest untouched.
#[surrealism]
fn capitalize(input: String) -> String {
	let mut chars = input.chars();
	match chars.next() {
		Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
		None => String::new(),
	}
}

/// Reverses `input` by character (not byte).
#[surrealism]
fn reverse(input: String) -> String {
	input.chars().rev().collect()
}
