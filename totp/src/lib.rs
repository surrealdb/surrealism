//! TOTP (RFC 6238) two-factor authentication codes for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::totp AS f"bucket:/totp.surli";` and
//! call `mod::totp::generate("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ")`,
//! `mod::totp::verify("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ", "123456")`, etc.
//! The secret must base32-decode to at least 128 bits (16 bytes).

use surrealism::surrealism;
use totp_rs::{Algorithm, Secret, TOTP};

/// Builds a standard Google-Authenticator-compatible TOTP (SHA1, 6 digits,
/// 30-second step, 1-step skew) from a base32-encoded secret.
fn build_totp(secret_base32: &str) -> Result<TOTP, String> {
	let secret = Secret::Encoded(secret_base32.to_string())
		.to_bytes()
		.map_err(|e| format!("Invalid base32 secret: {e}"))?;
	TOTP::new(Algorithm::SHA1, 6, 1, 30, secret).map_err(|e| e.to_string())
}

/// Generates the current 6-digit TOTP code for a base32-encoded secret.
#[surrealism]
fn generate(secret_base32: String) -> Result<String, String> {
	let totp = build_totp(&secret_base32)?;
	totp.generate_current().map_err(|e| e.to_string())
}

/// Checks whether a code is valid for a base32-encoded secret at the current
/// time, allowing the configured skew.
#[surrealism]
fn verify(secret_base32: String, code: String) -> Result<bool, String> {
	let totp = build_totp(&secret_base32)?;
	totp.check_current(&code).map_err(|e| e.to_string())
}
