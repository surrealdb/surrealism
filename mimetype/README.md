# surrealism-mimetype

File type detection from magic bytes for Surrealism. Uses `infer`.

```surql
DEFINE MODULE mod::mimetype AS f"bucket:/mimetype.surli";

RETURN mod::mimetype::detect(data);    -- 'image/png'
RETURN mod::mimetype::extension(data); -- 'png'
```

| Function | Signature | Description |
|---|---|---|
| `detect` | `(data: bytes) -> string \| none` | Detected MIME type, e.g. `"image/png"` |
| `extension` | `(data: bytes) -> string \| none` | Detected file extension, e.g. `"png"` |
