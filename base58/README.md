# surrealism-base58

Base58 and Base58Check encoding for Surrealism.

```surql
DEFINE MODULE mod::base58 AS f"bucket:/base58.surli";

RETURN mod::base58::encode(<bytes>"hello");
RETURN mod::base58::decode("Cn8eVZg");
RETURN mod::base58::encode_check(<bytes>"hello");
RETURN mod::base58::decode_check("2L5B5yqsVG8Vt");
```

| Function | Signature | Description |
|---|---|---|
| `encode` | `(data: bytes) -> string` | Base58-encodes `data` |
| `decode` | `(data: string) -> bytes` | Decodes a Base58 string back into bytes |
| `encode_check` | `(data: bytes) -> string` | Base58Check-encodes `data`, appending a 4-byte checksum |
| `decode_check` | `(data: string) -> bytes` | Decodes a Base58Check string, verifying and stripping its checksum |

`decode` and `decode_check` return an error if `data` contains characters outside the Base58 alphabet. `decode_check` also returns an error if the checksum does not match.
