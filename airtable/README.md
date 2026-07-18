# surrealism-airtable

Airtable integration for Surrealism.

```surql
DEFINE MODULE mod::airtable AS f"bucket:/airtable.surli";

RETURN mod::airtable::create_record("keyXXX", "appXXX", "My Table", { Name: "Ada" });
RETURN mod::airtable::list_records("keyXXX", "appXXX", "My Table");
RETURN mod::airtable::get_record("keyXXX", "appXXX", "My Table", "recXXX");
```

| Function | Signature | Description |
|---|---|---|
| `create_record` | `(api_key: string, base_id: string, table: string, fields: object) -> object` | Creates a record in a table |
| `list_records` | `(api_key: string, base_id: string, table: string) -> object` | Lists records in a table |
| `get_record` | `(api_key: string, base_id: string, table: string, record_id: string) -> object` | Fetches a single record from a table |

Every request sends `Authorization: Bearer {api_key}`. `table` is
percent-encoded before being placed in the URL path, so names containing
spaces or other special characters (e.g. `"Table 1"`) work as-is. `base_id`
and `record_id` use Airtable's own URL-safe ID format and are not encoded.

Requires `allow_functions = ["http::post", "http::get"]` and
`api.airtable.com` in `allow_net`. Non-2xx responses surface as an `Err`.
