//! Natural language detection for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::lang AS f"bucket:/lang.surli";`
//! and call `mod::lang::detect("This is an English sentence.")`,
//! `mod::lang::code("This is an English sentence.")`, etc.

use serde_json::{Value, json};
use surrealism::surrealism;

/// Detects the language of a text, returning an object with `lang` (the
/// ISO 639-3 code), `confidence` (0.0 to 1.0), and `is_reliable`; errors if
/// the text is too short or ambiguous to detect.
#[surrealism]
fn detect(text: String) -> Result<Value, String> {
	match whatlang::detect(&text) {
		Some(info) => Ok(json!({
			"lang": info.lang().code(),
			"confidence": info.confidence(),
			"is_reliable": info.is_reliable(),
		})),
		None => Err("could not detect language".to_string()),
	}
}

/// Detects the ISO 639-3 language code of a text, or returns `none` if the
/// text is too short or ambiguous to detect.
#[surrealism]
fn code(text: String) -> Option<String> {
	whatlang::detect(&text).map(|info| info.lang().code().to_string())
}
