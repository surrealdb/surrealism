# surrealism-webhook

Send HTTP/HTTPS webhook events from Surrealism — Slack, Discord, Teams, generic
webhook receivers, etc.

```surql
DEFINE MODULE mod::webhook AS f"bucket:/webhook.surli";

RETURN mod::webhook::post_via_host("https://hooks.slack.com/services/...", '{"text":"hi"}');
```

## Two ways to send a request

| Function | Signature | Connects via | Use when |
|---|---|---|---|
| `post` | `(url: string, body: string) -> [int, string]` | Raw `TcpStream` (+ hand-driven `rustls` for `https://`), directly from the guest | Target is a literal **IP address** (guest code cannot resolve hostnames) |
| `post_with_headers` | `(url: string, body: string, headers: array<[string, string]>) -> [int, string]` | Same as `post`, with custom headers | Same as `post`, plus custom auth/headers |
| `get` | `(url: string) -> [int, string]` | Same as `post` | Same as `post` |
| `post_via_host` | `(url: string, json_body: string) -> string` | Delegates to the SurrealDB host's native `http::post` | Target is a **hostname** (Slack, Discord, Teams, any CDN-backed service) |

### Why two paths?

WASI's `ip-name-lookup` capability is intentionally disabled in the Surrealism
runtime to prevent DNS-tunnelling data exfiltration — guest code can open a raw
socket to an allow-listed **IP**, but it cannot resolve a **hostname** itself.
Since real webhook targets (Slack, Discord, etc.) are always addressed by
hostname and served from rotating/CDN IPs, `post_via_host` instead asks the
*host* (which has full, unsandboxed DNS + TLS) to make the request via
SurrealDB's built-in `http::post`, and just hands back the response body.

`post`/`post_with_headers`/`get` remain useful for internal services with a
stable, known IP, and don't need any extra server-side capability beyond
`allow_net`.

### Capabilities

- `post` / `post_with_headers` / `get` need the target `ip:port` listed in
  `allow_net`.
- `post_via_host` needs `allow_functions = ["http::post"]`, and needs the
  target **hostname listed with no port** in `allow_net` (e.g.
  `"hooks.slack.com"`, not `"hooks.slack.com:443"`) — see the comment in
  `surrealism.toml` for why the port must be omitted here specifically.
- Either way, the server itself must also be started with the target allowed
  via its own `--allow-net`.

## Limitations

- `post`/`get` don't handle `Transfer-Encoding: chunked` responses (the raw
  chunk framing would show up in the body) — fine for typical small webhook
  acknowledgement bodies, not for arbitrary HTTP.
- `post_via_host` surfaces non-2xx responses as an `Err` rather than a status
  code (that's how the host's `http::post` behaves).
