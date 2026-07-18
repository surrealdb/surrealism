# surrealism-compress

Gzip compression functions for Surrealism.

```surql
DEFINE MODULE mod::compress AS f"bucket:/compress.surli";

LET $packed = mod::compress::gzip($data);
LET $original = mod::compress::gunzip($packed);
```

| Function | Signature | Description |
|---|---|---|
| `gzip` | `(input: bytes) -> bytes` | Gzip-compresses `input` |
| `gunzip` | `(input: bytes) -> bytes` | Gzip-decompresses `input` |
