# surrealism-totp

TOTP (RFC 6238) two-factor authentication codes for Surrealism.

```surql
DEFINE MODULE mod::totp AS f"bucket:/totp.surli";

RETURN mod::totp::generate("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ");
RETURN mod::totp::verify("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ", "123456");
```

| Function | Signature | Description |
|---|---|---|
| `generate` | `(secret_base32: string) -> string` | Current 6-digit TOTP code for a base32 secret |
| `verify` | `(secret_base32: string, code: string) -> bool` | Whether the code is valid for the current time |

`secret_base32` must base32-decode to at least 128 bits (16 bytes).
