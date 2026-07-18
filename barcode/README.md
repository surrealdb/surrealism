# surrealism-barcode

Linear barcode generation (Code128, EAN-13) for Surrealism, rendered as PNG bytes.

```surql
DEFINE MODULE mod::barcode AS f"bucket:/barcode.surli";

LET $code128 = mod::barcode::code128("HELLO123");
LET $ean13 = mod::barcode::ean13("5901234123457");
```

| Function | Signature | Description |
|---|---|---|
| `code128` | `(data: string) -> bytes` | Encodes `data` as a Code128 barcode (character-set B) and renders it to PNG bytes |
| `ean13` | `(data: string) -> bytes` | Encodes `data` as an EAN-13 barcode and renders it to PNG bytes |

`ean13` requires exactly 12 or 13 ASCII digit characters; anything else returns an `Err`.
