//! Semantic version parsing and comparison for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::semver AS f"bucket:/semver.surli";`
//! and call `mod::semver::parse("1.2.3-alpha+build.1")`,
//! `mod::semver::compare("1.2.3", "1.2.4")`,
//! `mod::semver::satisfies("1.5.0", "^1.0.0")`, etc.

use serde_json::{Value, json};
use std::cmp::Ordering;
use surrealism::surrealism;

/// Parses a semantic version string into its major, minor, patch, pre-release,
/// and build components.
#[surrealism]
fn parse(version: String) -> Result<Value, String> {
	let parsed = semver::Version::parse(&version).map_err(|e| e.to_string())?;
	Ok(json!({
		"major": parsed.major,
		"minor": parsed.minor,
		"patch": parsed.patch,
		"pre": parsed.pre.as_str(),
		"build": parsed.build.as_str(),
	}))
}

/// Compares two semantic versions by SemVer precedence (build metadata is
/// ignored), returning -1, 0, or 1.
#[surrealism]
fn compare(a: String, b: String) -> Result<i64, String> {
	let a = semver::Version::parse(&a).map_err(|e| e.to_string())?;
	let b = semver::Version::parse(&b).map_err(|e| e.to_string())?;
	Ok(match a.cmp_precedence(&b) {
		Ordering::Less => -1,
		Ordering::Equal => 0,
		Ordering::Greater => 1,
	})
}

/// Checks whether a version satisfies a semver requirement (e.g. `^1.0.0`).
#[surrealism]
fn satisfies(version: String, requirement: String) -> Result<bool, String> {
	let version = semver::Version::parse(&version).map_err(|e| e.to_string())?;
	let requirement = semver::VersionReq::parse(&requirement).map_err(|e| e.to_string())?;
	Ok(requirement.matches(&version))
}
