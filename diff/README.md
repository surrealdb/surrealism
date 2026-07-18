# surrealism-diff

Text diffing functions for Surrealism.

```surql
DEFINE MODULE mod::diff AS f"bucket:/diff.surli";

RETURN mod::diff::unified("a\nb\nc\n", "a\nx\nc\n");
RETURN mod::diff::ratio("a\nb\nc\n", "a\nx\nc\n"); -- 0.6666666666666666f
```

| Function | Signature | Description |
|---|---|---|
| `unified` | `(a: string, b: string) -> string` | Unified-diff (`diff -u` style) between `a` and `b` |
| `ratio` | `(a: string, b: string) -> float` | Similarity ratio between `a` and `b`, from `0.0` to `1.0` |
