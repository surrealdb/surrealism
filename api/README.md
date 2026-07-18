# api/

Third-party API integrations for Surrealism. Each platform is its own
independent module — separately built, versioned, and capability-scoped —
so using one doesn't require pulling in or granting capabilities for the
others.

| Module | Crate | What it does |
|---|---|---|
| [`discord`](discord/) | `surrealism-discord` | Discord webhook messages and embeds |
| [`slack`](slack/) | `surrealism-slack` | Slack Incoming Webhook messages |
