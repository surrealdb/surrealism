//! Send HTTP/HTTPS webhook events from Surrealism (Slack, Discord, Teams,
//! generic webhook receivers, etc.).
//!
//! Register with e.g. `DEFINE MODULE webhook AS f"bucket:/webhook.surli";` and
//! call `webhook::post("https://hooks.slack.com/...", '{"text":"hi"}')`. The
//! target host:port must be listed in `allow_net` in `surrealism.toml`.
//!
//! Connections are made directly over WASI sockets (no async runtime): plain
//! `TcpStream` for `http://`, and a hand-driven `rustls` handshake over
//! `TcpStream` for `https://`.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, OnceLock};

use anyhow::{Context, Result, bail};
use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use surrealism::surrealism;

struct ParsedUrl {
	https: bool,
	host: String,
	port: u16,
	path_and_query: String,
}

fn parse_url(raw: &str) -> Result<ParsedUrl> {
	let url = ::url::Url::parse(raw).with_context(|| format!("Invalid URL: '{raw}'"))?;
	let https = match url.scheme() {
		"https" => true,
		"http" => false,
		other => bail!("Unsupported URL scheme '{other}', expected http or https"),
	};
	let host = url.host_str().context("URL is missing a host")?.to_string();
	let port = url.port_or_known_default().context("Could not determine a port")?;
	let mut path_and_query = url.path().to_string();
	if path_and_query.is_empty() {
		path_and_query.push('/');
	}
	if let Some(q) = url.query() {
		path_and_query.push('?');
		path_and_query.push_str(q);
	}
	Ok(ParsedUrl {
		https,
		host,
		port,
		path_and_query,
	})
}

fn root_store() -> &'static Arc<RootCertStore> {
	static STORE: OnceLock<Arc<RootCertStore>> = OnceLock::new();
	STORE.get_or_init(|| {
		let mut store = RootCertStore::empty();
		store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
		Arc::new(store)
	})
}

fn tls_config() -> Arc<ClientConfig> {
	static CONFIG: OnceLock<Arc<ClientConfig>> = OnceLock::new();
	Arc::clone(CONFIG.get_or_init(|| {
		Arc::new(
			ClientConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
				.with_protocol_versions(&[&rustls::version::TLS12, &rustls::version::TLS13])
				.expect("supported protocol versions")
				.with_root_certificates(Arc::clone(root_store()))
				.with_no_client_auth(),
		)
	}))
}

fn build_request(
	parsed: &ParsedUrl,
	method: &str,
	body: &str,
	extra_headers: &[(String, String)],
	default_json_content_type: bool,
) -> String {
	let mut req = format!(
		"{method} {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\nContent-Length: {len}\r\n",
		path = parsed.path_and_query,
		host = parsed.host,
		len = body.len(),
	);
	let has_content_type = extra_headers.iter().any(|(k, _)| k.eq_ignore_ascii_case("content-type"));
	if default_json_content_type && !has_content_type {
		req.push_str("Content-Type: application/json\r\n");
	}
	for (k, v) in extra_headers {
		req.push_str(&format!("{k}: {v}\r\n"));
	}
	req.push_str("\r\n");
	req.push_str(body);
	req
}

fn parse_response(raw: &[u8]) -> Result<(i64, String)> {
	let text = String::from_utf8_lossy(raw);
	let (headers, body) = text.split_once("\r\n\r\n").unwrap_or((&text, ""));
	let status_line = headers.lines().next().context("Empty HTTP response")?;
	let status = status_line
		.split_whitespace()
		.nth(1)
		.and_then(|s| s.parse::<i64>().ok())
		.with_context(|| format!("Could not parse HTTP status from '{status_line}'"))?;
	Ok((status, body.to_string()))
}

fn send_request(parsed: &ParsedUrl, request: &str) -> Result<(i64, String)> {
	let stream = TcpStream::connect((parsed.host.as_str(), parsed.port))
		.with_context(|| format!("Failed to connect to {}:{}", parsed.host, parsed.port))?;

	let raw_response = if parsed.https {
		let server_name = ServerName::try_from(parsed.host.clone())
			.map_err(|_| anyhow::anyhow!("'{}' is not a valid DNS name for TLS", parsed.host))?;
		let conn = ClientConnection::new(tls_config(), server_name)
			.context("Failed to initialise TLS session")?;
		let mut tls = StreamOwned::new(conn, stream);
		tls.write_all(request.as_bytes()).context("Failed to write HTTPS request")?;
		let mut buf = Vec::new();
		let result = tls.read_to_end(&mut buf);
		if buf.is_empty() {
			result.context("Failed to read HTTPS response")?;
		}
		buf
	} else {
		let mut stream = stream;
		stream.write_all(request.as_bytes()).context("Failed to write HTTP request")?;
		let mut buf = Vec::new();
		let result = stream.read_to_end(&mut buf);
		if buf.is_empty() {
			result.context("Failed to read HTTP response")?;
		}
		buf
	};

	parse_response(&raw_response)
}

/// Sends an HTTP(S) `POST` with an `application/json` content type. Returns
/// `(status_code, response_body)`.
#[surrealism]
fn post(url: String, body: String) -> Result<(i64, String)> {
	let parsed = parse_url(&url)?;
	let request = build_request(&parsed, "POST", &body, &[], true);
	send_request(&parsed, &request)
}

/// Sends an HTTP(S) `POST` with custom headers (e.g. `Authorization`,
/// `Content-Type`). Returns `(status_code, response_body)`.
#[surrealism]
fn post_with_headers(url: String, body: String, headers: Vec<(String, String)>) -> Result<(i64, String)> {
	let parsed = parse_url(&url)?;
	let request = build_request(&parsed, "POST", &body, &headers, true);
	send_request(&parsed, &request)
}

/// Sends an HTTP(S) `GET`. Returns `(status_code, response_body)`.
#[surrealism]
fn get(url: String) -> Result<(i64, String)> {
	let parsed = parse_url(&url)?;
	let request = build_request(&parsed, "GET", "", &[], false);
	send_request(&parsed, &request)
}

/// Sends a JSON body to `url` via the *host's* native `http::post` function
/// instead of connecting directly from inside the WASM guest. Returns the
/// response body (re-serialized as JSON text). Errors (including non-2xx
/// responses) surface as an `Err` rather than a status code.
///
/// Requires the module to be granted `allow_functions = ["http::post"]` (or
/// `["*"]`) in `surrealism.toml`, and the server itself to allow outbound
/// requests to `url`'s host.
///
/// Use this instead of [`post`] whenever the target is identified by
/// **hostname** rather than a literal IP address (e.g. Slack, Discord, Teams
/// webhooks) — guest code cannot resolve hostnames itself. WASI's
/// `ip-name-lookup` is intentionally disabled at the runtime level to prevent
/// DNS-tunnelling data exfiltration, so hostname resolution and the actual
/// TLS connection are instead performed by the host, which already has full,
/// unsandboxed network and DNS access via `http::post`.
#[surrealism]
fn post_via_host(url: String, json_body: String) -> Result<String> {
	let body: serde_json::Value = serde_json::from_str(&json_body)
		.with_context(|| format!("'{json_body}' is not valid JSON"))?;
	let value: serde_json::Value = surrealism::run("http::post".to_string(), None, (url, body))
		.context("Call to host 'http::post' failed")?;
	Ok(value.to_string())
}
