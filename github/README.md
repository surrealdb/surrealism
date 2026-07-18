# surrealism-github

GitHub REST API integration for Surrealism.

```surql
DEFINE MODULE mod::github AS f"bucket:/github.surli";

RETURN mod::github::create_issue("ghp_xxx", "surrealdb", "surrealdb", "Bug report", "Something is broken");
RETURN mod::github::create_comment("ghp_xxx", "surrealdb", "surrealdb", 123, "Thanks for the report!");
RETURN mod::github::get_issue("ghp_xxx", "surrealdb", "surrealdb", 123);
```

| Function | Signature | Description |
|---|---|---|
| `create_issue` | `(token: string, owner: string, repo: string, title: string, body: string) -> object` | Creates an issue in a repository |
| `create_comment` | `(token: string, owner: string, repo: string, issue_number: int, body: string) -> object` | Creates a comment on an issue |
| `get_issue` | `(token: string, owner: string, repo: string, issue_number: int) -> object` | Fetches an issue from a repository |

Every request sends `Authorization: Bearer {token}`, `User-Agent:
surrealism-github`, and `Accept: application/vnd.github+json`. GitHub rejects
requests without a `User-Agent` header with a 403.

Requires `allow_functions = ["http::post", "http::get"]` and `api.github.com`
in `allow_net`. Non-2xx responses surface as an `Err`.
