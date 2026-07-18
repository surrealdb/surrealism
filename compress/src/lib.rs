//! Gzip compression functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::compress AS f"bucket:/compress.surli";`
//! and call `mod::compress::gzip(data)`, `mod::compress::gunzip(data)`, etc.

use std::io::{Read, Write};

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use surrealdb_types::Bytes;
use surrealism::surrealism;

/// Gzip-compresses `input` using the default compression level.
#[surrealism]
fn gzip(input: Bytes) -> Result<Bytes, String> {
	let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
	encoder.write_all(&input).map_err(|e| e.to_string())?;
	let compressed = encoder.finish().map_err(|e| e.to_string())?;
	Ok(Bytes::from(compressed))
}

/// Gzip-decompresses `input`.
#[surrealism]
fn gunzip(input: Bytes) -> Result<Bytes, String> {
	let mut decoder = GzDecoder::new(&input[..]);
	let mut decompressed = Vec::new();
	decoder.read_to_end(&mut decompressed).map_err(|e| e.to_string())?;
	Ok(Bytes::from(decompressed))
}
