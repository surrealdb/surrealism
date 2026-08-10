# surrealism-anthropic

Anthropic Claude message generation for Surrealism.

```surql
DEFINE MODULE mod::anthropic AS f"bucket:/anthropic.surli";

RETURN mod::anthropic::chat("sk-ant-xxx", "claude-opus-5", [{ role: "user", content: "Hello" }], 1024);
RETURN mod::anthropic::generate("sk-ant-xxx", "claude-opus-5", "Write a haiku about databases", 1024);
```

| Function | Signature | Description |
|---|---|---|
| `chat` | `(api_key: string, model: string, messages: array, max_tokens: int) -> string` | Sends an array of `{ role, content }` messages to a model and returns the generated text |
| `generate` | `(api_key: string, model: string, prompt: string, max_tokens: int) -> string` | Sends a single user prompt to a model and returns the generated text |

Both functions POST to `https://api.anthropic.com/v1/messages` with an
`x-api-key` header carrying the API key and an `anthropic-version:
2023-06-01` header. The Messages API rejects requests without either header,
and rejects requests without `max_tokens` in the body.

The response `content` field is an array of typed blocks. Both functions
return the `text` of the first block whose `type` is `text`, and return an
`Err` when the response contains no text block.

Anthropic does not offer an embeddings API and recommends Voyage AI, so
embeddings live in the sibling [`voyage`](../voyage/) module.

Requires `allow_functions = ["http::post"]` and `api.anthropic.com` in
`allow_net`. Non-2xx responses surface as an `Err`.
