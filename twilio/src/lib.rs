//! Twilio SMS integration for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::twilio AS f"bucket:/twilio.surli";`
//! and call `mod::twilio::send(...)`.

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use serde_json::Value;
use surrealism::surrealism;

/// Sends an SMS via the Twilio Messages API using Account SID/Auth Token Basic Auth.
#[surrealism]
fn send(
	account_sid: String,
	auth_token: String,
	from: String,
	to: String,
	body: String,
) -> Result<Value, String> {
	let url = format!("https://api.twilio.com/2010-04-01/Accounts/{account_sid}/Messages.json");

	let form_body = form_urlencoded::Serializer::new(String::new())
		.append_pair("To", &to)
		.append_pair("From", &from)
		.append_pair("Body", &body)
		.finish();

	let credentials = STANDARD.encode(format!("{account_sid}:{auth_token}"));
	let headers = serde_json::json!({
		"Authorization": format!("Basic {credentials}"),
		"Content-Type": "application/x-www-form-urlencoded",
	});

	surrealism::run("http::post".to_string(), None, (url, form_body, headers))
		.map_err(|e| e.to_string())
}
