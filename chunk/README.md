# surrealism-chunk

RAG pipeline text chunking strategies for Surrealism.

```surql
DEFINE MODULE mod::chunk AS f"bucket:/chunk.surli";

RETURN mod::chunk::fixed($text, 500, 50);
RETURN mod::chunk::sentence($text, 500, 1);
RETURN mod::chunk::paragraph($text, 1000, 1);
RETURN mod::chunk::recursive($text, 500, 1);
RETURN mod::chunk::markdown($text, 500, 1);
```

| Function | Signature | Description |
|---|---|---|
| `fixed` | `(text: string, size: int, overlap: int) -> array<string>` | Fixed-size character windows |
| `sentence` | `(text: string, size: int, overlap: int) -> array<string>` | Sentences grouped up to `size` |
| `paragraph` | `(text: string, size: int, overlap: int) -> array<string>` | Paragraphs grouped up to `size` |
| `recursive` | `(text: string, size: int, overlap: int) -> array<string>` | Paragraph, then sentence, then fixed splitting |
| `markdown` | `(text: string, size: int, overlap: int) -> array<string>` | ATX heading sections, recursive for oversized ones |

`size` must be a positive integer, and `overlap` must be non-negative and less than `size`.

For `fixed` the `overlap` counts characters; for `sentence`, `paragraph`, `recursive`, and `markdown` it counts whole segments repeated between consecutive chunks.

Sentence boundaries are `.`, `!`, or `?` followed by whitespace or end-of-string, skipping common abbreviations such as `Dr.`, `Inc.`, and `etc.`. Paragraph boundaries are blank lines. Markdown sections start at lines whose first non-whitespace characters are 1-6 `#` followed by a space, and each heading stays attached to the section it introduces.
