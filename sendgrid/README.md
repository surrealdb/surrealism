# surrealism-sendgrid

SendGrid transactional email integration for Surrealism.

```surql
DEFINE MODULE mod::sendgrid AS f"bucket:/sendgrid.surli";

RETURN mod::sendgrid::send("SG.xxx", "from@example.com", "to@example.com", "Hello", "Hello from SurrealDB");
```

| Function | Signature | Description |
|---|---|---|
| `send` | `(api_key: string, from: string, to: string, subject: string, body: string) -> string` | Sends a plain-text email via the SendGrid API |

Requires `allow_functions = ["http::post"]` and `api.sendgrid.com` in
`allow_net`. Non-2xx responses surface as an `Err`.
