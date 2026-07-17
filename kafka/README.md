# surrealism-kafka

A minimal, plaintext-only Kafka producer for Surrealism — zero dependencies
beyond the Surrealism SDK itself; the wire protocol (request/response framing,
the v2 record batch format, CRC32C, zigzag varints) is hand-rolled directly
against the [Kafka protocol spec](https://kafka.apache.org/protocol.html).

```surql
DEFINE MODULE mod::kafka AS f"bucket:/kafka.surli";

RETURN mod::kafka::produce("10.0.0.5:9092", "events", "user-42", { type: "signup" });
-- returns the offset the broker assigned, e.g. 42
```

| Function | Signature | Description |
|---|---|---|
| `produce` | `(broker: string, topic: string, key: string, value: value) -> int` | Publishes one record to partition 0 of `topic`. Pass `key = ""` for an unkeyed record. Returns the assigned offset. |

`value` accepts any SurrealQL value: a string is sent as its raw UTF-8 bytes,
`bytes` as-is, `NONE`/`NULL` as a Kafka tombstone, and anything else (an
object, array, number, etc.) as its JSON encoding.

## Limitations

- `broker` must be a broker that is itself the leader for partition 0 of
  `topic`, and must be a literal `ip:port` (guest code cannot resolve
  hostnames).
- Partition 0 only — no partitioner.
- No compression, transactions, idempotence, SASL, or TLS — plaintext only.
- No retries.
