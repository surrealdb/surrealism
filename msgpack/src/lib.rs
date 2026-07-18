//! MessagePack binary serialization for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::msgpack AS f"bucket:/msgpack.surli";`
//! and call `mod::msgpack::encode({name: "Ferris", tags: ["a", "b"]})`,
//! `mod::msgpack::decode($bytes)`, etc.

use serde_json::Value;
use surrealdb_types::Bytes;
use surrealism::surrealism;

/// Serializes a value into MessagePack-encoded bytes.
#[surrealism]
fn encode(value: Value) -> Result<Bytes, String> {
	let bytes = rmp_serde::to_vec(&value).map_err(|e| e.to_string())?;
	Ok(Bytes::from(bytes))
}

/// Deserializes MessagePack-encoded bytes back into a value.
#[surrealism]
fn decode(data: Bytes) -> Result<Value, String> {
	rmp_serde::from_slice::<Value>(&data).map_err(|e| e.to_string())
}
