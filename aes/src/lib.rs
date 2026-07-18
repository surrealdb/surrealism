//! AES-256-GCM authenticated symmetric encryption for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::aes AS f"bucket:/aes.surli";` and
//! call `mod::aes::generate_key()`, `mod::aes::encrypt($key, $plaintext)`,
//! `mod::aes::decrypt($key, $data)`, etc.

use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, Nonce};
use surrealdb_types::Bytes;
use surrealism::surrealism;

const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12;

/// Generates a random 32-byte AES-256 key.
#[surrealism]
fn generate_key() -> Bytes {
	let key = Aes256Gcm::generate_key(&mut OsRng);
	Bytes::from(key.to_vec())
}

/// Encrypts `plaintext` with a 32-byte `key`, returning `nonce || ciphertext`.
#[surrealism]
fn encrypt(key: Bytes, plaintext: Bytes) -> Result<Bytes, String> {
	if key.len() != KEY_LEN {
		return Err(format!("Key must be {KEY_LEN} bytes, got {}", key.len()));
	}
	let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
	let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
	let ciphertext =
		cipher.encrypt(&nonce, plaintext.as_ref()).map_err(|e| e.to_string())?;
	let mut out = nonce.to_vec();
	out.extend_from_slice(&ciphertext);
	Ok(Bytes::from(out))
}

/// Decrypts `nonce || ciphertext` data with the 32-byte `key` that encrypted it.
#[surrealism]
fn decrypt(key: Bytes, data: Bytes) -> Result<Bytes, String> {
	if key.len() != KEY_LEN {
		return Err(format!("Key must be {KEY_LEN} bytes, got {}", key.len()));
	}
	if data.len() <= NONCE_LEN {
		return Err(format!("Data must be longer than {NONCE_LEN} bytes"));
	}
	let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
	let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
	let nonce = Nonce::from_slice(nonce_bytes);
	let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| e.to_string())?;
	Ok(Bytes::from(plaintext))
}
