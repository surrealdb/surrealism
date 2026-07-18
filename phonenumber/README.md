# surrealism-phonenumber

Phone number parsing, validation and formatting for Surrealism.

```surql
DEFINE MODULE mod::phonenumber AS f"bucket:/phonenumber.surli";

RETURN mod::phonenumber::is_valid("6502530000", "US");     -- true
RETURN mod::phonenumber::format_e164("6502530000", "US");  -- "+16502530000"
RETURN mod::phonenumber::parse("6502530000", "US");
-- { valid: true, e164: "+16502530000", country_code: 1, national_number: "6502530000" }
```

| Function | Signature | Description |
|---|---|---|
| `is_valid` | `(number: string, country: string) -> bool` | Whether the number parses to a valid phone number |
| `format_e164` | `(number: string, country: string) -> string` | Parses the number and formats it in E.164 form |
| `parse` | `(number: string, country: string) -> object` | Parses the number into `valid`, `e164`, `country_code` and `national_number` fields |

`country` is a 2-letter ISO 3166-1 alpha-2 region code (e.g. `"US"`, `"GB"`) used as the default region for numbers not already in international (`+...`) format.

`is_valid` returns `false`, not an error, for a number or country code that fails to parse. `format_e164` and `parse` return an error in that case.
