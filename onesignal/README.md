# surrealism-onesignal

OneSignal push notification integration for Surrealism.

```surql
DEFINE MODULE mod::onesignal AS f"bucket:/onesignal.surli";

RETURN mod::onesignal::send("os_v2_app_xxx", "app-id", "Hello from SurrealDB", "Subscribed Users");
```

| Function | Signature | Description |
|---|---|---|
| `send` | `(api_key: string, app_id: string, message: string, segment: string) -> object` | Sends a push notification to a segment via the OneSignal API |

Every request sends `Authorization: Key {api_key}`.

Requires `allow_functions = ["http::post"]` and `api.onesignal.com` in
`allow_net`. Non-2xx responses surface as an `Err`.
