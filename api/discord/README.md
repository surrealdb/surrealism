# surrealism-discord

Discord webhook integration for Surrealism.

```surql
DEFINE MODULE mod::discord AS f"bucket:/discord.surli";

RETURN mod::discord::send("https://discord.com/api/webhooks/...", "hello from SurrealDB");
```

| Function | Signature | Description |
|---|---|---|
| `send` | `(webhook_url: string, content: string) -> string` | Plain-text message |
| `send_embed` | `(webhook_url: string, title: string, description: string, color: int) -> string` | Rich embed (colored card); `color` is a decimal RGB int, e.g. `16711680` for red |

Requires `allow_functions = ["http::post"]` and `discord.com` in `allow_net`
(no port — see the comment in `surrealism.toml`). Non-2xx responses surface
as an `Err`.
