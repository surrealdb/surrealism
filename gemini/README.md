# surrealism-gemini

Google Gemini embeddings and text generation for Surrealism.

```surql
DEFINE MODULE mod::gemini AS f"bucket:/gemini.surli";

RETURN mod::gemini::embed("AIzaSyxxx", "gemini-embedding-001", "The quick brown fox");
RETURN mod::gemini::generate("AIzaSyxxx", "gemini-2.5-flash", "Write a haiku about databases");
```

| Function | Signature | Description |
|---|---|---|
| `embed` | `(api_key: string, model: string, text: string) -> array<float>` | Embeds a piece of text with a Gemini embedding model |
| `generate` | `(api_key: string, model: string, prompt: string) -> string` | Generates text from a prompt with a Gemini model |

Every request sends the API key in the `x-goog-api-key` header. Gemini does not
use an `Authorization: Bearer` header.

`model` is placed directly in the URL path, and the action is appended after a
colon: `/v1beta/models/{model}:embedContent` and
`/v1beta/models/{model}:generateContent`.

`embed` returns `embedding.values` from the response. `generate` returns
`candidates[0].content.parts[0].text`. A response missing either surfaces as an
`Err`.

Requires `allow_functions = ["http::post"]` and
`generativelanguage.googleapis.com` in `allow_net`. Non-2xx responses surface
as an `Err`.
