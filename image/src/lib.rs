//! Image manipulation functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE image AS f"bucket:/image.surli";` and
//! call `image::resize(data, 200, 200)`, `image::grayscale(data)`, etc.
//! Images are passed and returned as raw `bytes`.

use std::io::Cursor;

use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat, ImageReader};
use surrealdb_types::Bytes;
use surrealism::surrealism;

fn decode_with_format(data: &Bytes) -> Result<(DynamicImage, ImageFormat)> {
	let reader = ImageReader::new(Cursor::new(data.to_vec()))
		.with_guessed_format()
		.context("Failed to detect image format")?;
	let format = reader.format().context("Could not determine image format")?;
	let img = reader.decode().context("Failed to decode image")?;
	Ok((img, format))
}

fn encode(img: &DynamicImage, format: ImageFormat) -> Result<Bytes> {
	let mut buf = Cursor::new(Vec::new());
	img.write_to(&mut buf, format).context("Failed to encode image")?;
	Ok(Bytes::from(buf.into_inner()))
}

fn parse_format(name: &str) -> Result<ImageFormat> {
	ImageFormat::from_extension(name)
		.ok_or_else(|| anyhow::anyhow!("Unsupported or unrecognised image format: '{name}'"))
}

fn positive_dim(v: i64) -> Result<u32> {
	let v =
		u32::try_from(v).map_err(|_| anyhow::anyhow!("dimension must be a positive integer, got {v}"))?;
	if v == 0 {
		anyhow::bail!("dimension must be greater than zero");
	}
	Ok(v)
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
	let r = f64::from(r) / 255.0;
	let g = f64::from(g) / 255.0;
	let b = f64::from(b) / 255.0;
	let max = r.max(g).max(b);
	let min = r.min(g).min(b);
	let l = (max + min) / 2.0;
	if max == min {
		return (0.0, 0.0, l);
	}
	let d = max - min;
	let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
	let mut h = if max == r {
		(g - b) / d + if g < b { 6.0 } else { 0.0 }
	} else if max == g {
		(b - r) / d + 2.0
	} else {
		(r - g) / d + 4.0
	};
	h *= 60.0;
	(h, s, l)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
	if s == 0.0 {
		let v = (l * 255.0).round() as u8;
		return (v, v, v);
	}
	let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
	let p = 2.0 * l - q;
	let h = h.rem_euclid(360.0) / 360.0;
	let hue_to_rgb = |t: f64| -> f64 {
		let t = if t < 0.0 {
			t + 1.0
		} else if t > 1.0 {
			t - 1.0
		} else {
			t
		};
		if t < 1.0 / 6.0 {
			p + (q - p) * 6.0 * t
		} else if t < 1.0 / 2.0 {
			q
		} else if t < 2.0 / 3.0 {
			p + (q - p) * (2.0 / 3.0 - t) * 6.0
		} else {
			p
		}
	};
	(
		(hue_to_rgb(h + 1.0 / 3.0) * 255.0).round() as u8,
		(hue_to_rgb(h) * 255.0).round() as u8,
		(hue_to_rgb(h - 1.0 / 3.0) * 255.0).round() as u8,
	)
}

fn adjust_saturation(img: &DynamicImage, amount: f64) -> DynamicImage {
	let mut rgba = img.to_rgba8();
	for pixel in rgba.pixels_mut() {
		let [r, g, b, _a] = pixel.0;
		let (h, s, l) = rgb_to_hsl(r, g, b);
		let (nr, ng, nb) = hsl_to_rgb(h, (s * amount).clamp(0.0, 1.0), l);
		pixel.0[0] = nr;
		pixel.0[1] = ng;
		pixel.0[2] = nb;
	}
	DynamicImage::ImageRgba8(rgba)
}

/// Resizes the image to exactly `width` x `height`, ignoring aspect ratio.
#[surrealism]
fn resize(data: Bytes, width: i64, height: i64) -> Result<Bytes> {
	let (img, format) = decode_with_format(&data)?;
	let resized =
		img.resize_exact(positive_dim(width)?, positive_dim(height)?, image::imageops::FilterType::Lanczos3);
	encode(&resized, format)
}

