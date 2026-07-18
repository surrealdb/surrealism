# surrealism-cron

Cron expression parsing and next-occurrence computation for Surrealism.

```surql
DEFINE MODULE mod::cron AS f"bucket:/cron.surli";

RETURN mod::cron::is_valid("0 0 * * *");                            -- true
RETURN mod::cron::next("0 0 * * *", "2026-08-01T09:00:00Z");        -- "2026-08-02T00:00:00+00:00"
```

| Function | Signature | Description |
|---|---|---|
| `is_valid` | `(expression: string) -> bool` | Whether the cron expression parses successfully |
| `next` | `(expression: string, from: string) -> string` | Next occurrence of the expression strictly after `from` |

`expression` is a standard 5-field cron expression (minute hour day month weekday).

`from` and the returned occurrence are RFC3339 datetime strings; `from` is converted to UTC before computing the next occurrence.
