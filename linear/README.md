# surrealism-linear

Linear issue tracking integration for Surrealism.

```surql
DEFINE MODULE mod::linear AS f"bucket:/linear.surli";

RETURN mod::linear::create_issue("lin_api_xxx", "TEAM-ID", "Bug report", "Something is broken");
RETURN mod::linear::get_issue("lin_api_xxx", "ISSUE-ID");
```

| Function | Signature | Description |
|---|---|---|
| `create_issue` | `(api_key: string, team_id: string, title: string, description: string) -> object` | Creates an issue on a team |
| `get_issue` | `(api_key: string, issue_id: string) -> object` | Fetches an issue by id |

Every request POSTs a GraphQL query to `https://api.linear.app/graphql` with
an `Authorization` header set to the raw API key, with no `Bearer` prefix.

Requires `allow_functions = ["http::post"]` and `api.linear.app` in
`allow_net`. Non-2xx responses surface as an `Err`.
