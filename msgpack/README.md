# surrealism-msgpack

MessagePack binary serialization for Surrealism.

```surql
DEFINE MODULE mod::msgpack AS f"bucket:/msgpack.surli";

LET $packed = mod::msgpack::encode({ name: "Ferris", tags: ["a", "b"], count: 2 });
RETURN mod::msgpack::decode($packed);
```

| Function | Signature | Description |
|---|---|---|
| `encode` | `(value: value) -> bytes` | Serializes a value into MessagePack-encoded bytes |
| `decode` | `(data: bytes) -> value` | Deserializes MessagePack-encoded bytes back into a value |
