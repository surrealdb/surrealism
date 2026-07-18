# surrealism-teams

Microsoft Teams incoming webhook integration for Surrealism.

```surql
DEFINE MODULE mod::teams AS f"bucket:/teams.surli";

RETURN mod::teams::send("https://outlook.office.com/webhook/...", "hello from SurrealDB");
```

| Function | Signature | Description |
|---|---|---|
| `send` | `(webhook_url: string, text: string) -> string` | Plain-text message via a Teams incoming webhook |

Requires `allow_functions = ["http::post"]` and the webhook's hostname
(e.g. `outlook.office.com`) in `allow_net`. Non-2xx responses surface as an
`Err`.
