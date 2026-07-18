# surrealism-csv

CSV parsing and writing functions for Surrealism.

```surql
DEFINE MODULE mod::csv AS f"bucket:/csv.surli";

RETURN mod::csv::parse("a,b\n1,2");                 -- [["a", "b"], ["1", "2"]]
RETURN mod::csv::parse_objects("a,b\n1,2");          -- [{ a: "1", b: "2" }]
RETURN mod::csv::stringify([["a", "b"], ["1", "2"]]); -- 'a,b\n1,2\n'
```

| Function | Signature | Description |
|---|---|---|
| `parse` | `(input: string) -> array<array<string>>` | Parses CSV into rows of raw string cells |
| `parse_objects` | `(input: string) -> array<object>` | Parses CSV with a header row into an array of objects |
| `stringify` | `(rows: array<array<string>>) -> string` | Writes rows back out as CSV text |
