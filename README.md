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
| [`html`](html/) | `surrealism-html` | HTML sanitization and extraction (text, title, links, meta) |
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

## Guest sandboxing

Surrealism modules run inside a WASI sandbox with a capabilities model
declared per-module in `surrealism.toml` (`allow_net`, `allow_functions`,
etc.), enforced by the host at connect time — see each module's own
`surrealism.toml` for the specific grants it needs.

Guest code cannot resolve hostnames, so any function that opens a socket
directly from the guest (`kafka::produce`) needs a literal IP address, not a
hostname. `api`'s integrations instead delegate to the host's `http::post`,
which can resolve hostnames — see that module's README for its capability
setup.

## Dependencies

Each module depends on the published [`surrealism`](https://crates.io/crates/surrealism)
and [`surrealdb-types`](https://crates.io/crates/surrealdb-types) crates from
crates.io.

## License

Every module is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE),
at your option, matching the copies inside each module's own directory (kept
in sync so a module can be split into its own repo later without losing its
license files).
