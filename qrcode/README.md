# surrealism-qrcode

QR code generation for Surrealism, rendered as PNG bytes or an SVG string.

```surql
DEFINE MODULE mod::qrcode AS f"bucket:/qrcode.surli";

LET $png = mod::qrcode::generate("https://surrealdb.com");
LET $svg = mod::qrcode::generate_svg("https://surrealdb.com");
```

| Function | Signature | Description |
|---|---|---|
| `generate` | `(data: string) -> bytes` | Encodes `data` as a QR code and renders it to PNG bytes |
| `generate_svg` | `(data: string) -> string` | Encodes `data` as a QR code and renders it as an SVG string |
