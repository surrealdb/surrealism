# surrealism-emoji

Emoji shortcode and unicode conversion for Surrealism.

```surql
DEFINE MODULE mod::emoji AS f"bucket:/emoji.surli";

RETURN mod::emoji::to_unicode(":smile:");
RETURN mod::emoji::to_shortcode("😄");
RETURN mod::emoji::replace_shortcodes("Hello :smile: World :fire:!");
```

| Function | Signature | Description |
|---|---|---|
| `to_unicode` | `(shortcode: string) -> string \| none` | Unicode emoji for a shortcode, with or without surrounding colons |
| `to_shortcode` | `(emoji: string) -> string \| none` | Canonical shortcode (without colons) for a unicode emoji |
| `replace_shortcodes` | `(text: string) -> string` | Replaces every `:shortcode:` in `text` with its unicode emoji |

`to_unicode` and `replace_shortcodes` use a static emoji database compiled into the module; no network access is required. Unrecognized shortcodes are left untouched by `replace_shortcodes`.
