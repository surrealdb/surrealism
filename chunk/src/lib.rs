//! RAG pipeline text chunking strategies for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::chunk AS f"bucket:/chunk.surli";` and
//! call `mod::chunk::fixed($text, 500, 50)`,
//! `mod::chunk::sentence($text, 500, 1)`,
//! `mod::chunk::paragraph($text, 1000, 1)`,
//! `mod::chunk::recursive($text, 500, 1)`,
//! `mod::chunk::markdown($text, 500, 1)`, etc.
//! `size` is a positive integer, and `overlap` is non-negative and less than
//! `size`. For `fixed` the overlap counts characters; for the other strategies
//! it counts whole segments repeated between consecutive chunks.

use surrealism::surrealism;

/// Checks that the size is a positive integer.
fn validate_size(size: i64) -> Result<usize, String> {
	if size <= 0 {
		return Err(format!("The size argument must be a positive integer, got: {size}"));
	}
	usize::try_from(size).map_err(|_| format!("The size argument is too large: {size}"))
}

/// Checks that the overlap is non-negative and less than the size.
fn validate_overlap(overlap: i64, size: usize) -> Result<usize, String> {
	if overlap < 0 {
		return Err(format!("The overlap argument must be a non-negative integer, got: {overlap}"));
	}
	let overlap = usize::try_from(overlap)
		.map_err(|_| format!("The overlap argument is too large: {overlap}"))?;
	if overlap >= size {
		return Err(format!("The overlap ({overlap}) must be less than the size ({size})"));
	}
	Ok(overlap)
}

/// Splits text into sentences on `.`, `!`, or `?` followed by whitespace or
/// end-of-string, skipping common abbreviations.
fn split_sentences(text: &str) -> Vec<&str> {
	let mut sentences = Vec::new();
	let bytes = text.as_bytes();
	let len = bytes.len();
	let mut start = 0;

	let mut i = 0;
	while i < len {
		let b = bytes[i];
		if b == b'.' || b == b'!' || b == b'?' {
			let mut end = i + 1;
			while end < len && matches!(bytes[end], b'"' | b'\'' | b')' | b']') {
				end += 1;
			}
			if end >= len || bytes[end].is_ascii_whitespace() {
				if b == b'.' {
					let word_start = text[start..i].rfind(char::is_whitespace).map_or(start, |p| {
						let width = text[start + p..].chars().next().map_or(1, char::len_utf8);
						start + p + width
					});
					let word = &text[word_start..i];
					let lower = word.to_ascii_lowercase();
					if matches!(
						lower.as_str(),
						"mr" | "mrs"
							| "ms" | "dr" | "prof"
							| "sr" | "jr" | "vs" | "etc"
							| "inc" | "ltd" | "st"
							| "e.g" | "i.e" | "fig"
							| "vol" | "no" | "approx"
							| "dept" | "est" | "gen"
							| "gov"
					) {
						i += 1;
						continue;
					}
				}

				let sentence = text[start..end].trim();
				if !sentence.is_empty() {
					sentences.push(sentence);
				}
				while end < len && bytes[end].is_ascii_whitespace() {
					end += 1;
				}
				start = end;
				i = end;
				continue;
			}
		}
		i += 1;
	}

	let rest = text[start..].trim();
	if !rest.is_empty() {
		sentences.push(rest);
	}

	sentences
}

/// Splits text into paragraphs on blank-line boundaries.
fn split_paragraphs(text: &str) -> Vec<&str> {
	let mut paragraphs = Vec::new();
	let mut rest = text;

	loop {
		let boundary = rest.find("\n\n").or_else(|| rest.find("\r\n\r\n"));
		match boundary {
			Some(pos) => {
				let para = rest[..pos].trim();
				if !para.is_empty() {
					paragraphs.push(para);
				}
				let after = &rest[pos..];
				let skip = after.find(|c: char| c != '\n' && c != '\r').unwrap_or(after.len());
				rest = &rest[pos + skip..];
			}
			None => {
				let para = rest.trim();
				if !para.is_empty() {
					paragraphs.push(para);
				}
				break;
			}
		}
	}

	paragraphs
}

/// Groups text segments into chunks up to `max_size` characters, repeating the
/// last `overlap` segments of each chunk at the start of the next one.
fn group_segments(segments: &[&str], max_size: usize, overlap: usize) -> Vec<String> {
	if segments.is_empty() {
		return Vec::new();
	}
	if max_size == 0 {
		return segments.iter().map(|s| (*s).to_string()).collect();
	}

	let mut chunks = Vec::new();
	let mut i = 0;

	while i < segments.len() {
		let mut chunk = String::new();
		let mut count = 0;
		let chunk_start = i;

		while i < segments.len() {
			let sep = if chunk.is_empty() {
				""
			} else {
				" "
			};
			let needed = sep.len() + segments[i].len();
			if !chunk.is_empty() && chunk.len() + needed > max_size {
				break;
			}
			if !chunk.is_empty() {
				chunk.push(' ');
			}
			chunk.push_str(segments[i]);
			count += 1;
			i += 1;
		}

		chunks.push(chunk);

		if overlap > 0 && i < segments.len() {
			let back = overlap.min(count);
			i = i.saturating_sub(back);
			if i <= chunk_start {
				i = chunk_start + 1;
			}
		}
	}

	chunks
}

