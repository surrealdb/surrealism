# surrealism-openai

OpenAI embeddings and chat completions for Surrealism.

```surql
DEFINE MODULE mod::openai AS f"bucket:/openai.surli";

RETURN mod::openai::embed("sk-xxx", "text-embedding-3-small", "The quick brown fox");
RETURN mod::openai::chat("sk-xxx", "gpt-4o-mini", [{ role: "user", content: "Hello" }]);
RETURN mod::openai::generate("sk-xxx", "gpt-4o-mini", "Write a haiku about databases");
```

| Function | Signature | Description |
|---|---|---|
| `embed` | `(api_key: string, model: string, input: string) -> array<float>` | Embeds a piece of text with an OpenAI embedding model |
| `chat` | `(api_key: string, model: string, messages: array) -> string` | Runs a chat completion over an array of `{ role, content }` messages |
| `generate` | `(api_key: string, model: string, prompt: string) -> string` | Runs a one-shot chat completion for a single user prompt |

`embed` posts to `https://api.openai.com/v1/embeddings` and returns
`data[0].embedding`, which feeds straight into
`vector::similarity::cosine`. `chat` and `generate` post to
`https://api.openai.com/v1/chat/completions` and return
`choices[0].message.content`.

Every request sends `Authorization: Bearer {api_key}`.

Requires `allow_functions = ["http::post"]` and `api.openai.com` in
`allow_net`. Non-2xx responses surface as an `Err`.
