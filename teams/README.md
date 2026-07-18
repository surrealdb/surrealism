# surrealism-teams

Microsoft Teams webhook and Graph API integration for Surrealism.

`send` posts a MessageCard-shaped JSON body to a Teams webhook URL. Create
the webhook with the **Workflows** app in Teams (Connectors' classic
"Incoming Webhook" is retired) using the "When a Teams webhook request is
received" trigger, and configure the flow to parse and forward the
MessageCard body.

```surql
DEFINE MODULE mod::teams AS f"bucket:/teams.surli";

RETURN mod::teams::send("https://prod-00.northcentralus.logic.azure.com/workflows/...", "hello from SurrealDB");
RETURN mod::teams::post_channel_message("eyJ0eXAi...", "fbe2bf47-16c8-47cf-b4a5-4b9b187c508b", "19:4a95f7d8db4c4e7fae857bcebe0623e6@thread.tacv2", "hello from SurrealDB");
```

| Function | Signature | Description |
|---|---|---|
| `send` | `(webhook_url: string, text: string) -> string` | Plain-text message via a Teams webhook |
| `post_channel_message` | `(token: string, team_id: string, channel_id: string, text: string) -> object` | Posts a plain-text message to a Teams channel via the Microsoft Graph API |

Requires `allow_functions = ["http::post"]`. A Workflows webhook URL's
hostname is a per-tenant Azure Logic Apps host (e.g.
`prod-00.northcentralus.logic.azure.com`) — `allow_net` only matches exact
hostnames, so add your specific one before deploying `send`. Non-2xx
responses surface as an `Err`.

`post_channel_message` calls `POST
https://graph.microsoft.com/v1.0/teams/{team_id}/channels/{channel_id}/messages`
with `Authorization: Bearer {token}`. `team_id` and `channel_id` can be
obtained from the Graph API or from the Teams UI's "Get link to channel"
option, which embeds the `channel_id` in the URL. Requires
`graph.microsoft.com` in `allow_net`.
