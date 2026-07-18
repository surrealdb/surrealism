# surrealism-googlesheets

Google Sheets API v4 integration for Surrealism.

```surql
DEFINE MODULE mod::googlesheets AS f"bucket:/googlesheets.surli";

RETURN mod::googlesheets::append_row("ya29.xxx", "1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms", "Sheet1!A1:D1", ["a", "b", "c"]);
RETURN mod::googlesheets::get_values("ya29.xxx", "1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms", "Sheet1!A1:D1");
```

| Function | Signature | Description |
|---|---|---|
| `append_row` | `(access_token: string, spreadsheet_id: string, range: string, values: array<string>) -> object` | Appends a row of values to a sheet |
| `get_values` | `(access_token: string, spreadsheet_id: string, range: string) -> object` | Fetches the values in a range of a sheet |

This module does not perform Google's OAuth2 flow. The caller supplies an
already-valid OAuth2 access token obtained through whatever flow they use
elsewhere. Every request sends `Authorization: Bearer {access_token}`.

`range` is Google's A1 notation (e.g. `Sheet1!A1:D1`) and is percent-encoded
before being placed in the URL path.

Requires `allow_functions = ["http::post", "http::get"]` and
`sheets.googleapis.com` in `allow_net`. Non-2xx responses surface as an `Err`.
