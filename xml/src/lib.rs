//! XML-to-JSON conversion for Surrealism, using the "xmltodict" convention.
//!
//! Register with e.g. `DEFINE MODULE mod::xml AS f"bucket:/xml.surli";` and
//! call `mod::xml::to_json("<root attr=\"1\"><item>a</item><item>b</item></root>")`.

use std::collections::HashMap;

use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use serde_json::{Map, Value};
use surrealism::surrealism;

/// Reads a start tag's name and attributes as owned strings.
fn read_start(start: &BytesStart) -> Result<(String, Vec<(String, String)>), String> {
	let name = String::from_utf8_lossy(start.name().as_ref()).into_owned();
	let mut attrs = Vec::new();
	for attr in start.attributes() {
		let attr = attr.map_err(|e| e.to_string())?;
		let key = String::from_utf8_lossy(attr.key.as_ref()).into_owned();
		let value = attr.unescape_value().map_err(|e| e.to_string())?.into_owned();
		attrs.push((key, value));
	}
	Ok((name, attrs))
}

/// Adds a child element's value under its tag name, collecting repeated
/// sibling tags into an array in encounter order.
fn push_child(
	order: &mut Vec<String>,
	children: &mut HashMap<String, Vec<Value>>,
	name: String,
	value: Value,
) {
	if !children.contains_key(&name) {
		order.push(name.clone());
	}
	children.entry(name).or_default().push(value);
}

/// Builds the JSON value for an element from its attributes, direct text,
/// and children, following the xmltodict convention.
fn element_value(
	attrs: Vec<(String, String)>,
	text: Option<String>,
	order: Vec<String>,
	mut children: HashMap<String, Vec<Value>>,
) -> Value {
	if attrs.is_empty() && order.is_empty() {
		return match text {
			Some(text) => Value::String(text),
			None => Value::Null,
		};
	}
	let mut object = Map::new();
	for (key, value) in attrs {
		object.insert(format!("@{key}"), Value::String(value));
	}
	if let Some(text) = text {
		object.insert("#text".to_string(), Value::String(text));
	}
	for name in order {
		let mut values = children.remove(&name).unwrap_or_default();
		let value = if values.len() == 1 {
			values.remove(0)
		} else {
			Value::Array(values)
		};
		object.insert(name, value);
	}
	Value::Object(object)
}

/// Parses an element's body (everything after its start tag up to and
/// including its matching end tag) into a JSON value.
fn parse_body(
	reader: &mut Reader<&[u8]>,
	buf: &mut Vec<u8>,
	attrs: Vec<(String, String)>,
) -> Result<Value, String> {
	let mut text_parts: Vec<String> = Vec::new();
	let mut order: Vec<String> = Vec::new();
	let mut children: HashMap<String, Vec<Value>> = HashMap::new();

	loop {
		buf.clear();
		match reader.read_event_into(buf).map_err(|e| e.to_string())? {
			Event::Start(start) => {
				let (name, child_attrs) = read_start(&start)?;
				let value = parse_body(reader, buf, child_attrs)?;
				push_child(&mut order, &mut children, name, value);
			}
			Event::Empty(start) => {
				let (name, child_attrs) = read_start(&start)?;
				let value = element_value(child_attrs, None, Vec::new(), HashMap::new());
				push_child(&mut order, &mut children, name, value);
			}
			Event::Text(text) => {
				let decoded = text.unescape().map_err(|e| e.to_string())?;
				let trimmed = decoded.trim();
				if !trimmed.is_empty() {
					text_parts.push(trimmed.to_string());
				}
			}
			Event::CData(text) => {
				let decoded = String::from_utf8_lossy(text.as_ref()).into_owned();
				let trimmed = decoded.trim();
				if !trimmed.is_empty() {
					text_parts.push(trimmed.to_string());
				}
			}
			Event::End(_) => break,
			Event::Eof => return Err("unexpected end of document".to_string()),
			_ => {}
		}
	}

	let text = if text_parts.is_empty() {
		None
	} else {
		Some(text_parts.join(""))
	};
	Ok(element_value(attrs, text, order, children))
}

/// Converts an XML document to JSON: the root element becomes the single
/// top-level key; attributes become `@`-prefixed keys; direct text becomes
/// the element's value (or its `#text` key when attributes/children are
/// present); repeated sibling tags collect into arrays.
#[surrealism]
fn to_json(xml: String) -> Result<Value, String> {
	let mut reader = Reader::from_str(&xml);
	reader.config_mut().trim_text(false);

	let mut buf = Vec::new();
	loop {
		buf.clear();
		match reader.read_event_into(&mut buf).map_err(|e| e.to_string())? {
			Event::Start(start) => {
				let (name, attrs) = read_start(&start)?;
				let value = parse_body(&mut reader, &mut buf, attrs)?;
				let mut root = Map::new();
				root.insert(name, value);
				return Ok(Value::Object(root));
			}
			Event::Empty(start) => {
				let (name, attrs) = read_start(&start)?;
				let value = element_value(attrs, None, Vec::new(), HashMap::new());
				let mut root = Map::new();
				root.insert(name, value);
				return Ok(Value::Object(root));
			}
			Event::Eof => return Err("no root element found".to_string()),
			_ => {}
		}
	}
}
