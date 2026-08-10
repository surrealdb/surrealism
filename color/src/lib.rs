//! Color conversion, manipulation, and accessibility functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::color AS f"bucket:/color.surli";` and call
//! `mod::color::hex_to_rgb("#3366ff")`, `mod::color::lighten("#3366ff", 10)`, etc.

use surrealism::surrealism;

/// Parses a `#rgb` or `#rrggbb` hex color (the leading `#` is optional) into
/// its red, green and blue channels.
fn parse_hex(input: &str) -> Result<(u8, u8, u8), String> {
	let s = input.trim().trim_start_matches('#');
	let expanded;
	let s = if s.len() == 3 {
		expanded = s.chars().flat_map(|c| [c, c]).collect::<String>();
		expanded.as_str()
	} else {
		s
	};
	if s.len() != 6 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
		return Err(format!("'{input}' is not a valid hex color"));
	}
	let r = u8::from_str_radix(&s[0..2], 16).map_err(|e| e.to_string())?;
	let g = u8::from_str_radix(&s[2..4], 16).map_err(|e| e.to_string())?;
	let b = u8::from_str_radix(&s[4..6], 16).map_err(|e| e.to_string())?;
	Ok((r, g, b))
}

fn format_hex(r: u8, g: u8, b: u8) -> String {
	format!("#{r:02x}{g:02x}{b:02x}")
}

fn validate_channel(name: &str, v: i64) -> Result<u8, String> {
	u8::try_from(v).map_err(|_| format!("{name} must be between 0 and 255, got {v}"))
}

/// Converts sRGB channels (0-255) to HSL, returning hue in degrees `[0, 360)`
/// and saturation/lightness as percentages `[0, 100]`.
fn rgb_to_hsl_f(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
	let r = f64::from(r) / 255.0;
	let g = f64::from(g) / 255.0;
	let b = f64::from(b) / 255.0;
	let max = r.max(g).max(b);
	let min = r.min(g).min(b);
	let l = (max + min) / 2.0;
	if max == min {
		return (0.0, 0.0, l * 100.0);
	}
	let d = max - min;
	let s = if l > 0.5 {
		d / (2.0 - max - min)
	} else {
		d / (max + min)
	};
	let mut h = if max == r {
		(g - b) / d
			+ if g < b {
				6.0
			} else {
				0.0
			}
	} else if max == g {
		(b - r) / d + 2.0
	} else {
		(r - g) / d + 4.0
	};
	h *= 60.0;
	(h, s * 100.0, l * 100.0)
}

