//! QR code generation functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::qrcode AS f"bucket:/qrcode.surli";`
//! and call `mod::qrcode::generate(data)` or `mod::qrcode::generate_svg(data)`.

use std::io::Cursor;

use image::{ImageFormat, Luma};
use qrcode::QrCode;
use qrcode::render::svg;
use surrealdb_types::Bytes;
use surrealism::surrealism;

/// Encodes `data` as a QR code and renders it to PNG bytes.
#[surrealism]
fn generate(data: String) -> Result<Bytes, String> {
	let code = QrCode::new(data.as_bytes()).map_err(|e| e.to_string())?;
	let image = code.render::<Luma<u8>>().build();
	let mut buf = Cursor::new(Vec::new());
	image.write_to(&mut buf, ImageFormat::Png).map_err(|e| e.to_string())?;
	Ok(Bytes::from(buf.into_inner()))
}

/// Encodes `data` as a QR code and renders it as an SVG string.
#[surrealism]
fn generate_svg(data: String) -> Result<String, String> {
	let code = QrCode::new(data.as_bytes()).map_err(|e| e.to_string())?;
	Ok(code
		.render()
		.min_dimensions(200, 200)
		.dark_color(svg::Color("#000000"))
		.light_color(svg::Color("#ffffff"))
		.build())
}
