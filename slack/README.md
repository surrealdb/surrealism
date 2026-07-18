# surrealism-slack

Slack Incoming Webhook and Web API integration for Surrealism.

```surql
DEFINE MODULE mod::slack AS f"bucket:/slack.surli";

RETURN mod::slack::send("https://hooks.slack.com/services/...", "hello from SurrealDB");
RETURN mod::slack::post_message("xoxb-...", "C0123456789", "hello from SurrealDB");
```

| Function | Signature | Description |
|---|---|---|
| `send` | `(webhook_url: string, text: string) -> string` | Plain-text message via Incoming Webhook |
| `post_message` | `(token: string, channel: string, text: string) -> object` | Posts a message via the Slack Web API using a Bearer token |

Requires `allow_functions = ["http::post"]` and both `hooks.slack.com` and
`slack.com` in `allow_net` (no port — see the comment in `surrealism.toml`).
Non-2xx responses surface as an `Err`.

The Slack Web API returns HTTP 200 even on failure, signaling errors via
`"ok": false` in the JSON body. `post_message` checks this field: when `ok` is
not `true`, it returns an `Err` containing the response's `error` string (or
`"unknown_error"` if that field is absent).
