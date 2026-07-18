# surrealism-zip

ZIP archive creation/extraction for Surrealism.

```surql
DEFINE MODULE mod::zip AS f"bucket:/zip.surli";

LET $archive = mod::zip::create({ "a.txt": "hello", "b.bin": $bytes });
LET $files = mod::zip::extract($archive);
```

| Function | Signature | Description |
|---|---|---|
| `create` | `(files: object) -> bytes` | Builds a ZIP archive from an object mapping filename to file content (a string or `bytes` value) |
| `extract` | `(archive: bytes) -> object` | Extracts a ZIP archive into an object mapping each entry's filename to its content as `bytes` |

Entries are stored with deflate compression; no encryption, no bzip2/zstd.
