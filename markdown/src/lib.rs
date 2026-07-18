//! Markdown to HTML rendering functions for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::markdown AS f"bucket:/markdown.surli";`
//! and call `mod::markdown::to_html("# Hi")`.

use pulldown_cmark::{Options, Parser, html};
use surrealism::surrealism;

/// Renders markdown to HTML, with tables, strikethrough, and task-list
/// extensions enabled.
#[surrealism]
fn to_html(input: String) -> String {
	let mut options = Options::empty();
	options.insert(Options::ENABLE_TABLES);
	options.insert(Options::ENABLE_STRIKETHROUGH);
	options.insert(Options::ENABLE_TASKLISTS);
	let parser = Parser::new_ext(&input, options);
	let mut output = String::new();
	html::push_html(&mut output, parser);
	output
}
