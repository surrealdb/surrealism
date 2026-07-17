# surrealism-fake

Realistic fake data generation for Surrealism. Built on the [`fake`](https://docs.rs/fake)
crate plus `rand` and `uuid`.

```surql
DEFINE MODULE mod::fake AS f"bucket:/fake.surli";

RETURN mod::fake::name();      -- 'Broderick Gulgowski'
RETURN mod::fake::email();     -- 'rita_et@yahoo.com'
-- address: mod::fake::street_address(), city(), state(), zip_code()
```

| Function | Signature | Description |
|---|---|---|
| `name` / `first_name` / `last_name` | `() -> string` | Random person name |
| `email` | `() -> string` | Random email address |
| `username` | `() -> string` | Random username |
| `password` | `(length: int) -> string` | Random password of exactly `length` chars |
| `phone_number` | `() -> string` | Random phone number |
| `company` / `job_title` / `industry` | `() -> string` | Random company/profession/industry |
| `street_address` / `city` / `state` / `country` / `zip_code` | `() -> string` | Random address components |
| `latitude` / `longitude` | `() -> float` | Random coordinate |
| `word` | `() -> string` | A single lorem-ipsum word |
| `sentence` | `(word_count: int) -> string` | A sentence of exactly `word_count` words |
| `paragraph` | `(sentence_count: int) -> string` | A paragraph of exactly `sentence_count` sentences |
| `uuid` | `() -> string` | Random UUID v4 |
| `ipv4` / `ipv6` | `() -> string` | Random IP address |
| `user_agent` | `() -> string` | Random browser user-agent string |
| `color_hex` | `() -> string` | Random `#rrggbb` color |
| `boolean` | `() -> bool` | Random boolean |
| `number` | `(min: int, max: int) -> int` | Random integer in `[min, max]` |
| `date` | `() -> string` | Random `YYYY-MM-DD` date (1970-2024) |
| `credit_card_number` | `() -> string` | Random 16-digit Luhn-valid number (not a real card) |
