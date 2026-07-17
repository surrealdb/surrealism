# surrealism-text

String and text manipulation functions for Surrealism. Uses `deunicode`, `heck`, and
`regex`.

```surql
DEFINE MODULE mod::text AS f"bucket:/text.surli";

RETURN mod::text::slugify("Héllo, World! Café"); -- 'hello-world-cafe'
RETURN mod::text::snake_case("myVarName");       -- 'my_var_name'
RETURN mod::text::levenshtein("kitten", "sitting"); -- 3
```

| Function | Signature | Description |
|---|---|---|
| `slugify` | `(input: string) -> string` | ASCII-transliterated, lowercased, hyphenated slug |
| `truncate` | `(input: string, max_len: int) -> string` | Truncates to `max_len` characters |
| `snake_case` | `(input: string) -> string` | `snake_case` |
| `camel_case` | `(input: string) -> string` | `camelCase` |
| `kebab_case` | `(input: string) -> string` | `kebab-case` |
| `pascal_case` | `(input: string) -> string` | `PascalCase` |
| `title_case` | `(input: string) -> string` | `Title Case` |
| `levenshtein` | `(a: string, b: string) -> int` | Edit distance between `a` and `b` |
| `word_count` | `(input: string) -> int` | Whitespace-separated word count |
| `strip_html` | `(input: string) -> string` | Removes `<tag>` markup |
| `capitalize` | `(input: string) -> string` | Uppercases the first character |
| `reverse` | `(input: string) -> string` | Reverses by character |