/// Splits text into fixed-size character windows.
fn chunk_fixed(text: &str, size: usize, overlap: usize) -> Vec<String> {
	if text.is_empty() || size == 0 {
		return Vec::new();
	}
	let step = size.saturating_sub(overlap).max(1);
	let chars: Vec<char> = text.chars().collect();
	let mut chunks = Vec::new();
	let mut start = 0;

	while start < chars.len() {
		let end = start.saturating_add(size).min(chars.len());
		let chunk: String = chars[start..end].iter().collect();
		let trimmed = chunk.trim();
		if !trimmed.is_empty() {
			chunks.push(trimmed.to_string());
		}
		start += step;
	}

	chunks
}

/// Chunks text by paragraph, falling back to sentence and then fixed-size
/// splitting for oversized pieces, then regroups up to `size`.
fn chunk_recursive(text: &str, size: usize, overlap: usize) -> Vec<String> {
	let paragraphs = split_paragraphs(text);

	let mut result = Vec::new();
	for para in paragraphs {
		if para.len() <= size {
			result.push(para.to_string());
		} else {
			let sentences = split_sentences(para);
			for sent in &sentences {
				if sent.len() <= size {
					result.push((*sent).to_string());
				} else {
					result.extend(chunk_fixed(sent, size, overlap));
				}
			}
		}
	}

	if result.is_empty() {
		return result;
	}
	let refs: Vec<&str> = result.iter().map(|s| s.as_str()).collect();
	group_segments(&refs, size, overlap)
}

/// Reports whether a line is an ATX heading: 1-6 `#` characters followed by a
/// space.
fn is_heading(line: &str) -> bool {
	let line = line.trim_start();
	let hashes = line.bytes().take_while(|b| *b == b'#').count();
	(1..=6).contains(&hashes) && line[hashes..].starts_with(' ')
}

/// Splits text into sections at ATX heading lines, keeping each heading at the
/// top of the section it introduces.
fn split_markdown(text: &str) -> Vec<&str> {
	let mut sections = Vec::new();
	let mut start = 0;
	let mut offset = 0;

	for line in text.split_inclusive('\n') {
		if offset > start && is_heading(line) {
			let section = text[start..offset].trim();
			if !section.is_empty() {
				sections.push(section);
			}
			start = offset;
		}
		offset += line.len();
	}

	let rest = text[start..].trim();
	if !rest.is_empty() {
		sections.push(rest);
	}

	sections
}

/// Splits text into fixed-size character windows, overlapping by `overlap`
/// characters.
#[surrealism]
fn fixed(text: String, size: i64, overlap: i64) -> Result<Vec<String>, String> {
	let size = validate_size(size)?;
	let overlap = validate_overlap(overlap, size)?;
	Ok(chunk_fixed(&text, size, overlap))
}

/// Splits text on sentence boundaries, grouping sentences into chunks of up to
/// `size` characters overlapping by `overlap` sentences.
#[surrealism]
fn sentence(text: String, size: i64, overlap: i64) -> Result<Vec<String>, String> {
	let size = validate_size(size)?;
	let overlap = validate_overlap(overlap, size)?;
	Ok(group_segments(&split_sentences(&text), size, overlap))
}

/// Splits text on blank-line boundaries, grouping paragraphs into chunks of up
/// to `size` characters overlapping by `overlap` paragraphs.
#[surrealism]
fn paragraph(text: String, size: i64, overlap: i64) -> Result<Vec<String>, String> {
	let size = validate_size(size)?;
	let overlap = validate_overlap(overlap, size)?;
	Ok(group_segments(&split_paragraphs(&text), size, overlap))
}

/// Chunks text by paragraph, falling back to sentence and then fixed-size
/// splitting, overlapping by `overlap` segments.
#[surrealism]
fn recursive(text: String, size: i64, overlap: i64) -> Result<Vec<String>, String> {
	let size = validate_size(size)?;
	let overlap = validate_overlap(overlap, size)?;
	Ok(chunk_recursive(&text, size, overlap))
}

/// Splits text at ATX heading lines with each heading kept atop its section,
/// chunking oversized sections recursively, overlapping by `overlap` segments.
#[surrealism]
fn markdown(text: String, size: i64, overlap: i64) -> Result<Vec<String>, String> {
	let size = validate_size(size)?;
	let overlap = validate_overlap(overlap, size)?;

	let mut result = Vec::new();
	for section in split_markdown(&text) {
		if section.len() <= size {
			result.push(section.to_string());
		} else {
			result.extend(chunk_recursive(section, size, overlap));
		}
	}

	let refs: Vec<&str> = result.iter().map(|s| s.as_str()).collect();
	Ok(group_segments(&refs, size, overlap))
}
