# surrealism-twilio

Twilio SMS integration for Surrealism.

```surql
DEFINE MODULE mod::twilio AS f"bucket:/twilio.surli";

RETURN mod::twilio::send("ACxxx", "auth_token", "+15551234567", "+15557654321", "Hello from SurrealDB");
```

| Function | Signature | Description |
|---|---|---|
| `send` | `(account_sid: string, auth_token: string, from: string, to: string, body: string) -> value` | Sends an SMS via the Twilio Messages API |

Requires `allow_functions = ["http::post"]` and `api.twilio.com` in
`allow_net`. Authenticates with HTTP Basic Auth (`account_sid`/`auth_token`)
and sends the request body as `application/x-www-form-urlencoded`, not JSON.
The response is the Twilio Message resource, decoded as JSON.
