# surrealism-html

HTML sanitization and extraction functions for Surrealism. Uses `ammonia`
and `scraper`.

```surql
DEFINE MODULE mod::html AS f"bucket:/html.surli";

RETURN mod::html::sanitize("<script>alert(1)</script><p onclick='x'>hi</p>");
RETURN mod::html::title("<title>My Page</title>");
```

| Function | Signature | Description |
|---|---|---|
| `sanitize` | `(input: string) -> string` | Strips unsafe elements/attributes (scripts, event handlers, `javascript:` URLs, etc.) |
| `text` | `(input: string) -> string` | Visible text content, whitespace collapsed, `<script>`/`<style>` excluded |
| `title` | `(input: string) -> string \| none` | `<title>` content |
| `links` | `(input: string) -> array<string>` | Every `<a href>` target |
| `meta` | `(input: string, name: string) -> string \| none` | `<meta name="...">` or `<meta property="...">` content, matched case-insensitively (covers Open Graph tags) |
