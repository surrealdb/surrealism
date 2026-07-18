# surrealism-ed25519

Ed25519 signing and verification for Surrealism.

```surql
DEFINE MODULE mod::ed25519 AS f"bucket:/ed25519.surli";

LET $secret_key = mod::ed25519::generate_secret_key();
LET $public_key = mod::ed25519::public_key($secret_key);
LET $signature = mod::ed25519::sign($secret_key, <bytes> "hello");
RETURN mod::ed25519::verify($public_key, <bytes> "hello", $signature);
```

| Function | Signature | Description |
|---|---|---|
| `generate_secret_key` | `() -> bytes` | Generates a random 32-byte secret key seed |
| `public_key` | `(secret_key: bytes) -> bytes` | Derives the 32-byte public key from a secret key seed |
| `sign` | `(secret_key: bytes, message: bytes) -> bytes` | Signs `message`, returning a 64-byte signature |
| `verify` | `(public_key: bytes, message: bytes, signature: bytes) -> bool` | Whether `signature` is valid for `message` and `public_key` |

`secret_key` must be exactly 32 bytes, `public_key` must be exactly 32 bytes, and `signature` must be exactly 64 bytes; otherwise the function returns an error. A well-formed but invalid or tampered signature makes `verify` return `false`, not an error.
