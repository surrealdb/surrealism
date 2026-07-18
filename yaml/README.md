# surrealism-yaml

YAML/JSON conversion functions for Surrealism.

```surql
DEFINE MODULE mod::yaml AS f"bucket:/yaml.surli";

RETURN mod::yaml::to_json("a: 1\nb:\n  - 1\n  - 2\n"); -- { a: 1, b: [1, 2] }
RETURN mod::yaml::from_json({ a: 1, b: [1, 2] });      -- "a: 1\nb:\n- 1\n- 2\n"
```

| Function | Signature | Description |
|---|---|---|
| `to_json` | `(yaml: string) -> value` | Parses a YAML document into a value |
| `from_json` | `(value: value) -> string` | Serializes a value into a YAML document |
