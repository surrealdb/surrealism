# surrealism-postmark

Postmark transactional email integration for Surrealism.

```surql
DEFINE MODULE mod::postmark AS f"bucket:/postmark.surli";

RETURN mod::postmark::send(
	"POSTMARK_SERVER_TOKEN",
	"sender@example.com",
	"recipient@example.com",
	"Hello",
	"This is a test email"
);
```

| Function | Signature | Description |
|---|---|---|
| `send` | `(server_token: string, from: string, to: string, subject: string, body: string) -> string` | Sends a transactional email via the Postmark API |
