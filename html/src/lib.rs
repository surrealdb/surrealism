//! HTML sanitization and extraction functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::html AS f"bucket:/html.surli";`
//! and call `mod::html::sanitize("<script>...")`, `mod::html::text("...")`,
//! etc.

use scraper::{ElementRef, Html, Node, Selector};
use surrealism::surrealism;

fn selector(sel: &str) -> Selector {
	Selector::parse(sel).expect("valid CSS selector")
}

/// Collects text nodes under `el`, skipping `<script>`/`<style>` subtrees
/// (their content isn't visible page text).
fn collect_text(el: ElementRef, out: &mut String) {
	for child in el.children() {
		match child.value() {
			Node::Text(text) => {
				out.push_str(text);
				out.push(' ');
			}
			Node::Element(elem) if !matches!(elem.name(), "script" | "style") => {
				if let Some(child_el) = ElementRef::wrap(child) {
					collect_text(child_el, out);
				}
			}
			_ => {}
		}
	}
}

/// Removes unsafe elements/attributes (scripts, event handlers, `javascript:`
/// URLs, etc.), leaving safe formatting markup intact.
#[surrealism]
fn sanitize(input: String) -> String {
	ammonia::clean(&input)
}

/// Extracts the visible text content, with whitespace collapsed.
#[surrealism]
fn text(input: String) -> String {
	let document = Html::parse_document(&input);
	let mut raw = String::new();
	collect_text(document.root_element(), &mut raw);
	raw.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Extracts the `<title>` content, if present.
#[surrealism]
fn title(input: String) -> Option<String> {
	let document = Html::parse_document(&input);
	document
		.select(&selector("title"))
		.next()
		.map(|el| el.text().collect::<String>().trim().to_string())
}

/// Extracts every `<a href="...">` target.
#[surrealism]
fn links(input: String) -> Vec<String> {
	let document = Html::parse_document(&input);
	document
		.select(&selector("a[href]"))
		.filter_map(|el| el.value().attr("href").map(String::from))
		.collect()
}

/// Extracts a `<meta name="..." content="...">` or
/// `<meta property="..." content="...">` value by name/property
/// (case-insensitive) — covers standard meta tags and Open Graph tags.
#[surrealism]
fn meta(input: String, name: String) -> Option<String> {
	let document = Html::parse_document(&input);
	document.select(&selector("meta")).find_map(|el| {
		let attrs = el.value();
		let matches = attrs.attr("name").is_some_and(|n| n.eq_ignore_ascii_case(&name))
			|| attrs.attr("property").is_some_and(|p| p.eq_ignore_ascii_case(&name));
		matches.then(|| attrs.attr("content").map(String::from)).flatten()
	})
}
