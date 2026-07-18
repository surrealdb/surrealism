# surrealism-ntfy

ntfy.sh push notification integration for Surrealism.

```surql
DEFINE MODULE mod::ntfy AS f"bucket:/ntfy.surli";

RETURN mod::ntfy::send("mytopic", "Hello");
RETURN mod::ntfy::send_titled("mytopic", "Alert", "Hello");
```

| Function | Signature | Description |
|---|---|---|
| `send` | `(topic: string, message: string) -> value` | Sends a plain-text push notification to an ntfy.sh topic |
| `send_titled` | `(topic: string, title: string, message: string) -> value` | Sends a titled plain-text push notification to an ntfy.sh topic |

Requires `allow_functions = ["http::post"]` and `ntfy.sh` in `allow_net`.
Non-2xx responses surface as an `Err`.
