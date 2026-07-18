//! ZIP archive creation/extraction for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::zip AS f"bucket:/zip.surli";` and
//! call `mod::zip::create({"a.txt": "hello", "b.bin": $bytes})`,
//! `mod::zip::extract($archive)`, etc.

use std::io::{Cursor, Read, Write};

use surrealdb_types::{Bytes, Object, Value};
use surrealism::surrealism;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

fn value_to_bytes(value: Value) -> Result<Vec<u8>, String> {
	match value {
		Value::String(s) => Ok(s.into_bytes()),
		Value::Bytes(b) => Ok(b.into_inner().to_vec()),
		other => Err(format!("Unsupported file content type: {other:?}")),
	}
}

/// Builds a ZIP archive from an object mapping filename to file content
/// (a string or `bytes` value).
#[surrealism]
fn create(files: Value) -> Result<Bytes, String> {
	let Value::Object(files) = files else {
		return Err("files must be an object mapping filename to content".to_string());
	};

	let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
	for (name, value) in files {
		let data = value_to_bytes(value)?;
		writer.start_file(name, SimpleFileOptions::default()).map_err(|e| e.to_string())?;
		writer.write_all(&data).map_err(|e| e.to_string())?;
	}
	let cursor = writer.finish().map_err(|e| e.to_string())?;
	Ok(Bytes::from(cursor.into_inner()))
}

/// Extracts a ZIP archive into an object mapping each entry's filename to
/// its content as `bytes`.
#[surrealism]
fn extract(archive: Bytes) -> Result<Value, String> {
	let data = archive.into_inner();
	let mut zip = ZipArchive::new(Cursor::new(&*data)).map_err(|e| e.to_string())?;

	let mut files = Object::new();
	for i in 0..zip.len() {
		let mut entry = zip.by_index(i).map_err(|e| e.to_string())?;
		let name = entry.name().to_string();
		let mut contents = Vec::new();
		entry.read_to_end(&mut contents).map_err(|e| e.to_string())?;
		files.insert(name, Bytes::from(contents));
	}
	Ok(Value::Object(files))
}
