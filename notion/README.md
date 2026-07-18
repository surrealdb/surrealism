# surrealism-notion

Notion API integration for Surrealism.

```surql
DEFINE MODULE mod::notion AS f"bucket:/notion.surli";

RETURN mod::notion::create_page("secret_xxx", "database-id", { Name: { title: [{ text: { content: "Task 1" } }] } });
RETURN mod::notion::get_page("secret_xxx", "page-id");
RETURN mod::notion::query_database("secret_xxx", "database-id");
```

| Function | Signature | Description |
|---|---|---|
| `create_page` | `(token: string, parent_database_id: string, properties: object) -> object` | Creates a page in a database |
| `get_page` | `(token: string, page_id: string) -> object` | Fetches a page by id |
| `query_database` | `(token: string, database_id: string) -> object` | Queries all rows in a database, unfiltered |

`properties` must match the target database's property schema (property
names and types vary per database).

Every request sends `Authorization: Bearer {token}` and `Notion-Version:
2022-06-28`.

Requires `allow_functions = ["http::post", "http::get"]` and
`api.notion.com` in `allow_net`. Non-2xx responses surface as an `Err`.
