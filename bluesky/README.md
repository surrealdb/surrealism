# surrealism-bluesky

Bluesky / AT Protocol integration for Surrealism.

```surql
DEFINE MODULE mod::bluesky AS f"bucket:/bluesky.surli";

LET $session = mod::bluesky::create_session("handle.bsky.social", "app-password");
RETURN mod::bluesky::post($session.accessJwt, $session.did, "Hello, Bluesky!");
```

| Function | Signature | Description |
|---|---|---|
| `create_session` | `(identifier: string, app_password: string) -> object` | Creates an AT Protocol session against `bsky.social` |
| `post` | `(access_jwt: string, did: string, text: string) -> object` | Publishes a text post to the authenticated account's repo |

`create_session` returns the full `com.atproto.server.createSession` response,
including `accessJwt` and `did`, which are passed into `post`. `post` sets
`createdAt` to the current UTC time.

Requires `allow_functions = ["http::post"]` and `bsky.social` in `allow_net`.
Self-hosted PDS users need to adjust `allow_net` to their own PDS hostname.
Non-2xx responses surface as an `Err`.
