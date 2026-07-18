# surrealism-ical

iCalendar (.ics) event generation for Surrealism.

```surql
DEFINE MODULE mod::ical AS f"bucket:/ical.surli";

RETURN mod::ical::event("Standup", "2026-08-01T09:00:00Z", "2026-08-01T09:15:00Z");
RETURN mod::ical::event_with_details("Standup", "2026-08-01T09:00:00Z", "2026-08-01T09:15:00Z", "Daily sync", "Room 1");
```

| Function | Signature | Description |
|---|---|---|
| `event` | `(summary: string, start: string, end: string) -> string` | Builds a single-event .ics calendar, returns the full iCalendar text |
| `event_with_details` | `(summary: string, start: string, end: string, description: string, location: string) -> string` | Same as `event` but also sets a description and location |

`start` and `end` are RFC3339 datetime strings (e.g. `"2026-08-01T09:00:00Z"`).
