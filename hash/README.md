# surrealism-hash

Hashing, HMAC, and encoding functions for Surrealism. Uses `sha2`, `md-5`, `hmac`,
`base64`, `hex`, `uuid`, and `ulid`.

```surql
DEFINE MODULE mod::hash AS f"bucket:/hash.surli";

RETURN mod::hash::sha256("hello");     -- '2cf24dba...'
RETURN mod::hash::uuid_v4();
RETURN mod::hash::base64_encode("hello"); -- 'aGVsbG8='
```

| Function | Signature | Description |
|---|---|---|
| `sha256` | `(input: string) -> string` | SHA-256 hex digest |
| `sha512` | `(input: string) -> string` | SHA-512 hex digest |
| `md5` | `(input: string) -> string` | MD5 hex digest (checksums only, not secure) |
| `hmac_sha256` | `(input: string, key: string) -> string` | HMAC-SHA256 hex digest |
| `base64_encode` | `(input: string) -> string` | Standard base64 encode |
| `base64_decode` | `(input: string) -> string` | Standard base64 decode (must be valid UTF-8) |
| `hex_encode` | `(input: string) -> string` | Lowercase hex encode |
| `hex_decode` | `(input: string) -> string` | Hex decode (must be valid UTF-8) |
| `uuid_v4` | `() -> string` | Random UUID v4 |
| `ulid` | `() -> string` | New ULID (sortable, timestamp-prefixed) |
