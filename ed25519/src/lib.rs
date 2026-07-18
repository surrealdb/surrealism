//! Ed25519 signing and verification for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::ed25519 AS f"bucket:/ed25519.surli";`
//! and call `mod::ed25519::generate_secret_key()`,
//! `mod::ed25519::public_key($secret_key)`,
//! `mod::ed25519::sign($secret_key, $message)`,
//! `mod::ed25519::verify($public_key, $message, $signature)`, etc.

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey, Verifier};
use rand_core::OsRng;
use surrealdb_types::Bytes;
use surrealism::surrealism;

fn signing_key_from_seed(secret_key: &Bytes) -> Result<SigningKey, String> {
	let seed: [u8; 32] =
		secret_key.as_ref().try_into().map_err(|_| "secret_key must be 32 bytes".to_string())?;
	Ok(SigningKey::from_bytes(&seed))
}

/// Generates a new random 32-byte ed25519 secret key seed.
#[surrealism]
fn generate_secret_key() -> Bytes {
	let signing_key = SigningKey::generate(&mut OsRng);
	Bytes::from(signing_key.to_bytes().to_vec())
}

/// Derives the 32-byte public key from a 32-byte secret key seed.
#[surrealism]
fn public_key(secret_key: Bytes) -> Result<Bytes, String> {
	let signing_key = signing_key_from_seed(&secret_key)?;
	Ok(Bytes::from(signing_key.verifying_key().to_bytes().to_vec()))
}

/// Signs `message` with the 32-byte secret key seed, returning a 64-byte signature.
#[surrealism]
fn sign(secret_key: Bytes, message: Bytes) -> Result<Bytes, String> {
	let signing_key = signing_key_from_seed(&secret_key)?;
	let signature = signing_key.sign(message.as_ref());
	Ok(Bytes::from(signature.to_bytes().to_vec()))
}

/// Verifies `signature` over `message` against a 32-byte public key.
#[surrealism]
fn verify(public_key: Bytes, message: Bytes, signature: Bytes) -> Result<bool, String> {
	let key_bytes: [u8; 32] =
		public_key.as_ref().try_into().map_err(|_| "public_key must be 32 bytes".to_string())?;
	let verifying_key = VerifyingKey::from_bytes(&key_bytes).map_err(|e| e.to_string())?;
	let sig_bytes: [u8; 64] =
		signature.as_ref().try_into().map_err(|_| "signature must be 64 bytes".to_string())?;
	let signature = Signature::from_bytes(&sig_bytes);
	Ok(verifying_key.verify(message.as_ref(), &signature).is_ok())
}
