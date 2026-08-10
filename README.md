# Surrealism Modules

Open-source, independently-licensed (MIT OR Apache-2.0) [Surrealism](https://surrealdb.com/docs/learn/extensions/guides/creating-custom-modules)
WASM modules for [SurrealDB](https://surrealdb.com). Each module in this repo
is its own standalone Cargo project — own `Cargo.toml`, own dependency tree,
own license files, buildable and versionable independently of the others.

| Module | What it does |
|---|---|
| [`aes`](aes/) | AES-256-GCM authenticated encryption and decryption |
| [`airtable`](airtable/) | Airtable record creation, listing, and retrieval |
| [`anthropic`](anthropic/) | Anthropic Claude message generation |
| [`barcode`](barcode/) | Code128 and EAN-13 barcode generation (PNG) |
| [`base58`](base58/) | Base58 and Base58Check encoding/decoding |
| [`bluesky`](bluesky/) | Bluesky (AT Protocol) session login and posting |
| [`chunk`](chunk/) | Text chunking for RAG (fixed, sentence, paragraph, recursive, markdown) |
| [`color`](color/) | Color conversion, manipulation, and WCAG accessibility checks |
| [`compress`](compress/) | Gzip compression |
| [`cron`](cron/) | Cron expression parsing and next-occurrence computation |
| [`csv`](csv/) | CSV parsing and writing |
| [`diff`](diff/) | Text diffing (unified diff, similarity ratio) |
| [`discord`](discord/) | Discord webhook messages and embeds |
| [`ed25519`](ed25519/) | Ed25519 key generation, signing, and verification |
| [`emoji`](emoji/) | Emoji shortcode ↔ unicode conversion |
| [`fake`](fake/) | Realistic fake data generation (names, addresses, lorem ipsum, etc.) |
| [`gemini`](gemini/) | Google Gemini embeddings and text generation |
| [`github`](github/) | GitHub issue and comment creation/retrieval |
| [`googlesheets`](googlesheets/) | Google Sheets row append and range read |
| [`hash`](hash/) | Hashing, HMAC, and base64/hex encoding |
| [`hibp`](hibp/) | Have I Been Pwned k-anonymity password breach check |
| [`html`](html/) | HTML sanitization and extraction (text, title, links, meta) |
| [`ical`](ical/) | iCalendar (.ics) event generation |
| [`image`](image/) | Image resize, thumbnail, format conversion, filters |
| [`jwt`](jwt/) | JSON Web Token (HS256) encoding and decoding |
| [`kafka`](kafka/) | Minimal plaintext Kafka producer |
| [`lang`](lang/) | Language detection |
| [`linear`](linear/) | Linear issue creation and retrieval |
| [`markdown`](markdown/) | Markdown to HTML rendering |
| [`mastodon`](mastodon/) | Mastodon status posting |
| [`mimetype`](mimetype/) | File type detection from magic bytes |
| [`msgpack`](msgpack/) | MessagePack binary serialization |
| [`notion`](notion/) | Notion page creation, retrieval, and database querying |
| [`ntfy`](ntfy/) | ntfy.sh push notifications |
| [`onesignal`](onesignal/) | OneSignal push notifications |
| [`openai`](openai/) | OpenAI embeddings and chat completions |
| [`pagerduty`](pagerduty/) | PagerDuty Events API incident triggering/resolving |
| [`password`](password/) | Password hashing and verification (Argon2) |
| [`phonenumber`](phonenumber/) | Phone number parsing, validation, and E.164 formatting |
| [`postmark`](postmark/) | Postmark transactional email |
| [`qrcode`](qrcode/) | QR code generation (PNG or SVG) |
| [`semver`](semver/) | Semantic version parsing and comparison |
| [`sendgrid`](sendgrid/) | SendGrid transactional email |
| [`slack`](slack/) | Slack Incoming Webhook messages |
| [`stem`](stem/) | Word stemming (Snowball algorithms) |
| [`teams`](teams/) | Microsoft Teams webhook messages |
| [`telegram`](telegram/) | Telegram Bot API messages |
| [`text`](text/) | String case conversion, slugify, Levenshtein, HTML stripping |
| [`toml`](toml/) | TOML to/from JSON conversion |
| [`totp`](totp/) | TOTP (RFC 6238) two-factor authentication codes |
| [`twilio`](twilio/) | Twilio SMS messages |
| [`validate`](validate/) | Format validation: email, URL, IBAN, credit card, phone, etc. |
| [`voyage`](voyage/) | Voyage AI embeddings |
| [`xml`](xml/) | XML to JSON conversion |
| [`yaml`](yaml/) | YAML to/from JSON conversion |
| [`zip`](zip/) | ZIP archive creation and extraction |

## Building a RAG pipeline

`chunk` splits documents, the embedding modules turn chunks into vectors, and
SurrealDB's built-in [`vector::`](https://surrealdb.com/docs/surrealql/functions/database/vector)
functions do the similarity search:

```surql
FOR $piece IN mod::chunk::recursive($doc, 500, 1) {
    CREATE chunk SET
        text = $piece,
        embedding = mod::openai::embed($key, "text-embedding-3-small", $piece);
};

LET $query = mod::openai::embed($key, "text-embedding-3-small", "how do I ...?");
SELECT text, vector::similarity::cosine(embedding, $query) AS score
    FROM chunk ORDER BY score DESC LIMIT 5;
```

## Building

Each module builds independently with the [`surreal` CLI](https://surrealdb.com/docs/learn/extensions/guides/creating-custom-modules):

```bash
cd <name>
surreal module build --debug -o <name>.surli .
surreal module info <name>.surli
surreal module run --fnc <function> --arg <value> <name>.surli
```

The toolchain, target and lint components are pinned in `rust-toolchain.toml`,
and each module's dependencies are pinned by its own committed `Cargo.lock`.

## Testing

Each module has a `tests.toml` listing its cases, run by a shared script:

```bash
python3 scripts/run-module-tests.py <name> --build
```

Functions that reach the network delegate to a host function, and the CLI
intercepts those: it prints the resolved call and reads a mocked response from
stdin. A case supplies that response in its `stdin` field, so modules like
`slack` and `openai` are tested without a server, credentials, or network
access. See the script's docstring for the full manifest schema.

CI runs the same script, plus `cargo clippy -D warnings` and
`cargo fmt --check`, against every module on each push and pull request.

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
`github`, `pagerduty`, `twilio`, `ntfy`, `linear`, `notion`, `airtable`,
`mastodon`, `bluesky`, `onesignal`, `hibp`, `googlesheets`, `openai`,
`voyage`, `gemini`, and `anthropic` instead delegate to the host's
`http::get`/`http::post`, which can resolve hostnames — see each module's
README for its capability setup.

`mastodon` talks to a federated network with no single fixed hostname, so
its `surrealism.toml` ships with `allow_net = []`; add your own instance's
hostname before deploying it.

## Dependencies

Each module depends on the published [`surrealism`](https://crates.io/crates/surrealism)
and [`surrealdb-types`](https://crates.io/crates/surrealdb-types) crates from
crates.io.

## License

Every module is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE),
at your option, matching the copies inside each module's own directory (kept
in sync so a module can be split into its own repo later without losing its
license files).
