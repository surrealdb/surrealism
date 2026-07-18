# surrealism-pagerduty

PagerDuty Events API v2 integration for Surrealism.

```surql
DEFINE MODULE mod::pagerduty AS f"bucket:/pagerduty.surli";

RETURN mod::pagerduty::trigger("R0ABC...", "Database is down", "prod-db-1", "critical");
RETURN mod::pagerduty::resolve("R0ABC...", "some-dedup-key");
```

| Function | Signature | Description |
|---|---|---|
| `trigger` | `(routing_key: string, summary: string, source: string, severity: string) -> object` | Triggers a new incident via the Events API v2 |
| `resolve` | `(routing_key: string, dedup_key: string) -> object` | Resolves an existing incident by its dedup key |

`routing_key` is the integration key from a PagerDuty service's Events API v2
integration; it is the sole authentication, there is no separate auth header.
`severity` must be one of `"critical"`, `"error"`, `"warning"`, or `"info"`.

Requires `allow_functions = ["http::post"]` and `events.pagerduty.com` in
`allow_net`. Non-2xx responses surface as an `Err`.
