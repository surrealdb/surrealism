# surrealism-slack

Slack Incoming Webhook integration for Surrealism.

```surql
DEFINE MODULE mod::slack AS f"bucket:/slack.surli";

RETURN mod::slack::send("https://hooks.slack.com/services/...", "hello from SurrealDB");
```

| Function | Signature | Description |
|---|---|---|
| `send` | `(webhook_url: string, text: string) -> string` | Plain-text message |

Requires `allow_functions = ["http::post"]` and `hooks.slack.com` in
`allow_net` (no port — see the comment in `surrealism.toml`). Non-2xx
responses surface as an `Err`.