/// Shrinks the image to fit within `max_size` x `max_size`, preserving
/// aspect ratio.
#[surrealism]
fn thumbnail(data: Bytes, max_size: i64) -> Result<Bytes> {
	let (img, format) = decode_with_format(&data)?;
	let size = positive_dim(max_size)?;
	encode(&img.thumbnail(size, size), format)
}

/// Converts the image to grayscale (black and white).
#[surrealism]
fn grayscale(data: Bytes) -> Result<Bytes> {
	let (img, format) = decode_with_format(&data)?;
	encode(&img.grayscale(), format)
}

/// Re-encodes the image into a different format (e.g. "png", "jpeg", "gif",
/// "bmp", "ico", "tiff", "webp").
#[surrealism]
fn convert(data: Bytes, format: String) -> Result<Bytes> {
	let (img, _) = decode_with_format(&data)?;
	encode(&img, parse_format(&format)?)
}

/// Scales the image's color saturation by `amount` (1.0 = unchanged, 0.0 =
/// grayscale, >1.0 = more saturated).
#[surrealism]
fn saturate(data: Bytes, amount: f64) -> Result<Bytes> {
	let (img, format) = decode_with_format(&data)?;
	encode(&adjust_saturation(&img, amount), format)
}

/// Rotates the image clockwise by `degrees`, which must be one of 0, 90,
/// 180, or 270.
#[surrealism]
fn rotate(data: Bytes, degrees: i64) -> Result<Bytes> {
	let (img, format) = decode_with_format(&data)?;
	let rotated = match degrees.rem_euclid(360) {
		0 => img,
		90 => img.rotate90(),
		180 => img.rotate180(),
		270 => img.rotate270(),
		other => anyhow::bail!("degrees must be one of 0, 90, 180, 270, got {other}"),
	};
	encode(&rotated, format)
}

/// Flips the image horizontally (mirror left-right).
#[surrealism]
fn flip_horizontal(data: Bytes) -> Result<Bytes> {
	let (img, format) = decode_with_format(&data)?;
	encode(&img.fliph(), format)
}

/// Flips the image vertically (mirror top-bottom).
#[surrealism]
fn flip_vertical(data: Bytes) -> Result<Bytes> {
	let (img, format) = decode_with_format(&data)?;
	encode(&img.flipv(), format)
}

/// Applies a Gaussian blur with the given `sigma`.
#[surrealism]
fn blur(data: Bytes, sigma: f64) -> Result<Bytes> {
	let (img, format) = decode_with_format(&data)?;
	#[allow(clippy::cast_possible_truncation)]
	encode(&img.blur(sigma as f32), format)
}

/// Crops a `width` x `height` region starting at `(x, y)`.
#[surrealism]
fn crop(data: Bytes, x: i64, y: i64, width: i64, height: i64) -> Result<Bytes> {
	let (mut img, format) = decode_with_format(&data)?;
	let x = u32::try_from(x).map_err(|_| anyhow::anyhow!("x must not be negative"))?;
	let y = u32::try_from(y).map_err(|_| anyhow::anyhow!("y must not be negative"))?;
	let cropped = img.crop(x, y, positive_dim(width)?, positive_dim(height)?);
	encode(&cropped, format)
}

/// Inverts the image's colors.
#[surrealism]
fn invert(data: Bytes) -> Result<Bytes> {
	let (mut img, format) = decode_with_format(&data)?;
	img.invert();
	encode(&img, format)
}

/// Adjusts brightness by adding `value` to every pixel (-255..=255).
#[surrealism]
fn brightness(data: Bytes, value: i64) -> Result<Bytes> {
	let (img, format) = decode_with_format(&data)?;
	let value = i32::try_from(value).map_err(|_| anyhow::anyhow!("value out of range"))?;
	encode(&img.brighten(value), format)
}

/// Adjusts contrast by `value` (negative reduces, positive increases).
#[surrealism]
fn contrast(data: Bytes, value: f64) -> Result<Bytes> {
	let (img, format) = decode_with_format(&data)?;
	#[allow(clippy::cast_possible_truncation)]
	encode(&img.adjust_contrast(value as f32), format)
}

/// Returns the image's `(width, height)` in pixels.
#[surrealism]
fn dimensions(data: Bytes) -> Result<(i64, i64)> {
	let (img, _) = decode_with_format(&data)?;
	Ok((i64::from(img.width()), i64::from(img.height())))
}
