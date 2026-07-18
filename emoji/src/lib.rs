//! Emoji shortcode and unicode conversion for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::emoji AS f"bucket:/emoji.surli";`
//! and call `mod::emoji::to_unicode(":smile:")`,
//! `mod::emoji::to_shortcode("😄")`,
//! `mod::emoji::replace_shortcodes("Hello :smile:!")`, etc.

use surrealism::surrealism;

/// Looks up a shortcode (with or without surrounding colons) and returns its
/// unicode emoji, or `none` if not found.
#[surrealism]
fn to_unicode(shortcode: String) -> Option<String> {
	let trimmed = shortcode.trim_matches(':');
	emojis::get_by_shortcode(trimmed).map(|e| e.as_str().to_string())
}

/// Looks up a unicode emoji and returns its canonical shortcode without
/// surrounding colons, or `none` if not recognized.
#[surrealism]
fn to_shortcode(emoji: String) -> Option<String> {
	emojis::get(&emoji).and_then(|e| e.shortcode()).map(|s| s.to_string())
}

/// Replaces every `:shortcode:` pattern in the text with its unicode emoji,
/// leaving unrecognized shortcodes and surrounding text untouched.
#[surrealism]
fn replace_shortcodes(text: String) -> String {
	let mut result = String::with_capacity(text.len());
	let mut rest = text.as_str();
	while let Some(start) = rest.find(':') {
		result.push_str(&rest[..start]);
		let after_start = &rest[start + 1..];
		let found = after_start.find(':').and_then(|end| {
			let candidate = &after_start[..end];
			let valid = !candidate.is_empty()
				&& candidate.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '+' || c == '-');
			valid.then(|| emojis::get_by_shortcode(candidate)).flatten().map(|e| (e.as_str(), end))
		});
		match found {
			Some((unicode, end)) => {
				result.push_str(unicode);
				rest = &after_start[end + 1..];
			}
			None => {
				result.push(':');
				rest = after_start;
			}
		}
	}
	result.push_str(rest);
	result
}
