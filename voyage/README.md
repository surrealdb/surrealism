# surrealism-voyage

Voyage AI text embeddings for Surrealism.

```surql
DEFINE MODULE mod::voyage AS f"bucket:/voyage.surli";

RETURN mod::voyage::embed("pa-xxx", "voyage-4", "Hello, world!");
RETURN mod::voyage::embed_for_query("pa-xxx", "voyage-4", "What is SurrealDB?");
RETURN mod::voyage::embed_for_document("pa-xxx", "voyage-4", "SurrealDB is a multi-model database.");
```

| Function | Signature | Description |
|---|---|---|
| `embed` | `(api_key: string, model: string, input: string) -> array<float>` | Embeds a text string with a Voyage AI model |
| `embed_for_query` | `(api_key: string, model: string, input: string) -> array<float>` | Embeds a text string as the query side of a retrieval pair |
| `embed_for_document` | `(api_key: string, model: string, input: string) -> array<float>` | Embeds a text string as the document side of a retrieval pair |

Every request sends `Authorization: Bearer {api_key}` and posts to
`https://api.voyageai.com/v1/embeddings`. `embed_for_query` and
`embed_for_document` add `"input_type": "query"` and `"input_type": "document"`
to the request body; these produce different embeddings optimised for each side
of a retrieval pair. `embed` sends no `input_type`.

The vector is read from `data[0].embedding` in the response. A response without
that field surfaces as an `Err`.

Requires `allow_functions = ["http::post"]` and `api.voyageai.com` in
`allow_net`. Non-2xx responses surface as an `Err`.
