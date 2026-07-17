# surrealism-kafka

A minimal, plaintext-only Kafka producer for Surrealism — zero dependencies
beyond the Surrealism SDK itself; the wire protocol (request/response framing,
the v2 record batch format, CRC32C, zigzag varints) is hand-rolled directly
against the [Kafka protocol spec](https://kafka.apache.org/protocol.html).

```surql
DEFINE MODULE mod::kafka AS f"bucket:/kafka.surli";

RETURN mod::kafka::produce("10.0.0.5:9092", "events", "user-42", '{"type":"signup"}');
-- returns the offset the broker assigned, e.g. 42
```

| Function | Signature | Description |
|---|---|---|
| `produce` | `(broker: string, topic: string, key: string, value: string) -> int` | Publishes one record to partition 0 of `topic`. Pass `key = ""` for an unkeyed record. Returns the assigned offset. |

## What this deliberately does not do

This is intentionally the smallest thing that can talk to a real Kafka
broker, not a general-purpose client:

- **No metadata/leader discovery.** `broker` must be a broker that is itself
  the leader for partition 0 of `topic` — true for a single-broker cluster
  (e.g. local development), not necessarily for a multi-broker one.
- **Partition 0 only.** No partitioner, no explicit partition selection.
- **No compression, transactions, idempotence, SASL, or TLS.** Plaintext
  `PLAINTEXT` listener only.
- **No retries.** A dropped connection or broker error surfaces immediately
  as an `Err`.

Bring your own retry/partitioning logic in SurrealQL around the call, or open
an issue if a specific missing piece would make this useful for your setup.

## Hostnames

`broker` must be a literal `ip:port` — guest code cannot resolve hostnames
(WASI's `ip-name-lookup` is disabled to prevent DNS-tunnelling exfiltration).
If your Kafka deployment is only reachable by hostname, resolve it once
outside the module and pass the IP directly. See the `webhook` module's
README for the same constraint and how it works around it for HTTP targets
by delegating to the host.

## Verification

Verified against a hand-written mock broker (see the module's development
notes) that independently decodes the wire format and checks the CRC32C, not
just against a real Kafka broker — the mock confirmed byte-exact framing,
a matching CRC32C, and correct key/value/offset round-tripping for both
keyed and unkeyed records, plus correct error surfacing when the broker
returns a non-zero error code.
