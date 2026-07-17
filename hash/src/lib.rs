//! Hashing, HMAC, and encoding functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE hash AS f"bucket:/hash.surli";` and call
//! `hash::sha256("hello")`, `hash::uuid_v4()`, etc.

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use hmac::{Hmac, Mac};
use md5::Md5;
use sha2::{Digest, Sha256, Sha512};
use surrealism::surrealism;

/// SHA-256 digest of `input`, as a lowercase hex string.
#[surrealism]
fn sha256(input: String) -> String {
	hex::encode(Sha256::digest(input.as_bytes()))
}

/// SHA-512 digest of `input`, as a lowercase hex string.
#[surrealism]
fn sha512(input: String) -> String {
	hex::encode(Sha512::digest(input.as_bytes()))
}

/// MD5 digest of `input`, as a lowercase hex string.
///
/// MD5 is not cryptographically secure; use it only for checksums or
/// compatibility with legacy systems, not for security-sensitive purposes.
#[surrealism]
fn md5(input: String) -> String {
	hex::encode(Md5::digest(input.as_bytes()))
}

/// HMAC-SHA256 of `input` keyed by `key`, as a lowercase hex string.
#[surrealism]
fn hmac_sha256(input: String, key: String) -> String {
	let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(key.as_bytes())
		.expect("HMAC accepts a key of any length");
	mac.update(input.as_bytes());
	hex::encode(mac.finalize().into_bytes())
}

/// Encodes `input` as standard base64.
#[surrealism]
fn base64_encode(input: String) -> String {
	BASE64.encode(input.as_bytes())
}

/// Decodes a standard base64 string back into UTF-8 text.
#[surrealism]
fn base64_decode(input: String) -> Result<String, String> {
	let bytes = BASE64.decode(input.as_bytes()).map_err(|e| e.to_string())?;
	String::from_utf8(bytes).map_err(|e| e.to_string())
}

/// Encodes `input` as lowercase hex.
#[surrealism]
fn hex_encode(input: String) -> String {
	hex::encode(input.as_bytes())
}

/// Decodes a hex string back into UTF-8 text.
#[surrealism]
fn hex_decode(input: String) -> Result<String, String> {
	let bytes = hex::decode(&input).map_err(|e| e.to_string())?;
	String::from_utf8(bytes).map_err(|e| e.to_string())
}

/// Generates a random UUID (version 4).
#[surrealism]
fn uuid_v4() -> String {
	uuid::Uuid::new_v4().to_string()
}

/// Generates a new ULID (lexicographically sortable, timestamp-prefixed).
#[surrealism]
fn ulid() -> String {
	ulid::Ulid::new().to_string()
}
