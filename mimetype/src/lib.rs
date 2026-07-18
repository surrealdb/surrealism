//! File type detection functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::mimetype AS f"bucket:/mimetype.surli";`
//! and call `mod::mimetype::detect(data)`, `mod::mimetype::extension(data)`.

use surrealdb_types::Bytes;
use surrealism::surrealism;

/// Detects the MIME type from magic bytes, e.g. `"image/png"`, or returns
/// `none` if the type is unrecognized.
#[surrealism]
fn detect(data: Bytes) -> Option<String> {
	infer::get(&data).map(|t| t.mime_type().to_string())
}

/// Detects the file extension from magic bytes, e.g. `"png"`, or returns
/// `none` if the type is unrecognized.
#[surrealism]
fn extension(data: Bytes) -> Option<String> {
	infer::get(&data).map(|t| t.extension().to_string())
}
