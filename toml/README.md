# surrealism-toml

TOML/JSON conversion functions for Surrealism.

```surql
DEFINE MODULE mod::toml AS f"bucket:/toml.surli";

RETURN mod::toml::to_json("a = 1\n[b]\nc = 2\n"); -- { a: 1, b: { c: 2 } }
RETURN mod::toml::from_json({ a: 1, b: { c: 2 } }); -- "a = 1\n\n[b]\nc = 2\n"
```

| Function | Signature | Description |
|---|---|---|
| `to_json` | `(toml: string) -> value` | Parses a TOML document into a value |
| `from_json` | `(value: value) -> string` | Serializes a value into a TOML document |

`from_json` requires an object at the top level and fails on any value containing null.
