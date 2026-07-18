# surrealism-stem

Word stemming (Snowball algorithms) for Surrealism.

```surql
DEFINE MODULE mod::stem AS f"bucket:/stem.surli";

RETURN mod::stem::stem("running", "english");
RETURN mod::stem::stem_words(["running", "jumps", "flies"], "english");
```

| Function | Signature | Description |
|---|---|---|
| `stem` | `(word: string, language: string) -> string` | Snowball stem of a single word |
| `stem_words` | `(words: array<string>, language: string) -> array<string>` | Snowball stem of each word |

`language` is a lowercase language name: arabic, danish, dutch, english, finnish, french, german, greek, hungarian, italian, norwegian, portuguese, romanian, russian, spanish, swedish, tamil, turkish. An unrecognized language returns an error.
