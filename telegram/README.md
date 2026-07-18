# surrealism-telegram

Telegram Bot API integration for Surrealism.

```surql
DEFINE MODULE mod::telegram AS f"bucket:/telegram.surli";

RETURN mod::telegram::send("123456:ABC-DEF", "987654321", "hello from SurrealDB");
```

| Function | Signature | Description |
|---|---|---|
| `send` | `(bot_token: string, chat_id: string, text: string) -> string` | Sends a plain-text message to a chat via a bot |
