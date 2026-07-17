# surrealism-hash

Hashing and HMAC functions for Surrealism, plus `base64`/`hex` encoding
submodules. Uses `sha2`, `md-5`, `hmac`, `base64`, and `hex`.

```surql
DEFINE MODULE mod::hash AS f"bucket:/hash.surli";

RETURN mod::hash::sha256("hello");            -- '2cf24dba...'
RETURN mod::hash::base64::encode("hello");    -- 'aGVsbG8='
RETURN mod::hash::hex::decode("68656c6c6f");  -- 'hello'
```

| Function | Signature | Description |
|---|---|---|
| `sha256` | `(input: string) -> string` | SHA-256 hex digest |
| `sha512` | `(input: string) -> string` | SHA-512 hex digest |
| `md5` | `(input: string) -> string` | MD5 hex digest (checksums only, not secure) |
| `hmac_sha256` | `(input: string, key: string) -> string` | HMAC-SHA256 hex digest |
| `base64::encode` | `(input: string) -> string` | Standard base64 encode |
| `base64::decode` | `(input: string) -> string` | Standard base64 decode (must be valid UTF-8) |
| `hex::encode` | `(input: string) -> string` | Lowercase hex encode |
| `hex::decode` | `(input: string) -> string` | Hex decode (must be valid UTF-8) |

For UUID/ULID generation, use SurrealDB's native `rand::uuid()` / `rand::ulid()`.