fn hsl_to_rgb_f(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
	let h = (h.rem_euclid(360.0)) / 360.0;
	let s = (s / 100.0).clamp(0.0, 1.0);
	let l = (l / 100.0).clamp(0.0, 1.0);
	if s == 0.0 {
		let v = (l * 255.0).round() as u8;
		return (v, v, v);
	}
	let q = if l < 0.5 {
		l * (1.0 + s)
	} else {
		l + s - l * s
	};
	let p = 2.0 * l - q;
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

fn channel_luminance(c: u8) -> f64 {
	let c = f64::from(c) / 255.0;
	if c <= 0.03928 {
		c / 12.92
	} else {
		((c + 0.055) / 1.055).powf(2.4)
	}
}

/// WCAG 2.x relative luminance, in `[0, 1]`.
fn relative_luminance(r: u8, g: u8, b: u8) -> f64 {
	0.2126 * channel_luminance(r) + 0.7152 * channel_luminance(g) + 0.0722 * channel_luminance(b)
}

/// Parses a hex color into its `(r, g, b)` channels, each `0..=255`.
#[surrealism]
fn hex_to_rgb(hex: String) -> Result<(i64, i64, i64), String> {
	let (r, g, b) = parse_hex(&hex)?;
	Ok((i64::from(r), i64::from(g), i64::from(b)))
}

/// Formats `(r, g, b)` channels (each `0..=255`) as a `#rrggbb` hex color.
#[surrealism]
fn rgb_to_hex(r: i64, g: i64, b: i64) -> Result<String, String> {
	Ok(format_hex(validate_channel("r", r)?, validate_channel("g", g)?, validate_channel("b", b)?))
}

/// Converts `(r, g, b)` channels (each `0..=255`) to `(h, s, l)`, where `h` is
/// in degrees `[0, 360)` and `s`/`l` are percentages `[0, 100]`.
#[surrealism]
fn rgb_to_hsl(r: i64, g: i64, b: i64) -> Result<(f64, f64, f64), String> {
	let (r, g, b) =
		(validate_channel("r", r)?, validate_channel("g", g)?, validate_channel("b", b)?);
	Ok(rgb_to_hsl_f(r, g, b))
}

/// Converts `(h, s, l)` (`h` in degrees, `s`/`l` as percentages) to `(r, g, b)`
/// channels, each `0..=255`.
#[surrealism]
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (i64, i64, i64) {
	let (r, g, b) = hsl_to_rgb_f(h, s, l);
	(i64::from(r), i64::from(g), i64::from(b))
}

/// Lightens a hex color by adding `amount` percentage points to its
/// lightness (clamped to `[0, 100]`).
#[surrealism]
fn lighten(hex: String, amount: f64) -> Result<String, String> {
	let (r, g, b) = parse_hex(&hex)?;
	let (h, s, l) = rgb_to_hsl_f(r, g, b);
	let (r, g, b) = hsl_to_rgb_f(h, s, (l + amount).clamp(0.0, 100.0));
	Ok(format_hex(r, g, b))
}

/// Darkens a hex color by subtracting `amount` percentage points from its
/// lightness (clamped to `[0, 100]`).
#[surrealism]
fn darken(hex: String, amount: f64) -> Result<String, String> {
	let (r, g, b) = parse_hex(&hex)?;
	let (h, s, l) = rgb_to_hsl_f(r, g, b);
	let (r, g, b) = hsl_to_rgb_f(h, s, (l - amount).clamp(0.0, 100.0));
	Ok(format_hex(r, g, b))
}

/// WCAG 2.x relative luminance of a hex color, in `[0, 1]`.
#[surrealism]
fn luminance(hex: String) -> Result<f64, String> {
	let (r, g, b) = parse_hex(&hex)?;
	Ok(relative_luminance(r, g, b))
}

/// WCAG 2.x contrast ratio between two hex colors, in `[1, 21]`.
#[surrealism]
fn contrast_ratio(hex_a: String, hex_b: String) -> Result<f64, String> {
	let (ra, ga, ba) = parse_hex(&hex_a)?;
	let (rb, gb, bb) = parse_hex(&hex_b)?;
	let la = relative_luminance(ra, ga, ba);
	let lb = relative_luminance(rb, gb, bb);
	let (lighter, darker) = if la > lb {
		(la, lb)
	} else {
		(lb, la)
	};
	Ok((lighter + 0.05) / (darker + 0.05))
}

/// Whether a hex color reads as "light" (i.e. dark text on it would pass
/// contrast checks better than light text).
#[surrealism]
fn is_light(hex: String) -> Result<bool, String> {
	let (r, g, b) = parse_hex(&hex)?;
	Ok(relative_luminance(r, g, b) > 0.179)
}

/// Inverts a hex color (255 - channel, per channel).
#[surrealism]
fn invert(hex: String) -> Result<String, String> {
	let (r, g, b) = parse_hex(&hex)?;
	Ok(format_hex(255 - r, 255 - g, 255 - b))
}

/// Returns the complementary hex color (hue rotated 180 degrees).
#[surrealism]
fn complementary(hex: String) -> Result<String, String> {
	let (r, g, b) = parse_hex(&hex)?;
	let (h, s, l) = rgb_to_hsl_f(r, g, b);
	let (r, g, b) = hsl_to_rgb_f(h + 180.0, s, l);
	Ok(format_hex(r, g, b))
}
