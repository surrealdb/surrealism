//! JSON Web Token encoding and decoding for Surrealism.
//!
//! Register with e.g. `DEFINE MODULE mod::jwt AS f"bucket:/jwt.surli";` and
//! call `mod::jwt::encode({sub: "user-1"}, "my-secret")`.

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde_json::Value;
use surrealism::surrealism;

/// Encodes `claims` (a JSON object) as a signed JWT using HS256.
#[surrealism]
fn encode(claims: Value, secret: String) -> Result<String, String> {
	jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
		.map_err(|e| e.to_string())
}

/// Verifies a JWT's HS256 signature and returns its claims. Tokens without
/// an `exp` claim decode successfully; a present but expired `exp` still
/// fails.
#[surrealism]
fn decode(token: String, secret: String) -> Result<Value, String> {
	let mut validation = Validation::new(Algorithm::HS256);
	validation.required_spec_claims.clear();
	jsonwebtoken::decode::<Value>(&token, &DecodingKey::from_secret(secret.as_bytes()), &validation)
		.map(|data| data.claims)
		.map_err(|e| e.to_string())
}
