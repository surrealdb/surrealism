//! Text diffing functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::diff AS f"bucket:/diff.surli";`
//! and call `mod::diff::unified(a, b)`, `mod::diff::ratio(a, b)`.

use similar::TextDiff;
use surrealism::surrealism;

/// Returns a unified-diff (`diff -u` style) between `a` and `b`.
#[surrealism]
fn unified(a: String, b: String) -> String {
	TextDiff::from_lines(&a, &b).unified_diff().header("a", "b").to_string()
}

/// Returns a similarity ratio between `a` and `b`, from `0.0` (completely
/// different) to `1.0` (identical).
#[surrealism]
fn ratio(a: String, b: String) -> f64 {
	f64::from(TextDiff::from_lines(&a, &b).ratio())
}
