# surrealism-aes

AES-256-GCM authenticated symmetric encryption for Surrealism.

```surql
DEFINE MODULE mod::aes AS f"bucket:/aes.surli";

LET $key = mod::aes::generate_key();
LET $enc = mod::aes::encrypt($key, <bytes> "hello world");
LET $dec = mod::aes::decrypt($key, $enc);
```

| Function | Signature | Description |
|---|---|---|
| `generate_key` | `() -> bytes` | Generates a random 32-byte AES-256 key |
| `encrypt` | `(key: bytes, plaintext: bytes) -> bytes` | Encrypts `plaintext` with a 32-byte `key`, returning `nonce \|\| ciphertext` |
| `decrypt` | `(key: bytes, data: bytes) -> bytes` | Decrypts `nonce \|\| ciphertext` data with the 32-byte `key` that encrypted it |

`key` must be exactly 32 bytes. `data` passed to `decrypt` must be longer than
12 bytes (the nonce prefix). A wrong key or tampered ciphertext makes
`decrypt` return an error rather than panic.
