//! CSV parsing and writing functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::csv AS f"bucket:/csv.surli";`
//! and call `mod::csv::parse(",...")`, `mod::csv::parse_objects("...")`,
//! `mod::csv::stringify([...])`, etc.

use serde_json::Value;
use surrealism::surrealism;

/// Parses comma-delimited CSV into rows of raw string cells; no row is
/// treated as a header.
#[surrealism]
fn parse(input: String) -> Result<Vec<Vec<String>>, String> {
	let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(input.as_bytes());
	reader
		.records()
		.map(|record| {
			record.map(|r| r.iter().map(String::from).collect()).map_err(|e| e.to_string())
		})
		.collect()
}

/// Parses CSV where the first row is a header row; each subsequent row
/// becomes an object mapping header to cell value (all values as strings).
/// Errors if a row has a different number of fields than the header.
#[surrealism]
fn parse_objects(input: String) -> Result<Vec<Value>, String> {
	let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(input.as_bytes());
	let headers = reader.headers().map_err(|e| e.to_string())?.clone();
	reader
		.records()
		.map(|record| {
			let record = record.map_err(|e| e.to_string())?;
			if record.len() != headers.len() {
				return Err(format!(
					"row has {} fields, expected {} (from header row)",
					record.len(),
					headers.len()
				));
			}
			let object: serde_json::Map<String, Value> = headers
				.iter()
				.zip(record.iter())
				.map(|(key, cell)| (key.to_string(), Value::String(cell.to_string())))
				.collect();
			Ok(Value::Object(object))
		})
		.collect()
}

/// Writes rows back out as comma-delimited CSV text, quoting fields as
/// needed.
#[surrealism]
fn stringify(rows: Vec<Vec<String>>) -> Result<String, String> {
	let mut writer = csv::Writer::from_writer(Vec::new());
	for row in &rows {
		writer.write_record(row).map_err(|e| e.to_string())?;
	}
	let bytes = writer.into_inner().map_err(|e| e.to_string())?;
	String::from_utf8(bytes).map_err(|e| e.to_string())
}
