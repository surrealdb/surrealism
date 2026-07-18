# Surrealism Modules

Open-source, independently-licensed (MIT OR Apache-2.0) [Surrealism](https://surrealdb.com/docs/learn/extensions/guides/creating-custom-modules)
WASM modules for [SurrealDB](https://surrealdb.com). Each module in this repo
is its own standalone Cargo project — own `Cargo.toml`, own dependency tree,
own license files, buildable and versionable independently of the others.

| Module | Crate | What it does |
|---|---|---|
| [`aes`](aes/) | `surrealism-aes` | AES-256-GCM authenticated encryption and decryption |
| [`base58`](base58/) | `surrealism-base58` | Base58 and Base58Check encoding/decoding |
| [`color`](color/) | `surrealism-color` | Color conversion, manipulation, and WCAG accessibility checks |
| [`compress`](compress/) | `surrealism-compress` | Gzip compression |
| [`cron`](cron/) | `surrealism-cron` | Cron expression parsing and next-occurrence computation |
| [`csv`](csv/) | `surrealism-csv` | CSV parsing and writing |
| [`diff`](diff/) | `surrealism-diff` | Text diffing (unified diff, similarity ratio) |
| [`discord`](discord/) | `surrealism-discord` | Discord webhook messages and embeds |
| [`ed25519`](ed25519/) | `surrealism-ed25519` | Ed25519 key generation, signing, and verification |
| [`fake`](fake/) | `surrealism-fake` | Realistic fake data generation (names, addresses, lorem ipsum, etc.) |
| [`github`](github/) | `surrealism-github` | GitHub issue and comment creation/retrieval |
| [`hash`](hash/) | `surrealism-hash` | Hashing, HMAC, and base64/hex encoding |
| [`html`](html/) | `surrealism-html` | HTML sanitization and extraction (text, title, links, meta) |
| [`ical`](ical/) | `surrealism-ical` | iCalendar (.ics) event generation |
| [`image`](image/) | `surrealism-image` | Image resize, thumbnail, format conversion, filters |
| [`jwt`](jwt/) | `surrealism-jwt` | JSON Web Token (HS256) encoding and decoding |
| [`kafka`](kafka/) | `surrealism-kafka` | Minimal plaintext Kafka producer |
| [`lang`](lang/) | `surrealism-lang` | Language detection |
| [`markdown`](markdown/) | `surrealism-markdown` | Markdown to HTML rendering |
| [`mimetype`](mimetype/) | `surrealism-mimetype` | File type detection from magic bytes |
| [`msgpack`](msgpack/) | `surrealism-msgpack` | MessagePack binary serialization |
| [`ntfy`](ntfy/) | `surrealism-ntfy` | ntfy.sh push notifications |
| [`pagerduty`](pagerduty/) | `surrealism-pagerduty` | PagerDuty Events API incident triggering/resolving |
| [`password`](password/) | `surrealism-password` | Password hashing and verification (Argon2) |
| [`phonenumber`](phonenumber/) | `surrealism-phonenumber` | Phone number parsing, validation, and E.164 formatting |
| [`postmark`](postmark/) | `surrealism-postmark` | Postmark transactional email |
| [`qrcode`](qrcode/) | `surrealism-qrcode` | QR code generation (PNG or SVG) |
| [`semver`](semver/) | `surrealism-semver` | Semantic version parsing and comparison |
| [`sendgrid`](sendgrid/) | `surrealism-sendgrid` | SendGrid transactional email |
| [`slack`](slack/) | `surrealism-slack` | Slack Incoming Webhook messages |
| [`stem`](stem/) | `surrealism-stem` | Word stemming (Snowball algorithms) |
| [`teams`](teams/) | `surrealism-teams` | Microsoft Teams webhook messages |
| [`telegram`](telegram/) | `surrealism-telegram` | Telegram Bot API messages |
| [`text`](text/) | `surrealism-text` | String case conversion, slugify, Levenshtein, HTML stripping |
| [`totp`](totp/) | `surrealism-totp` | TOTP (RFC 6238) two-factor authentication codes |
| [`twilio`](twilio/) | `surrealism-twilio` | Twilio SMS messages |
| [`validate`](validate/) | `surrealism-validate` | Format validation: email, URL, IBAN, credit card, phone, etc. |
| [`xml`](xml/) | `surrealism-xml` | XML to JSON conversion |
| [`yaml`](yaml/) | `surrealism-yaml` | YAML to/from JSON conversion |
| [`zip`](zip/) | `surrealism-zip` | ZIP archive creation and extraction |

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

## Guest sandboxing

Surrealism modules run inside a WASI sandbox with a capabilities model
declared per-module in `surrealism.toml` (`allow_net`, `allow_functions`,
etc.), enforced by the host at connect time — see each module's own
`surrealism.toml` for the specific grants it needs.

Guest code cannot resolve hostnames, so any function that opens a socket
directly from the guest (`kafka::produce`) needs a literal IP address, not a
hostname. `discord`, `slack`, `teams`, `telegram`, `sendgrid`, `postmark`,
`github`, `pagerduty`, `twilio`, and `ntfy` instead delegate to the host's
`http::get`/`http::post`, which can resolve hostnames — see each module's
README for its capability setup.

## Dependencies

Each module depends on the published [`surrealism`](https://crates.io/crates/surrealism)
and [`surrealdb-types`](https://crates.io/crates/surrealdb-types) crates from
crates.io.

## License

Every module is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE),
at your option, matching the copies inside each module's own directory (kept
in sync so a module can be split into its own repo later without losing its
license files).
