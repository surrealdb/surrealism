# Surrealism Modules

Open-source, independently-licensed (MIT OR Apache-2.0) [Surrealism](https://surrealdb.com/docs/learn/extensions/guides/creating-custom-modules)
WASM modules for [SurrealDB](https://surrealdb.com). Each module in this repo
is its own standalone Cargo project — own `Cargo.toml`, own dependency tree,
own license files, buildable and versionable independently of the others.

| Module | Crate | What it does |
|---|---|---|
| [`api`](api/) | `surrealism-api` | Third-party API integrations: `api::discord`, `api::slack`, ... |
| [`color`](color/) | `surrealism-color` | Color conversion, manipulation, and WCAG accessibility checks |
| [`fake`](fake/) | `surrealism-fake` | Realistic fake data generation (names, addresses, lorem ipsum, etc.) |
| [`hash`](hash/) | `surrealism-hash` | Hashing, HMAC, and base64/hex encoding |
| [`image`](image/) | `surrealism-image` | Image resize, thumbnail, format conversion, filters |
| [`text`](text/) | `surrealism-text` | String case conversion, slugify, Levenshtein, HTML stripping |
| [`validate`](validate/) | `surrealism-validate` | Format validation: email, URL, IBAN, credit card, phone, etc. |
| [`kafka`](kafka/) | `surrealism-kafka` | Minimal plaintext Kafka producer |

## Building

Each module builds independently with the [`surreal` CLI](https://surrealdb.com/docs/learn/extensions/guides/creating-custom-modules):

```bash
cd <name>
surreal module build --debug -o <name>.surli .
surreal module info <name>.surli
surreal module run --fnc <function> --arg <value> <name>.surli
```

## Using from SurrealQL

Module names in `DEFINE MODULE` and in calls are prefixed with the literal
`mod::`:

```surql
DEFINE BUCKET modules BACKEND "file:/path/to/built/surli/files";
DEFINE MODULE mod::color AS f"modules:/color.surli";

RETURN mod::color::hex_to_rgb("#3366ff");
```

`api` groups its integrations as nested `#[surrealism] mod`s inside one
crate, so its functions are addressed with an extra segment:
`mod::api::discord::send(...)`, `mod::api::slack::send(...)`.

## Guest sandboxing: what these modules can and can't reach

Surrealism modules run inside a WASI sandbox with a capabilities model
declared per-module in `surrealism.toml` (`allow_net`, `allow_functions`,
etc.), enforced by the host at connect time — see each module's own
`surrealism.toml` for the specific grants it needs and why.

One constraint applies across all of them and is worth understanding up
front: **guest code cannot resolve hostnames.** WASI's `ip-name-lookup` is
intentionally disabled at the runtime level to prevent DNS-tunnelling data
exfiltration, so any function that opens a raw socket directly from the guest
(`kafka::produce`) requires the target to be given as a literal IP address,
not a hostname.

`api`'s integrations sidestep this: since Discord, Slack, and friends are
always hostname-addressed (and often behind a CDN with rotating IPs), each
one builds its JSON payload and delegates the actual request to the
SurrealDB host's own native `http::post` function via `surrealism::run(...)`,
since the host has full, unsandboxed DNS and TLS. See that module's README
for the capability configuration this needs (it's a little unintuitive: the
allowed hostname must be listed **without** a port). `kafka` has no
equivalent escape hatch (there's no built-in `kafka::produce` to delegate
to), so its `broker` argument is stuck being IP-only.

## Dependencies

Each module depends on the published [`surrealism`](https://crates.io/crates/surrealism)
and [`surrealdb-types`](https://crates.io/crates/surrealdb-types) crates from
crates.io — there's no path or git dependency on the main SurrealDB repo, so
every module here builds standalone from a fresh checkout.

## License

Every module is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE),
at your option, matching the copies inside each module's own directory (kept
in sync so a module can be split into its own repo later without losing its
license files).
