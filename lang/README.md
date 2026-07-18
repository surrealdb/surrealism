# surrealism-lang

Natural language detection for Surrealism. Uses `whatlang`.

```surql
DEFINE MODULE mod::lang AS f"bucket:/lang.surli";

RETURN mod::lang::detect("This is an English sentence."); -- { confidence: 0.99..., is_reliable: true, lang: 'eng' }
RETURN mod::lang::code("Ceci est une phrase en français."); -- 'fra'
```

| Function | Signature | Description |
|---|---|---|
| `detect` | `(text: string) -> object` | Detected language as `{ lang, confidence, is_reliable }` |
| `code` | `(text: string) -> string \| none` | Detected ISO 639-3 language code, or `none` if undetectable |

`lang` is an ISO 639-3 code, e.g. `"eng"`, `"fra"`. `confidence` is a value from 0.0 to 1.0. `detect` errors if the text is too short or ambiguous to detect a language; `code` returns `none` in that case instead.
