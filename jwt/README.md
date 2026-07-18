# surrealism-jwt

JSON Web Token (HS256) encoding and decoding for Surrealism.

```surql
DEFINE MODULE mod::jwt AS f"bucket:/jwt.surli";

LET $token = mod::jwt::encode({ sub: "user-1", role: "admin" }, "my-secret");
RETURN mod::jwt::decode($token, "my-secret");
```

| Function | Signature | Description |
|---|---|---|
| `encode` | `(claims: object, secret: string) -> string` | Signs `claims` as an HS256 JWT |
| `decode` | `(token: string, secret: string) -> object` | Verifies the signature and returns the claims |
