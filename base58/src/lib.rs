//! Base58 and Base58Check encoding for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::base58 AS f"bucket:/base58.surli";`
//! and call `mod::base58::encode(data)`, `mod::base58::decode("...")`,
//! `mod::base58::encode_check(data)`, `mod::base58::decode_check("...")`, etc.

use surrealdb_types::Bytes;
use surrealism::surrealism;

/// Encodes `data` as a Base58 string.
#[surrealism]
fn encode(data: Bytes) -> String {
	bs58::encode(&*data).into_string()
}

/// Decodes a Base58 string back into bytes.
#[surrealism]
fn decode(data: String) -> Result<Bytes, String> {
	bs58::decode(&data).into_vec().map(Bytes::from).map_err(|e| e.to_string())
}

/// Encodes `data` as a Base58Check string, appending a 4-byte checksum.
#[surrealism]
fn encode_check(data: Bytes) -> String {
	bs58::encode(&*data).with_check().into_string()
}

/// Decodes a Base58Check string, verifying and stripping its checksum.
#[surrealism]
fn decode_check(data: String) -> Result<Bytes, String> {
	bs58::decode(&data).with_check(None).into_vec().map(Bytes::from).map_err(|e| e.to_string())
}
