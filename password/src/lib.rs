//! Secure password hashing and verification (Argon2) for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::password AS f"bucket:/password.surli";`
//! and call `mod::password::hash("hunter2")`, `mod::password::verify("hunter2", hash)`.

use argon2::Argon2;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use surrealism::surrealism;

/// Hashes `password` with Argon2 (default params) and returns a self-contained
/// PHC-format string (includes algorithm, salt, and params).
#[surrealism]
fn hash(password: String) -> Result<String, String> {
	let salt = SaltString::generate(&mut OsRng);
	let hash =
		Argon2::default().hash_password(password.as_bytes(), &salt).map_err(|e| e.to_string())?;
	Ok(hash.to_string())
}

/// Verifies `password` against a stored PHC hash string. Returns `Ok(false)`
/// on mismatch; only a malformed `hash` string is an `Err`.
#[surrealism]
fn verify(password: String, hash: String) -> Result<bool, String> {
	let parsed = PasswordHash::new(&hash).map_err(|e| e.to_string())?;
	Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
}
