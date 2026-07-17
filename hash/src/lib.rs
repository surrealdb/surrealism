//! Hashing and HMAC functions for Surrealism, plus `base64`/`hex` encoding
//! submodules.
//!
//! Register with e.g. `DEFINE MODULE mod::hash AS f"bucket:/hash.surli";`
//! and call `mod::hash::sha256("hello")`, `mod::hash::base64::encode("hi")`,
//! `mod::hash::hex::decode("...")`, etc.

use hmac::{Hmac, Mac};
use md5::Md5;
use sha2::{Digest, Sha256, Sha512};
use surrealism::surrealism;

/// SHA-256 digest of `input`, as a lowercase hex string.
#[surrealism]
fn sha256(input: String) -> String {
	::hex::encode(Sha256::digest(input.as_bytes()))
}

/// SHA-512 digest of `input`, as a lowercase hex string.
#[surrealism]
fn sha512(input: String) -> String {
	::hex::encode(Sha512::digest(input.as_bytes()))
}

/// MD5 digest of `input`, as a lowercase hex string. Not cryptographically
/// secure; use for checksums, not security.
#[surrealism]
fn md5(input: String) -> String {
	::hex::encode(Md5::digest(input.as_bytes()))
}

/// HMAC-SHA256 of `input` keyed by `key`, as a lowercase hex string.
#[surrealism]
fn hmac_sha256(input: String, key: String) -> String {
	let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(key.as_bytes())
		.expect("HMAC accepts a key of any length");
	mac.update(input.as_bytes());
	::hex::encode(mac.finalize().into_bytes())
}

#[surrealism]
mod base64 {
	use base64::Engine as _;
	use base64::engine::general_purpose::STANDARD;

	/// Encodes `input` as standard base64.
	#[surrealism]
	fn encode(input: String) -> String {
		STANDARD.encode(input.as_bytes())
	}

	/// Decodes a standard base64 string back into UTF-8 text.
	#[surrealism]
	fn decode(input: String) -> Result<String, String> {
		let bytes = STANDARD.decode(input.as_bytes()).map_err(|e| e.to_string())?;
		String::from_utf8(bytes).map_err(|e| e.to_string())
	}
}

#[surrealism]
mod hex {
	/// Encodes `input` as lowercase hex.
	#[surrealism]
	fn encode(input: String) -> String {
		::hex::encode(input.as_bytes())
	}

	/// Decodes a hex string back into UTF-8 text.
	#[surrealism]
	fn decode(input: String) -> Result<String, String> {
		let bytes = ::hex::decode(&input).map_err(|e| e.to_string())?;
		String::from_utf8(bytes).map_err(|e| e.to_string())
	}
}
