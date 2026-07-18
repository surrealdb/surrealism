# surrealism-mastodon

Mastodon API integration for Surrealism.

```surql
DEFINE MODULE mod::mastodon AS f"bucket:/mastodon.surli";

RETURN mod::mastodon::post_status("https://mastodon.social", "your-access-token", "Hello, fediverse!");
```

| Function | Signature | Description |
|---|---|---|
| `post_status` | `(instance_url: string, access_token: string, status: string) -> object` | Publishes a status on the given Mastodon instance |

`instance_url` is the full base URL of the caller's Mastodon instance (e.g.
`https://mastodon.social` or `https://fosstodon.org`), since Mastodon is
federated and every user/community runs their own instance at its own
hostname.

Requires `allow_functions = ["http::post"]` in `surrealism.toml`, which is
shipped by default. `allow_net` ships as an explicit empty list (`allow_net =
[]`), so all network access is denied until the deployer edits it to add
their own instance's hostname, e.g. `allow_net = ["mastodon.social"]`. This
edit is required before the module can make any request. Non-2xx responses
surface as an `Err`.
