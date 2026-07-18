# surrealism-markdown

Markdown to HTML rendering for Surrealism.

```surql
DEFINE MODULE mod::markdown AS f"bucket:/markdown.surli";

RETURN mod::markdown::to_html("# Hi\n\n- [x] done\n- [ ] todo");
```

| Function | Signature | Description |
|---|---|---|
| `to_html` | `(input: string) -> string` | Renders markdown to HTML with tables, strikethrough, and task lists enabled |
