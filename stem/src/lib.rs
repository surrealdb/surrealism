//! Word stemming (Snowball algorithms) for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::stem AS f"bucket:/stem.surli";` and
//! call `mod::stem::stem("running", "english")`,
//! `mod::stem::stem_words(["running", "jumps"], "english")`, etc.
//! `language` is a lowercase language name: arabic, danish, dutch, english,
//! finnish, french, german, greek, hungarian, italian, norwegian,
//! portuguese, romanian, russian, spanish, swedish, tamil, turkish.

use rust_stemmers::{Algorithm, Stemmer};
use surrealism::surrealism;

/// Maps a lowercase language name to its Snowball algorithm.
fn algorithm(language: &str) -> Result<Algorithm, String> {
	match language {
		"arabic" => Ok(Algorithm::Arabic),
		"danish" => Ok(Algorithm::Danish),
		"dutch" => Ok(Algorithm::Dutch),
		"english" => Ok(Algorithm::English),
		"finnish" => Ok(Algorithm::Finnish),
		"french" => Ok(Algorithm::French),
		"german" => Ok(Algorithm::German),
		"greek" => Ok(Algorithm::Greek),
		"hungarian" => Ok(Algorithm::Hungarian),
		"italian" => Ok(Algorithm::Italian),
		"norwegian" => Ok(Algorithm::Norwegian),
		"portuguese" => Ok(Algorithm::Portuguese),
		"romanian" => Ok(Algorithm::Romanian),
		"russian" => Ok(Algorithm::Russian),
		"spanish" => Ok(Algorithm::Spanish),
		"swedish" => Ok(Algorithm::Swedish),
		"tamil" => Ok(Algorithm::Tamil),
		"turkish" => Ok(Algorithm::Turkish),
		other => Err(format!("Unrecognized language: {other}")),
	}
}

/// Stems a single word using the Snowball algorithm for the given language.
#[surrealism]
fn stem(word: String, language: String) -> Result<String, String> {
	let stemmer = Stemmer::create(algorithm(&language)?);
	Ok(stemmer.stem(&word).into_owned())
}

/// Stems a list of words using the Snowball algorithm for the given language.
#[surrealism]
fn stem_words(words: Vec<String>, language: String) -> Result<Vec<String>, String> {
	let stemmer = Stemmer::create(algorithm(&language)?);
	Ok(words.iter().map(|word| stemmer.stem(word).into_owned()).collect())
}
