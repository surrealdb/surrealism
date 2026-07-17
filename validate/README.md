# surrealism-validate

Lightweight format validation functions for Surrealism (not full RFC parsers — for
everyday input validation). Uses `regex` and `url`.

```surql
DEFINE MODULE mod::validate AS f"bucket:/validate.surli";

RETURN mod::validate::email("tobie@surrealdb.com"); -- true
RETURN mod::validate::iban("GB29NWBK60161331926819"); -- true
RETURN mod::validate::credit_card("4111111111111111"); -- true
```

| Function | Signature | Description |
|---|---|---|
| `email` | `(input: string) -> bool` | Looks like a valid email address |
| `url` | `(input: string) -> bool` | Valid `http(s)://` URL |
| `ipv4` | `(input: string) -> bool` | Valid IPv4 address |
| `ipv6` | `(input: string) -> bool` | Valid IPv6 address |
| `credit_card` | `(input: string) -> bool` | Passes the Luhn checksum |
| `iban` | `(input: string) -> bool` | Structurally valid IBAN (ISO 7064 mod-97) |
| `uuid` | `(input: string) -> bool` | Valid UUID (any version) |
| `phone` | `(input: string) -> bool` | Loose E.164-shaped check (not full libphonenumber) |
| `hex_color` | `(input: string) -> bool` | Valid `#rgb` or `#rrggbb` hex color |
