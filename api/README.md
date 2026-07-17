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

## Why this delegates to the host

Discord and Slack webhook URLs are always hostname-addressed
(`discord.com`, `hooks.slack.com`), and guest code cannot resolve hostnames
itself — WASI's `ip-name-lookup` is intentionally disabled at the runtime
level to prevent DNS-tunnelling data exfiltration. So instead of opening a
raw socket from inside the WASM guest (which would only work for a literal
IP target), every function here builds the platform's JSON payload and hands
the actual request to the SurrealDB host's native `http::post` via
`surrealism::run(...)` — the host has full, unsandboxed DNS and TLS.

This needs `allow_functions = ["http::post"]` plus each platform's hostname
in `allow_net`, **listed without a port** (see the comment in
`surrealism.toml` — the host's outbound HTTP client's DNS resolver checks the
bare hostname before resolving, so a port-qualified entry is silently
rejected even though it looks more precise).

## Errors

Non-2xx responses surface as an `Err` (that's how the host's `http::post`
behaves), not a status code.

## Adding another platform

Add another `#[surrealism] mod <platform> { ... }` block building that
platform's payload shape and calling `crate::post_json(url, body)`, then add
its hostname to `allow_net` in `surrealism.toml`.
