//! Linear barcode generation for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::barcode AS f"bucket:/barcode.surli";`
//! and call `mod::barcode::code128("HELLO123")` or
//! `mod::barcode::ean13("5901234123457")`. Both return PNG bytes.

use barcoders::generators::image::Image;
use barcoders::sym::code128::Code128;
use barcoders::sym::ean13::EAN13;
use surrealdb_types::Bytes;
use surrealism::surrealism;

const HEIGHT: u32 = 80;

/// Encodes `data` as a Code128 barcode (character-set B) and renders it to PNG bytes.
#[surrealism]
fn code128(data: String) -> Result<Bytes, String> {
	let prefixed = format!("\u{0181}{data}");
	let barcode = Code128::new(prefixed).map_err(|e| e.to_string())?;
	let encoded = barcode.encode();
	let png = Image::png(HEIGHT).generate(encoded).map_err(|e| e.to_string())?;
	Ok(Bytes::from(png))
}

/// Encodes `data` (12 or 13 ASCII digits) as an EAN-13 barcode and renders it to PNG bytes.
#[surrealism]
fn ean13(data: String) -> Result<Bytes, String> {
	let barcode = EAN13::new(data).map_err(|e| e.to_string())?;
	let encoded = barcode.encode();
	let png = Image::png(HEIGHT).generate(encoded).map_err(|e| e.to_string())?;
	Ok(Bytes::from(png))
}
