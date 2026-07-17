# Surrealism Modules

Open-source, independently-licensed (MIT OR Apache-2.0) [Surrealism](https://surrealdb.com/docs/learn/extensions/guides/creating-custom-modules)
WASM modules for [SurrealDB](https://surrealdb.com). Each module in this repo
is its own standalone Cargo project — own `Cargo.toml`, own dependency tree,
own license files, buildable and versionable independently of the others.

| Module | Crate | What it does |
|---|---|---|
| [`color`](color/) | `surrealism-color` | Color conversion, manipulation, and WCAG accessibility checks |
| [`fake`](fake/) | `surrealism-fake` | Realistic fake data generation (names, addresses, lorem ipsum, etc.) |
| [`hash`](hash/) | `surrealism-hash` | Hashing, HMAC, and base64/hex encoding |
| [`image`](image/) | `surrealism-image` | Image resize, thumbnail, format conversion, filters |
| [`text`](text/) | `surrealism-text` | String case conversion, slugify, Levenshtein, HTML stripping |
| [`validate`](validate/) | `surrealism-validate` | Format validation: email, URL, IBAN, credit card, phone, etc. |
| [`webhook`](webhook/) | `surrealism-webhook` | Send HTTP/HTTPS events (Slack, Discord, Teams, generic webhooks) |
| [`kafka`](kafka/) | `surrealism-kafka` | Minimal plaintext Kafka producer |

## Building

Each module builds independently with the [`surreal` CLI](https://surrealdb.com/docs/learn/extensions/guides/creating-custom-modules):

```bash
cd <name>
surreal module build --debug -o <name>.surli .
surreal module info <name>.surli
surreal module run --fnc <function> --arg <value> <name>.surli
```

`webhook` additionally needs a [WASI SDK](#https-requires-a-wasi-sdk) on
`WASI_SDK_PATH` to build (see below); the rest have no such requirement.

## Using from SurrealQL

Module names in `DEFINE MODULE` and in calls are prefixed with the literal
`mod::`:

```surql
DEFINE BUCKET modules BACKEND "file:/path/to/built/surli/files";
DEFINE MODULE mod::color AS f"modules:/color.surli";

RETURN mod::color::hex_to_rgb("#3366ff");
```

## Guest sandboxing: what these modules can and can't reach

Surrealism modules run inside a WASI sandbox with a capabilities model
declared per-module in `surrealism.toml` (`allow_net`, `allow_functions`,
etc.), enforced by the host at connect time — see each module's own
`surrealism.toml` for the specific grants it needs and why.

One constraint applies across all of them and is worth understanding up
front: **guest code cannot resolve hostnames.** WASI's `ip-name-lookup` is
intentionally disabled at the runtime level to prevent DNS-tunnelling data
exfiltration, so any function that opens a raw socket directly from the guest
(`webhook::post`, `webhook::get`, `kafka::produce`) requires the target to be
given as a literal IP address, not a hostname.

`webhook` also ships a second path, `post_via_host`, for the common case of
needing to reach an actual hostname (Slack, Discord, Teams, or anything
behind a CDN with rotating IPs): it delegates the request to the SurrealDB
host's own native `http::post` function via `surrealism::run(...)`, since the
host has full, unsandboxed DNS and TLS. See that module's README for the
capability configuration this needs (it's a little unintuitive: the allowed
hostname must be listed **without** a port).

### HTTPS requires a WASI SDK

`webhook`'s `https://` support pulls in `rustls`, which needs `ring` for its
crypto, and `ring` cross-compiles a small amount of C for `wasm32-wasip2` —
which needs a real WASI-targeting C toolchain, not just cargo. Without one,
building `webhook` fails inside `ring`'s build script with something like
`unable to create target: 'No available targets are compatible with triple
"wasm32-unknown-wasip2"'`.

To fix it, install a [WASI SDK](https://github.com/WebAssembly/wasi-sdk/releases)
(verified against wasi-sdk-33.0) and point `WASI_SDK_PATH` at it for the build:

```bash
curl -L -o wasi-sdk.tar.gz https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-33/wasi-sdk-33.0-<arch>-<os>.tar.gz
mkdir -p ~/wasi-sdk-33.0 && tar -xzf wasi-sdk.tar.gz -C ~/wasi-sdk-33.0 --strip-components=1

cd webhook
WASI_SDK_PATH=~/wasi-sdk-33.0 surreal module build --debug -o webhook.surli .
```

(`cc`, the crate `ring`'s build script uses, looks for
`$WASI_SDK_PATH/bin/wasm32-wasip2-clang` automatically — no further
configuration needed.) The other modules here don't touch TLS and build fine
without any of this.

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
