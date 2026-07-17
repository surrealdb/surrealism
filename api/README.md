# surrealism-api

Third-party API integrations for Surrealism. One `#[surrealism] mod` per
platform inside a single crate, so calls are namespaced as
`mod::api::<platform>::<fn>`.

```surql
DEFINE MODULE mod::api AS f"bucket:/api.surli";

RETURN mod::api::discord::send("https://discord.com/api/webhooks/...", "hello from SurrealDB");
RETURN mod::api::slack::send("https://hooks.slack.com/services/...", "hello from SurrealDB");
```

| Function | Signature | Description |
|---|---|---|
| `discord::send` | `(webhook_url: string, content: string) -> string` | Plain-text message via a Discord webhook |
| `discord::send_embed` | `(webhook_url: string, title: string, description: string, color: int) -> string` | Rich embed (colored card); `color` is a decimal RGB int, e.g. `16711680` for red |
| `slack::send` | `(webhook_url: string, text: string) -> string` | Plain-text message via a Slack Incoming Webhook |

## Capabilities

Needs `allow_functions = ["http::post"]` plus each platform's hostname in
`allow_net`, **listed without a port** — see the comment in
`surrealism.toml`.

## Errors

Non-2xx responses surface as an `Err` (that's how the host's `http::post`
behaves), not a status code.

## Adding another platform

Add another `#[surrealism] mod <platform> { ... }` block building that
platform's payload shape and calling `crate::post_json(url, body)`, then add
its hostname to `allow_net` in `surrealism.toml`.
